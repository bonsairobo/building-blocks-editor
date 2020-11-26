use super::{
    drag_face::{drag_face_tool_system, DragFaceState},
    selection::SelectionPlugin,
    CurrentTool,
};

use bevy::{app::prelude::*, ecs::prelude::*};

/// Depends on the `VoxelPickingPlugin`.
pub struct EditToolsPlugin;

impl Plugin for EditToolsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(SelectionPlugin)
            .add_resource(CurrentTool::DragFace(DragFaceState::SelectionReady))
            .add_system(drag_face_tool_system.system());
    }
}
