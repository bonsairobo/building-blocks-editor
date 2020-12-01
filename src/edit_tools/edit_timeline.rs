use crate::{voxel::SdfVoxel, VOXEL_CHUNK_SHAPE};

use bevy::ecs::{prelude::*, SystemParam};
use bevy_building_blocks::{default_array, default_chunk_map, VoxelEditor};
use building_blocks::{prelude::*, storage::compressible_map::MaybeCompressed};
use std::collections::VecDeque;

#[derive(SystemParam)]
pub struct SnapshottingVoxelEditor<'a> {
    editor: VoxelEditor<'a, SdfVoxel>,
    timeline: ResMut<'a, EditTimeline>,
}

impl<'a> SnapshottingVoxelEditor<'a> {
    pub fn edit_extent(&mut self, extent: Extent3i, edit_func: impl FnMut(Point3i, &mut SdfVoxel)) {
        self.add_extent_to_snapshot(extent);
        self.editor.edit_extent(extent, edit_func);
    }

    pub fn edit_extent_and_touch_neighbors(
        &mut self,
        extent: Extent3i,
        edit_func: impl FnMut(Point3i, &mut SdfVoxel),
    ) {
        self.add_extent_to_snapshot(extent);
        self.editor
            .edit_extent_and_touch_neighbors(extent, edit_func);
    }

    fn add_extent_to_snapshot(&mut self, extent: Extent3i) {
        let map_voxels = &self.editor.map.voxels;
        let snapshot_voxels = &mut self.timeline.current_snapshot.voxels;

        // TODO: compress all snapshotted chunks
        for chunk_key in map_voxels.chunk_keys_for_extent(&extent) {
            snapshot_voxels.chunks.get_or_insert_with(chunk_key, || {
                map_voxels
                    // This chunk will eventually get cached after being written by the editor.
                    .copy_chunk_without_caching(&chunk_key)
                    .unwrap_or(Chunk3::with_array(default_array(
                        map_voxels.extent_for_chunk_at_key(&chunk_key),
                    )))
            });
        }
    }

    pub fn finish_edit(&mut self) {
        self.timeline.store_current_snapshot();
    }
}

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
                voxels: default_chunk_map(VOXEL_CHUNK_SHAPE),
            },
        }
    }

    pub fn store_current_snapshot(&mut self) {
        let chunk_shape = *self.current_snapshot.voxels.chunk_shape();
        let finalized_snapshot =
            std::mem::replace(&mut self.current_snapshot, Snapshot::new(chunk_shape));
        self.undo_queue.push_back(finalized_snapshot);

        // We don't want to keep "undone edits" before this new one.
        self.redo_queue.clear();
    }

    pub fn undo(&mut self, editor: &mut VoxelEditor<SdfVoxel>) {
        reversible_do(&mut self.undo_queue, &mut self.redo_queue, editor)
    }

    pub fn redo(&mut self, editor: &mut VoxelEditor<SdfVoxel>) {
        reversible_do(&mut self.redo_queue, &mut self.undo_queue, editor)
    }
}

fn reversible_do(
    do_queue: &mut VecDeque<Snapshot>,
    undo_queue: &mut VecDeque<Snapshot>,
    editor: &mut VoxelEditor<SdfVoxel>,
) {
    if let Some(snapshot) = do_queue.pop_back() {
        let mut redo_snap_chunks = default_chunk_map(*snapshot.voxels.chunk_shape());
        for (chunk_key, chunk) in snapshot.voxels.chunks.into_iter() {
            match chunk {
                MaybeCompressed::Compressed(c) => {
                    editor.insert_chunk_and_touch_neighbors(chunk_key, c.decompress().array);
                }
                MaybeCompressed::Decompressed(c) => {
                    // TODO: assert only compressed
                    editor.insert_chunk_and_touch_neighbors(chunk_key, c.array);
                }
            }

            if let Some(chunk) = editor.map.voxels.copy_chunk_without_caching(&chunk_key) {
                redo_snap_chunks.insert_chunk(chunk_key, chunk);
            }
        }
        undo_queue.push_back(Snapshot {
            voxels: redo_snap_chunks,
        });
    }
}

struct Snapshot {
    voxels: ChunkMap3<SdfVoxel>,
}

impl Snapshot {
    fn new(chunk_shape: Point3i) -> Self {
        Self {
            voxels: default_chunk_map(chunk_shape),
        }
    }
}
