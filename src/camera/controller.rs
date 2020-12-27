use crate::camera::orbit_transform::{OrbitTransform, PolarVector, Smoother};

use crate::CursorPosition;

use bevy::{
    app::prelude::*,
    ecs::prelude::*,
    input::{mouse::MouseWheel, prelude::*},
    math::prelude::*,
    transform::components::Transform,
};
use serde::{Deserialize, Serialize};

/// A camera controlled with the mouse in the same way as Unreal Engine's viewport controller.
pub struct MouseCameraController {
    control_config: CameraControlConfig,
    transform: OrbitTransform,
    smoother: Smoother,
    enabled: bool,
}

impl MouseCameraController {
    pub fn new(control_config: CameraControlConfig, pivot: Vec3, orbit: Vec3) -> Self {
        Self {
            control_config,
            transform: OrbitTransform { pivot, orbit },
            smoother: Default::default(),
            enabled: true,
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }
}

#[derive(Deserialize, Serialize)]
pub struct CameraControlConfig {
    pub mouse_rotate_sensitivity: f32,
    pub mouse_translate_sensitivity: f32,
    pub trackpad_translate_sensitivity: f32,
    pub smoothing_weight: f32,
}

pub fn mouse_camera_control_system(
    mut mouse_wheel_reader: Local<MouseWheelReader>,
    mouse_wheel: Res<Events<MouseWheel>>,
    mouse_buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    cursor_position: Res<CursorPosition>,
    mut cameras: Query<(&mut MouseCameraController, &mut Transform)>,
) {
    let (mut camera, mut camera_transform) = if let Some((camera, tfm)) = cameras.iter_mut().next()
    {
        (camera, tfm)
    } else {
        return;
    };
    let MouseCameraController {
        control_config,
        transform,
        smoother,
        enabled,
    } = &mut *camera;

    if keys.just_pressed(KeyCode::C) {
        *enabled = !*enabled;
    }

    // We must be pressing the camera button for anything to take effect.
    if *enabled {
        let cursor_delta = cursor_position.frame_delta();

        let look_vector = (transform.orbit - transform.pivot).normalize();
        let mut polar_vector = PolarVector::from_vector(look_vector);
        let forward_vector = Vec3::new(look_vector.x, 0.0, look_vector.z).normalize();

        if mouse_buttons.pressed(MouseButton::Left) && !mouse_buttons.pressed(MouseButton::Right) {
            // Drag translates forward/backward and rotates about the Y axis.
            polar_vector.add_yaw(-control_config.mouse_rotate_sensitivity * cursor_delta.x);
            transform.pivot +=
                control_config.mouse_translate_sensitivity * cursor_delta.y * forward_vector;
        }
        if !mouse_buttons.pressed(MouseButton::Left) && mouse_buttons.pressed(MouseButton::Right) {
            // Drag rotates with pitch and yaw.
            polar_vector.add_yaw(-control_config.mouse_rotate_sensitivity * cursor_delta.x);
            polar_vector.add_pitch(control_config.mouse_rotate_sensitivity * cursor_delta.y);
        }

        polar_vector.assert_not_looking_up();

        let yaw_rot = Quat::from_axis_angle(Vec3::unit_y(), polar_vector.get_yaw());
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

        transform.orbit = transform.pivot + transform.radius() * polar_vector.unit_vector();
    }

    *camera_transform = smoother
        .smooth_transform(control_config.smoothing_weight, transform)
        .pivot_look_at_orbit_transform();
}

#[derive(Default)]
pub struct MouseWheelReader {
    reader: EventReader<MouseWheel>,
}
