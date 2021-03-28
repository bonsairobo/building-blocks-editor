use crate::{
    ambient_sdf_array,
    voxel_renderer::{ArrayMaterial, MaterialLayer, MaterialVoxel, SmoothVoxelPbrBundle},
    DirtyChunks, SdfVoxelMap, StatePlugin, ThreadLocalResource, ThreadLocalVoxelCache,
};

use building_blocks::{
    mesh::{surface_nets::*, PosNormMesh},
    prelude::*,
    storage::Local,
};

use bevy::{
    asset::prelude::*,
    ecs,
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        pipeline::PrimitiveTopology,
    },
    tasks::ComputeTaskPool,
};
use std::cell::RefCell;

// TODO: make a collection of textures for different attributes (albedo, normal, metal, rough, emmisive, etc)
#[derive(Default)]
pub struct MeshMaterial(pub Handle<ArrayMaterial>);

/// Generates smooth meshes for voxel chunks. When a chunk becomes dirty, its old mesh is replaced with a newly generated one.
///
/// **NOTE**: Expects the `MeshMaterial` resource to exist before running.
pub struct MeshGeneratorPlugin;

impl MeshGeneratorPlugin {
    pub fn initialize(mut commands: Commands) {
        commands.insert_resource(ChunkMeshes::default());
    }
}

impl StatePlugin for MeshGeneratorPlugin {
    fn add_enter_systems(set: SystemSet) -> SystemSet {
        set.with_system(Self::initialize.system())
    }

    fn add_update_systems(set: SystemSet) -> SystemSet {
        set.with_system(mesh_generator_system.system())
    }
}

#[derive(Default)]
pub struct ChunkMeshes {
    // Map from chunk key to mesh entity.
    entities: SmallKeyHashMap<Point3i, Entity>,
}

/// Generates new meshes for all dirty chunks.
fn mesh_generator_system(
    mut commands: Commands,
    pool: Res<ComputeTaskPool>,
    voxel_map: Res<SdfVoxelMap>,
    dirty_chunks: Res<DirtyChunks>,
    local_caches: Res<ThreadLocalVoxelCache>,
    local_mesh_buffers: ecs::system::Local<ThreadLocalMeshBuffers>,
    mesh_material: Res<MeshMaterial>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_meshes: ResMut<ChunkMeshes>,
) {
    let new_chunk_meshes = generate_mesh_for_each_chunk(
        &*voxel_map,
        &*dirty_chunks,
        &*local_caches,
        &*local_mesh_buffers,
        &*pool,
    );

    for (chunk_key, item) in new_chunk_meshes.into_iter() {
        let old_mesh = if let Some((mesh, material_counts)) = item {
            chunk_meshes.entities.insert(
                chunk_key,
                commands
                    .spawn_bundle(create_voxel_mesh_bundle(
                        mesh,
                        material_counts,
                        mesh_material.0.clone(),
                        &mut *meshes,
                    ))
                    .id(),
            )
        } else {
            chunk_meshes.entities.remove(&chunk_key)
        };
        if let Some(old_mesh) = old_mesh {
            commands.entity(old_mesh).despawn();
        }
    }
}

