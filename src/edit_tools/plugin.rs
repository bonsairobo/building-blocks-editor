use super::{
    drag_face::{drag_face_tool_system, DragFaceTool},
    hover_hint::HoverHintPlugin,
    CurrentTool,
};

use bevy::{app::prelude::*, ecs::prelude::*};

/// Depends on the `VoxelPickingPlugin`.
pub struct EditToolsPlugin;

impl Plugin for EditToolsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(CurrentTool::DragFace(DragFaceTool::Start))
            .add_system(drag_face_tool_system.system())
            .add_plugin(HoverHintPlugin);
    }
}
