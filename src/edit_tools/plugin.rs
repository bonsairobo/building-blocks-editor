use super::{
    drag_face::{drag_face_tool_system, DragFaceState},
    edit_timeline::EditTimeline,
    selection::SelectionPlugin,
    terraformer::{terraformer_system, Terraformer},
    tool_switcher::tool_switcher_system,
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
            .insert_resource(Terraformer::default())
            .insert_resource(CurrentTool::DragFace(DragFaceState::SelectionReady));
    }

    pub fn initialize_in_stage(stage: &mut SystemStage) {
        SelectionPlugin::initialize_in_stage(stage);
        stage.add_system(Self::initialize);
    }

    pub fn update_in_stage(stage: &mut SystemStage) {
        SelectionPlugin::update_in_stage(stage);
        stage
            .add_system(undo_system)
            .add_system(tool_switcher_system)
            .add_system(terraformer_system)
            .add_system(drag_face_tool_system);
    }
}
