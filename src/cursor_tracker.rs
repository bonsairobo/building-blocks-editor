use bevy::{app::prelude::*, ecs::prelude::*, math::prelude::*, window::prelude::*};

// TODO: bevy
//
// This is just a hack to provide the `CursorPosition` resource, which really should just be a feature of bevy.
pub struct CursorPositionPlugin;

impl Plugin for CursorPositionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(CursorPosition::default())
            .add_system(cursor_tracker_system.system());
    }
}

#[derive(Default)]
pub struct CursorPosition {
    pub current: Vec2,
    pub previous: Vec2,
}

impl CursorPosition {
    pub fn frame_delta(&self) -> Vec2 {
        self.current - self.previous
    }
}

fn cursor_tracker_system(
    mut cursor_reader: EventReader<CursorMoved>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    cursor_position.previous = cursor_position.current;

    if let Some(event) = cursor_reader.iter().next_back() {
        cursor_position.current = event.position;
    }
}
