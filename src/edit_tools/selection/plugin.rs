use super::{
    controller::{initialize_selection_controller, selection_control_system},
    view::{initialize_selection_view, selection_view_system},
};

use bevy::ecs::prelude::*;

pub struct SelectionPlugin;

impl SelectionPlugin {
    pub fn add_to_enter_stage(stage: &mut SystemStage) {
        stage
            .add_system(initialize_selection_controller.system())
            .add_system(initialize_selection_view.system());
    }

    pub fn add_to_update_stage(stage: &mut SystemStage) {
        stage
            .add_system(selection_control_system.system())
            .add_system(selection_view_system.system());
    }
}
