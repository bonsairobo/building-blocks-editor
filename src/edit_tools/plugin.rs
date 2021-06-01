use super::{
    drag_face::{drag_face_tool_system, DragFaceState},
    edit_timeline::EditTimeline,
    selection::SelectionPlugin,
    terraformer::{
        terraformer_default_input_map, terraformer_system, Terraformer, TerraformerEvents,
    },
    tool_switcher::tool_switcher_system,
    undo::undo_system,
    CurrentTool,
};
use crate::StatePlugin;

use bevy::{ecs::prelude::*, prelude::AppBuilder};

/// Depends on the `VoxelPickingPlugin`.
pub struct EditToolsPlugin;

impl EditToolsPlugin {
    fn initialize(mut commands: Commands) {
        commands.insert_resource(EditTimeline::new());
        commands.insert_resource(Terraformer::default());
        commands.insert_resource(CurrentTool::DragFace(DragFaceState::SelectionReady));
    }

    pub fn register_events(app: &mut AppBuilder) {
        app.add_event::<TerraformerEvents>();

    }
}

impl StatePlugin for EditToolsPlugin {
    fn add_enter_systems(set: SystemSet) -> SystemSet {
        SelectionPlugin::add_enter_systems(set).with_system(Self::initialize.system())
    }

    fn add_update_systems(set: SystemSet) -> SystemSet {
        SelectionPlugin::add_update_systems(set)
            .with_system(undo_system.system())
            .with_system(tool_switcher_system.system())
            .with_system(terraformer_system.system())
            .with_system(terraformer_default_input_map.system())
            .with_system(drag_face_tool_system.system())
    }
}