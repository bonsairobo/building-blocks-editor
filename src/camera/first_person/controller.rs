use super::FirstPersonCamera;

use crate::CursorPosition;

use bevy::{
    app::prelude::*,
    ecs::prelude::*,
    input::{mouse::MouseWheel, prelude::*},
    math::prelude::*,
    transform::components::Transform,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ControlConfig {
    pub mouse_rotate_sensitivity: f32,
    pub mouse_translate_sensitivity: f32,
    pub trackpad_translate_sensitivity: f32,
    pub smoothing_weight: f32,
}

pub fn first_person_camera_control_system(
    mut mouse_wheel_reader: Local<MouseWheelReader>,
    mouse_wheel: Res<Events<MouseWheel>>,
    mouse_buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    cursor_position: Res<CursorPosition>,
    mut cameras: Query<(&mut FirstPersonCamera, &mut Transform)>,
) {
    let (mut camera, mut camera_transform) = if let Some((camera, tfm)) = cameras.iter_mut().next()
    {
        (camera, tfm)
    } else {
        return;
    };
    let FirstPersonCamera {
        control_config,
        transform,
        smoother,
        prev_cursor_position,
    } = &mut *camera;

    // We must be pressing the camera button for anything to take effect.
    if keys.pressed(KeyCode::C) {
        let cursor_delta = cursor_position.0 - *prev_cursor_position;

        if mouse_buttons.pressed(MouseButton::Left) && !mouse_buttons.pressed(MouseButton::Right) {
            // Drag translates forward/backward and rotates about the Y axis.
            transform.add_yaw(-control_config.mouse_rotate_sensitivity * cursor_delta.x);
            let forward_vec = transform.unit_vector();
            let forward_vec = Vec3::new(forward_vec.x, 0.0, forward_vec.z).normalize();
            transform.pivot +=
                control_config.mouse_translate_sensitivity * cursor_delta.y * forward_vec;
        }
        if !mouse_buttons.pressed(MouseButton::Left) && mouse_buttons.pressed(MouseButton::Right) {
            // Drag rotates with pitch and yaw.
            transform.add_yaw(-control_config.mouse_rotate_sensitivity * cursor_delta.x);
            transform.add_pitch(control_config.mouse_rotate_sensitivity * cursor_delta.y);
        }

        let yaw_rot = Quat::from_axis_angle(Vec3::unit_y(), transform.get_yaw());
        let rot_x = yaw_rot * Vec3::unit_x();
        let rot_y = yaw_rot * Vec3::unit_y();
        if mouse_buttons.pressed(MouseButton::Left) && mouse_buttons.pressed(MouseButton::Right) {
            // Drag translates up/down (Y) and left/right (X).
            transform.pivot += control_config.mouse_translate_sensitivity
                * (-cursor_delta.x * rot_x + cursor_delta.y * rot_y);
        }
        // On Mac, mouse wheel is the trackpad, treated the same as both mouse buttons down.
        let mut trackpad_delta = Vec2::zero();
        for event in mouse_wheel_reader.reader.iter(&*mouse_wheel) {
            trackpad_delta.x += event.x;
            trackpad_delta.y += event.y;
        }
        transform.pivot += control_config.trackpad_translate_sensitivity
            * (-trackpad_delta.x * rot_x + trackpad_delta.y * rot_y);
    }

    *camera_transform = smoother
        .smooth_transform(control_config.smoothing_weight, transform)
        .pivot_look_at_orbit_transform();

    *prev_cursor_position = cursor_position.0;
}

#[derive(Default)]
pub struct MouseWheelReader {
    reader: EventReader<MouseWheel>,
}
