use super::{
    chunk_cache_flusher::chunk_cache_flusher_system,
    chunk_compressor::chunk_compressor_system,
    edit_buffer::{double_buffering_system, DirtyChunks},
    empty_chunk_remover::empty_chunk_remover_system,
    EditBuffer, EmptyChunks, ThreadLocalVoxelCache,
};

use bevy::{app::prelude::*, ecs::prelude::*};
use building_blocks::core::Point3i;

pub use super::chunk_compressor::ChunkCacheConfig;

/// A bevy plugin that provides dynamic read caching and compression for the `SdfVoxelMap` resource.
///
/// This plugin expects the `SdfVoxelMap` resource to already exist before systems are dispatched.
///
/// This plugin uses thread-local caches for voxel chunks that are decompressed during access. At the end of the frame, these
/// caches are flushed back into the `SdfVoxelMap`'s global cache.
///
/// If the size of the global chunk cache grows beyond a limit, one of the plugin systems will start compressing the
/// least-recently-used chunks to save space.
///
/// In order to get maximum read parallelism from the voxel map, use the `VoxelEditor`, a `SystemParam` that writes your edits
/// out of place. The edits will get merged into the `SdfVoxelMap` at the end of the same frame. The edited chunks will also be
/// marked as "dirty" in the `DirtyChunks` resource, which makes it easier to do post-processing when chunks change.
///
/// **WARNING**: Cached reads will always be flushed before double-buffered writes. This means if you try to write directly into
/// the `SdfVoxelMap`, you risk having your changes overwritten by the flush.
pub struct MapIoPlugin {
    pub chunk_shape: Point3i,
    pub cache_config: ChunkCacheConfig,
}

impl MapIoPlugin {
    pub fn new(chunk_shape: Point3i, cache_config: ChunkCacheConfig) -> Self {
        Self {
            chunk_shape,
            cache_config,
        }
    }
}

impl Plugin for MapIoPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(self.cache_config)
            .insert_resource(EditBuffer::new(self.chunk_shape))
            .insert_resource(DirtyChunks::default())
            .insert_resource(EmptyChunks::default())
            // Each thread gets its own local chunk cache. The local caches are flushed into the global cache in the
            // chunk_cache_flusher_system.
            .insert_resource(ThreadLocalVoxelCache::new())
            // Ordering the cache flusher and double buffering is important, because we don't want to overwrite edits with
            // locally cached chunks. Similarly, empty chunks should be removed before new edits are merged in.
            .add_system_to_stage(CoreStage::Last, chunk_cache_flusher_system.system())
            .add_system_to_stage(CoreStage::Last, empty_chunk_remover_system.system())
            .add_system_to_stage(CoreStage::Last, double_buffering_system.system())
            .add_system_to_stage(CoreStage::Last, chunk_compressor_system.system());
    }
}
