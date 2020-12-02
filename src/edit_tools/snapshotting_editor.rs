use super::edit_timeline::EditTimeline;

use crate::voxel::SdfVoxel;

use bevy::ecs::{prelude::*, SystemParam};
use bevy_building_blocks::VoxelEditor;
use building_blocks::prelude::*;

#[derive(SystemParam)]
pub struct SnapshottingVoxelEditor<'a> {
    editor: VoxelEditor<'a, SdfVoxel>,
    timeline: ResMut<'a, EditTimeline>,
}

impl<'a> SnapshottingVoxelEditor<'a> {
    pub fn edit_extent_and_touch_neighbors(
        &mut self,
        extent: Extent3i,
        edit_func: impl FnMut(Point3i, &mut SdfVoxel),
    ) {
        self.timeline
            .add_extent_to_snapshot(extent, &self.editor.map.voxels);
        self.editor
            .edit_extent_and_touch_neighbors(extent, edit_func);
    }

    pub fn finish_edit(&mut self) {
        self.timeline.store_current_snapshot();
    }
}
