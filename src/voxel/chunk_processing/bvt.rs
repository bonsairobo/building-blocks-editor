use bevy_building_blocks::{DirtyChunks, ThreadLocalVoxelCache, Voxel, VoxelMap};

use building_blocks::{
    partition::{Octree, OctreeDBVT},
    prelude::*,
};

use bevy::{
    prelude::*,
    tasks::{ComputeTaskPool, TaskPool},
};

#[derive(Default)]
pub struct BVTPlugin<V> {
    marker: std::marker::PhantomData<V>,
}

impl<V> Plugin for BVTPlugin<V>
where
    V: Voxel,
    for<'r> &'r V::TypeInfo: IsEmpty,
{
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(VoxelBVT::default())
            .add_system(octree_generator_system::<V>.system());
    }
}

pub type VoxelBVT = OctreeDBVT<Point3i>;

/// Generates new octrees for all edited chunks.
fn octree_generator_system<V>(
    pool: Res<ComputeTaskPool>,
    voxel_map: Res<VoxelMap<V>>,
    local_caches: Res<ThreadLocalVoxelCache<V>>,
    dirty_chunks: Res<DirtyChunks>,
    mut voxel_bvt: ResMut<VoxelBVT>,
) where
    V: Voxel,
    for<'r> &'r V::TypeInfo: IsEmpty,
{
    println!("bvt START");
    let new_chunk_octrees =
        generate_octree_for_each_chunk(&*dirty_chunks, &*voxel_map, &*local_caches, &*pool);
    println!("bvt MIDDLE");

    for (chunk_key, octree) in new_chunk_octrees.into_iter() {
        if octree.is_empty() {
            voxel_bvt.remove(&chunk_key);
        } else {
            voxel_bvt.insert(chunk_key, octree);
        }
    }
    println!("bvt STOP");
}

fn generate_octree_for_each_chunk<V>(
    dirty_chunks: &DirtyChunks,
    map: &VoxelMap<V>,
    local_caches: &ThreadLocalVoxelCache<V>,
    pool: &TaskPool,
) -> Vec<(Point3i, Octree)>
where
    V: Voxel,
    for<'r> &'r V::TypeInfo: IsEmpty,
{
    println!("bvt SCOPE");
    pool.scope(|s| {
        for chunk_key in dirty_chunks.edited_chunk_keys.clone().into_iter() {
            println!("bvt SPAWN");
            s.spawn(async move {
                println!("bvt: Getting cache TLS");
                let cache_tls = local_caches.get();
                println!("bvt: Got cache TLS");
                let reader = map.reader(&cache_tls);
                let chunk = reader.get_chunk(chunk_key).unwrap();
                let transform_chunk = TransformMap::new(&chunk.array, map.voxel_info_transform());

                (
                    chunk_key,
                    Octree::from_array3(&transform_chunk, *chunk.array.extent()),
                )
            })
        }
    })
}
