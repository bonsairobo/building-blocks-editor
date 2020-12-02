use bevy::{app::prelude::*, ecs::prelude::*, math::prelude::*, window::prelude::*};

// TODO: bevy
//
// This is just a hack to provide the `CursorPosition` resource, which really should just be a
// feature of bevy.
pub struct CursorPositionPlugin;

impl Plugin for CursorPositionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(CursorPosition::default())
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
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    cursor_position.previous = cursor_position.current;

    let mut cursor_reader = EventReader::default();
    if let Some(event) = cursor_reader.latest(&cursor_moved_events) {
        cursor_position.current = event.position;
    }
}