fn generate_mesh_for_each_chunk(
    voxel_map: &SdfVoxelMap,
    dirty_chunks: &DirtyChunks,
    local_caches: &ThreadLocalVoxelCache,
    local_mesh_buffers: &ThreadLocalMeshBuffers,
    pool: &ComputeTaskPool,
) -> Vec<(Point3i, Option<(PosNormMesh, Vec<[u8; 4]>)>)> {
    pool.scope(|s| {
        for chunk_key in dirty_chunks.dirty_chunk_keys.iter().cloned() {
            s.spawn(async move {
                let cache_tls = local_caches.get();
                let reader = voxel_map.reader(&cache_tls);

                let padded_chunk_extent = padded_surface_nets_chunk_extent(
                    &reader.indexer.extent_for_chunk_at_key(chunk_key),
                );
                let mut padded_chunk = ambient_sdf_array(padded_chunk_extent);
                copy_extent(&padded_chunk_extent, &reader, &mut padded_chunk);

                let sdf_map = TransformMap::new(&padded_chunk, |(_type, dist)| dist);
                let mesh_tls = local_mesh_buffers.get();
                let mut surface_nets_buffer = mesh_tls.get_or_default().borrow_mut();
                surface_nets(
                    &sdf_map,
                    &padded_chunk_extent,
                    1.0,
                    &mut *surface_nets_buffer,
                );

                if surface_nets_buffer.mesh.indices.is_empty() {
                    (chunk_key, None)
                } else {
                    // Count materials adjacent to each vertex for texture blending.
                    let info_map =
                        TransformMap::new(&padded_chunk, voxel_map.voxel_info_transform());
                    let material_counts =
                        count_adjacent_materials(&info_map, &surface_nets_buffer.surface_strides);

                    (
                        chunk_key,
                        Some((surface_nets_buffer.mesh.clone(), material_counts)),
                    )
                }
            })
        }
    })
}

/// Uses a kernel to count the adjacent materials for each surface point. This is necessary because we used dual contouring to
/// construct the mesh, so a given vertex has 8 adjacent voxels, some of which may be empty. This also assumes that the material
/// layer can only be one of 0..4.
fn count_adjacent_materials<A, V>(voxels: &A, surface_strides: &[Stride]) -> Vec<[u8; 4]>
where
    A: IndexedArray<[i32; 3]> + Get<Stride, Item = V>,
    V: IsEmpty + MaterialVoxel,
{
    let mut corner_offsets = [Stride(0); 8];
    voxels.strides_from_local_points(
        &Local::localize_points_array(&Point3i::CUBE_CORNER_OFFSETS),
        &mut corner_offsets,
    );
    let mut material_counts = vec![[0; 4]; surface_strides.len()];
    for (stride, counts) in surface_strides.iter().zip(material_counts.iter_mut()) {
        for corner in corner_offsets.iter() {
            let corner_voxel = voxels.get(*stride + *corner);
            // Only add weights from non-empty voxels.
            if !corner_voxel.is_empty() {
                let material = corner_voxel.material();
                debug_assert!(material != MaterialLayer::NULL);
                counts[material.0 as usize] += 1;
            }
        }
    }

    material_counts
}

// ThreadLocal doesn't let you get a mutable reference, so we need to use RefCell. We lock this down to only be used in this
// module as a Local resource, so we know it's safe.
type ThreadLocalMeshBuffers = ThreadLocalResource<RefCell<SurfaceNetsBuffer>>;

fn create_voxel_mesh_bundle(
    mesh: PosNormMesh,
    material_counts: Vec<[u8; 4]>,
    material: Handle<ArrayMaterial>,
    meshes: &mut Assets<Mesh>,
) -> SmoothVoxelPbrBundle {
    assert_eq!(mesh.positions.len(), mesh.normals.len());
    assert_eq!(mesh.positions.len(), material_counts.len());

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.set_attribute(
        "Vertex_Position",
        VertexAttributeValues::Float3(mesh.positions),
    );
    render_mesh.set_attribute("Vertex_Normal", VertexAttributeValues::Float3(mesh.normals));
    render_mesh.set_attribute(
        "Vertex_MaterialWeights",
        VertexAttributeValues::Uint(
            material_counts
                .into_iter()
                .map(|c| {
                    (c[0] as u32) | (c[1] as u32) << 8 | (c[2] as u32) << 16 | (c[3] as u32) << 24
                })
                .collect(),
        ),
    );
    render_mesh.set_indices(Some(Indices::U32(mesh.indices)));

    SmoothVoxelPbrBundle {
        mesh: meshes.add(render_mesh),
        material,
        ..Default::default()
    }
}