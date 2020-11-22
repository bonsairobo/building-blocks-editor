use super::hover_hint::HoverHintPlugin;

use bevy::app::prelude::*;

pub struct SelectionToolPlugin;

impl Plugin for SelectionToolPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(HoverHintPlugin);
    }
}
