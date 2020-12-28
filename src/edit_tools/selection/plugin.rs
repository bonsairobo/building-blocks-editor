use super::{
    controller::{initialize_selection_controller, selection_control_system},
    view::{initialize_selection_view, selection_view_system},
};

use bevy::ecs::prelude::*;

pub struct SelectionPlugin;

impl SelectionPlugin {
    pub fn initialize_in_stage(stage: &mut SystemStage) {
        stage
            .add_system(initialize_selection_controller)
            .add_system(initialize_selection_view);
    }

    pub fn update_in_stage(stage: &mut SystemStage) {
        stage
            .add_system(selection_control_system)
            .add_system(selection_view_system);
    }
}
