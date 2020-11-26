use super::{
    controller::selection_control_system,
    view::{initialize_selection_view, selection_view_system},
    SelectionState,
};

use bevy::{app::prelude::*, ecs::prelude::*};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(SelectionState::SelectingFirstCorner)
            .add_startup_system(initialize_selection_view.system())
            .add_system(selection_control_system.system())
            .add_system(selection_view_system.system());
    }
}
