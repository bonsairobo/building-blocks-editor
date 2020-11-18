use crate::mesh::create_pos_norm_mesh_entity;

use bevy_building_blocks::{
    DirtyChunks, ThreadLocalResource, ThreadLocalVoxelCache, Voxel, VoxelMap,
};

use bevy::{ecs, prelude::*, tasks::ComputeTaskPool};
use building_blocks::mesh::*;
use building_blocks::prelude::*;
use fnv::FnvHashMap;
use std::cell::UnsafeCell;

/// Generates smooth meshes for voxel chunks. When a chunk becomes dirty, its old mesh is replaced
/// with a newly generated one.
pub struct SmoothMeshPlugin<V: Voxel> {
    marker: std::marker::PhantomData<V>,
}

impl<V: Voxel> SmoothMeshPlugin<V> {
    pub fn new() -> Self {
        Self {
            marker: Default::default(),
        }
    }
}

impl<V: Voxel + SignedDistance> Plugin for SmoothMeshPlugin<V> {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(initialize_mesh_generator::<V>.system())
            .add_resource(ChunkMeshes::default())
            .add_system(mesh_generator_system::<V>.system());
    }
}

fn initialize_mesh_generator<V>(
    commands: &mut Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = MeshMaterial(materials.add(StandardMaterial::from(Color::RED)));
    commands.insert_resource(material);
}

#[derive(Default)]
pub struct ChunkMeshes {
    // Map from chunk key to mesh entity.
    entities: FnvHashMap<Point3i, Entity>,
}

/// Generates new meshes for all dirty chunks.
fn mesh_generator_system<V: Voxel + SignedDistance>(
    commands: &mut Commands,
    pool: Res<ComputeTaskPool>,
    voxel_map: Res<VoxelMap<V>>,
    dirty_chunks: Res<DirtyChunks>,
    local_caches: Res<ThreadLocalVoxelCache<V>>,
    // local_mesh_buffers: ecs::Local<ThreadLocalMeshBuffers>,
    mesh_material: Res<MeshMaterial>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_meshes: ResMut<ChunkMeshes>,
) {
    println!("mesh START");
    let new_chunk_meshes = generate_mesh_for_each_chunk(
        &*voxel_map,
        &*dirty_chunks,
        &*local_caches,
        // &*local_mesh_buffers,
        &*pool,
    );
    println!("smooth_mesh: MIDDLE");

    for (chunk_key, mesh) in new_chunk_meshes.into_iter() {
        let old_mesh = if let Some(mesh) = mesh {
            chunk_meshes.entities.insert(
                chunk_key,
                create_pos_norm_mesh_entity(mesh, mesh_material.0.clone(), commands, &mut *meshes),
            )
        } else {
            chunk_meshes.entities.remove(&chunk_key)
        };
        if let Some(old_mesh) = old_mesh {
            commands.despawn(old_mesh);
        }
    }
    println!("smooth_mesh: DONE");
}

fn generate_mesh_for_each_chunk<V: Voxel + SignedDistance>(
    voxel_map: &VoxelMap<V>,
    dirty_chunks: &DirtyChunks,
    local_caches: &ThreadLocalVoxelCache<V>,
    // local_mesh_buffers: &ThreadLocalMeshBuffers,
    pool: &ComputeTaskPool,
) -> Vec<(Point3i, Option<PosNormMesh>)> {
    println!("smooth_mesh: SCOPE");
    pool.scope(|s| {
        for chunk_key in dirty_chunks.dirty_chunk_keys.iter().cloned() {
            println!("smooth_mesh: SPAWN");
            s.spawn(async move {
                println!("smooth_mesh: Getting cache TLS");
                let cache_tls = local_caches.get();
                println!("smooth_mesh: Got cache TLS");
                let reader = voxel_map.reader(&cache_tls);

                let padded_chunk_extent =
                    padded_surface_nets_chunk_extent(&reader.extent_for_chunk_at_key(&chunk_key));
                let mut padded_chunk = Array3::fill(padded_chunk_extent, V::default());
                copy_extent(&padded_chunk_extent, &reader, &mut padded_chunk);

                // let mesh_tls = local_mesh_buffers.get();
                // let surface_nets_buffer = unsafe { &mut *mesh_tls.get_or_default().get() };
                let mut surface_nets_buffer = SurfaceNetsBuffer::default();
                surface_nets(
                    &padded_chunk,
                    &padded_chunk_extent,
                    &mut surface_nets_buffer,
                );

                if surface_nets_buffer.mesh.indices.is_empty() {
                    (chunk_key, None)
                } else {
                    (chunk_key, Some(surface_nets_buffer.mesh.clone()))
                }
            })
        }
    })
}

// ThreadLocal doesn't let you get a mutable reference, so we need to use UnsafeCell. We lock this
// down to only be used in this module as a Local resource, so we know it's safe.
// type ThreadLocalMeshBuffers = ThreadLocalResource<UnsafeCell<SurfaceNetsBuffer>>;

#[derive(Default)]
pub struct MeshMaterial(pub Handle<StandardMaterial>);
