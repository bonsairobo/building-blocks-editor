use super::{
    controller::{initialize_selection_controller, selection_control_system},
    view::{initialize_selection_view, selection_view_system},
};
use crate::StatePlugin;

use bevy::ecs::prelude::*;

pub struct SelectionPlugin;

impl StatePlugin for SelectionPlugin {
    fn add_enter_systems(set: SystemSet) -> SystemSet {
        set.with_system(initialize_selection_controller.system())
            .with_system(initialize_selection_view.system())
    }

    fn add_update_systems(set: SystemSet) -> SystemSet {
        set.with_system(selection_control_system.system())
            .with_system(selection_view_system.system())
    }
}
