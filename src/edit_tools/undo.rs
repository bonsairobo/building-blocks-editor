use super::edit_timeline::EditTimeline;
use crate::VoxelEditor;

use bevy::{ecs::prelude::*, input::prelude::*};

pub fn undo_system(
    mut edit_timeline: ResMut<EditTimeline>,
    mut editor: VoxelEditor,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::U) {
        edit_timeline.undo(&mut editor);
    }
    if keys.just_pressed(KeyCode::R) {
        edit_timeline.redo(&mut editor);
    }
}
