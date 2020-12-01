use super::{
    drag_face::{drag_face_tool_system, DragFaceState},
    edit_timeline::EditTimeline,
    selection::SelectionPlugin,
    undo::undo_system,
    CurrentTool,
};

use bevy::{app::prelude::*, ecs::prelude::*};

/// Depends on the `VoxelPickingPlugin`.
pub struct EditToolsPlugin;

impl Plugin for EditToolsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(SelectionPlugin)
            .add_resource(EditTimeline::new())
            .add_resource(CurrentTool::DragFace(DragFaceState::SelectionReady))
            .add_system(undo_system.system())
            .add_system(drag_face_tool_system.system());
    }
}
