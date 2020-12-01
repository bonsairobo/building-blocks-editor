use super::edit_timeline::EditTimeline;

use crate::voxel::SdfVoxel;

use bevy::{ecs::prelude::*, input::prelude::*};
use bevy_building_blocks::VoxelEditor;

pub fn undo_system(
    mut edit_timeline: ResMut<EditTimeline>,
    mut editor: VoxelEditor<SdfVoxel>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::U) {
        edit_timeline.undo(&mut editor);
    }
}
