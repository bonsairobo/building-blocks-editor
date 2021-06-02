use bevy::{
    app::prelude::*,
    ecs::prelude::*,
    input::{mouse::MouseMotion, prelude::*},
    math::prelude::*,
};

use bevy_orbit_controls::*;

pub fn orbit_camera_default_input_map(
    mut events: EventWriter<CameraEvents>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&OrbitCamera>,
) {
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }
    for camera in query.iter_mut() {
        if keyboard.pressed(KeyCode::LControl) {
            events.send(CameraEvents::Orbit(delta));
        }

        if mouse_button_input.pressed(camera.pan_button) {
            events.send(CameraEvents::Pan(delta));
        }
    }
}
