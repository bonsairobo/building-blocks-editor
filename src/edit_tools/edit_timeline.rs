use crate::{
    ambient_sdf_array, empty_sdf_chunk_hash_map, CompressibleSdfChunkMap, SdfChunkHashMap,
    VoxelEditor, CHUNK_SHAPE,
};

use building_blocks::prelude::*;
use std::collections::VecDeque;

// TODO: limit the memory usage of the timeline somehow

pub struct EditTimeline {
    undo_queue: VecDeque<Snapshot>,
    redo_queue: VecDeque<Snapshot>,
    current_snapshot: Snapshot,
}

impl EditTimeline {
    pub fn new() -> Self {
        Self {
            undo_queue: Default::default(),
            redo_queue: Default::default(),
            current_snapshot: Snapshot {
                voxels: empty_sdf_chunk_hash_map(CHUNK_SHAPE),
            },
        }
    }

    pub fn store_current_snapshot(&mut self) {
        let chunk_shape = self.current_snapshot.voxels.indexer.chunk_shape();
        let finalized_snapshot =
            std::mem::replace(&mut self.current_snapshot, Snapshot::new(chunk_shape));
        self.undo_queue.push_back(finalized_snapshot);

        // We don't want to keep "undone edits" before this new one.
        self.redo_queue.clear();
    }

    pub fn undo(&mut self, editor: &mut VoxelEditor) {
        reversible_restore_snapshot(&mut self.undo_queue, &mut self.redo_queue, editor)
    }

    pub fn redo(&mut self, editor: &mut VoxelEditor) {
        reversible_restore_snapshot(&mut self.redo_queue, &mut self.undo_queue, editor)
    }

    pub fn add_extent_to_snapshot(&mut self, extent: Extent3i, src_map: &CompressibleSdfChunkMap) {
        for chunk_min in src_map.indexer.chunk_mins_for_extent(&extent) {
            let chunk_key = ChunkKey::new(0, chunk_min);
            self.current_snapshot
                .voxels
                .get_mut_chunk_or_insert_with(chunk_key, || {
                    src_map
                        .storage()
                        // This chunk will eventually get cached after being written by the editor.
                        .copy_without_caching(chunk_key)
                        .map(|c| c.into_decompressed())
                        .unwrap_or(ambient_sdf_array(
                            src_map.indexer.extent_for_chunk_with_min(chunk_min),
                        ))
                });
        }
    }
}

fn reversible_restore_snapshot(
    do_queue: &mut VecDeque<Snapshot>,
    undo_queue: &mut VecDeque<Snapshot>,
    editor: &mut VoxelEditor,
) {
    if let Some(snapshot) = do_queue.pop_back() {
        let indexer = snapshot.voxels.indexer;
        let storage = snapshot.voxels.take_storage();

        let mut redo_snap_chunks = empty_sdf_chunk_hash_map(indexer.chunk_shape());
        for (chunk_key, chunk) in storage.into_iter() {
            editor.insert_chunk_and_touch_neighbors(chunk_key.minimum, chunk);
            let old_chunk = editor
                .map
                .voxels
                .storage()
                .copy_without_caching(chunk_key)
                .map(|c| c.into_decompressed())
                .unwrap_or(ambient_sdf_array(
                    indexer.extent_for_chunk_with_min(chunk_key.minimum),
                ));
            redo_snap_chunks.write_chunk(chunk_key, old_chunk);
        }
        undo_queue.push_back(Snapshot {
            voxels: redo_snap_chunks,
        });
    }
}

struct Snapshot {
    voxels: SdfChunkHashMap,
}

impl Snapshot {
    fn new(chunk_shape: Point3i) -> Self {
        Self {
            voxels: empty_sdf_chunk_hash_map(chunk_shape),
        }
    }
}
