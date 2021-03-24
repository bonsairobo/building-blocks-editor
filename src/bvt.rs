use crate::{DirtyChunks, EmptyChunks, SdfVoxelMap, StatePlugin, ThreadLocalVoxelCache};

use building_blocks::{prelude::*, search::OctreeDBVT, storage::OctreeSet};

use bevy::{
    prelude::*,
    tasks::{ComputeTaskPool, TaskPool},
};

/// Manages the `VoxelBVT` resource by generating `OctreeSet`s for any edited chunks. Depends on the `MapIoPlugin`.
#[derive(Default)]
pub struct BVTPlugin;

impl BVTPlugin {
    pub fn initialize(mut commands: Commands) {
        commands.insert_resource(VoxelBVT::default());
    }
}

impl StatePlugin for BVTPlugin {
    fn add_enter_systems(set: SystemSet) -> SystemSet {
        set.with_system(Self::initialize.system())
    }

    fn add_update_systems(set: SystemSet) -> SystemSet {
        set.with_system(octree_generator_system.system())
    }
}

/// An `OctreeDBVT` that maps chunk keys to chunk `OctreeSet`s.
pub type VoxelBVT = OctreeDBVT<Point3i>;

/// Generates new octrees for all edited chunks.
fn octree_generator_system(
    pool: Res<ComputeTaskPool>,
    voxel_map: Res<SdfVoxelMap>,
    local_caches: Res<ThreadLocalVoxelCache>,
    dirty_chunks: Res<DirtyChunks>,
    mut voxel_bvt: ResMut<VoxelBVT>,
    mut empty_chunks: ResMut<EmptyChunks>,
) {
    let new_chunk_octrees =
        generate_octree_for_each_chunk(&*dirty_chunks, &*voxel_map, &*local_caches, &*pool);

    for (chunk_key, octree) in new_chunk_octrees.into_iter() {
        if octree.is_empty() {
            voxel_bvt.remove(&chunk_key);
            empty_chunks.mark_for_removal(chunk_key);
        } else {
            voxel_bvt.insert(chunk_key, octree);
        }
    }
}

fn generate_octree_for_each_chunk(
    dirty_chunks: &DirtyChunks,
    map: &SdfVoxelMap,
    local_caches: &ThreadLocalVoxelCache,
    pool: &TaskPool,
) -> Vec<(Point3i, OctreeSet)> {
    pool.scope(|s| {
        for chunk_key in dirty_chunks.edited_chunk_keys.clone().into_iter() {
            s.spawn(async move {
                let cache_tls = local_caches.get();
                let reader = map.reader(&cache_tls);
                let chunk = reader.get_chunk(chunk_key).unwrap();
                let transform_chunk = TransformMap::new(chunk, map.voxel_info_transform());

                (
                    chunk_key,
                    OctreeSet::from_array3(&transform_chunk, *chunk.extent()),
                )
            })
        }
    })
}
