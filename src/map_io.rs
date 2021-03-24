mod chunk_cache_flusher;
mod chunk_compressor;
mod edit_buffer;
mod editor;
mod empty_chunk_remover;
mod plugin;

pub use chunk_compressor::ChunkCacheConfig;
pub use edit_buffer::{double_buffering_system, DirtyChunks, EditBuffer};
pub use editor::VoxelEditor;
pub use empty_chunk_remover::EmptyChunks;
pub use plugin::MapIoPlugin;

use crate::{SdfChunkCache, ThreadLocalResource};

pub type ThreadLocalVoxelCache = ThreadLocalResource<SdfChunkCache>;
