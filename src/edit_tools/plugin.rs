use super::{
    drag_face::{drag_face_tool_system, DragFaceState},
    edit_timeline::EditTimeline,
    selection::SelectionPlugin,
    undo::undo_system,
    CurrentTool,
};

use bevy::ecs::prelude::*;

/// Depends on the `VoxelPickingPlugin`.
pub struct EditToolsPlugin;

impl EditToolsPlugin {
    fn initialize(commands: &mut Commands) {
        commands
            .insert_resource(EditTimeline::new())
            .insert_resource(CurrentTool::DragFace(DragFaceState::SelectionReady));
    }

    pub fn add_to_enter_stage(stage: &mut SystemStage) {
        SelectionPlugin::add_to_enter_stage(stage);
        stage.add_system(Self::initialize);
    }

    pub fn add_to_update_stage(stage: &mut SystemStage) {
        SelectionPlugin::add_to_update_stage(stage);
        stage
            .add_system(undo_system.system())
            .add_system(drag_face_tool_system.system());
    }
}
