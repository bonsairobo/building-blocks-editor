use crate::camera::orbit_transform::{OrbitTransform, PolarVector, Smoother};

use bevy::{
    app::prelude::*,
    ecs::prelude::*,
    input::{mouse::MouseMotion, prelude::*},
    math::prelude::*,
    transform::components::Transform,
};
use serde::Deserialize;

pub enum CameraEvent {
    Orbit(Vec2),
    Pan(Vec2),
}

pub fn orbit_camera_default_input_map(
    mut events: EventWriter<CameraEvent>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }

    if mouse_button_input.pressed(MouseButton::Right) {
        if keyboard.pressed(KeyCode::LControl) {
            events.send(CameraEvent::Orbit(delta));
        } else {
            events.send(CameraEvent::Pan(delta));
        }
    }
}

pub struct OrbitCameraController {
    control_config: OrbitCameraControlConfig,
    transform: OrbitTransform,
    smoother: Smoother,
    enabled: bool,
}

impl OrbitCameraController {
    pub fn new(control_config: OrbitCameraControlConfig, pivot: Vec3, orbit: Vec3) -> Self {
        Self {
            control_config,
            transform: OrbitTransform { pivot, orbit },
            smoother: Default::default(),
            enabled: true,
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
pub struct OrbitCameraControlConfig {
    pub mouse_rotate_sensitivity: f32,
    pub mouse_translate_sensitivity: f32,
    pub smoothing_weight: f32,
}

pub fn orbit_camera_control_system(
    mut events: EventReader<CameraEvent>,
    mut cameras: Query<(&mut OrbitCameraController, &mut Transform)>,
) {
    let (mut camera, mut camera_transform) = if let Some((camera, tfm)) = cameras.iter_mut().next()
    {
        (camera, tfm)
    } else {
        return;
    };
    let OrbitCameraController {
        control_config,
        transform,
        smoother,
        enabled,
    } = &mut *camera;

    if *enabled {
        let look_vector = (transform.orbit - transform.pivot).normalize();
        let mut polar_vector = PolarVector::from_vector(look_vector);

        for event in events.iter() {
            match event {
                CameraEvent::Orbit(delta) => {
                    polar_vector.add_yaw(-control_config.mouse_rotate_sensitivity * delta.x);
                    polar_vector.add_pitch(control_config.mouse_rotate_sensitivity * delta.y);
                    polar_vector.assert_not_looking_up();
                }
                CameraEvent::Pan(delta) => {
                    let right_dir = camera_transform.rotation * -Vec3::X;
                    let up_dir = camera_transform.rotation * Vec3::Y;
                    transform.pivot += (delta.x * right_dir + delta.y * up_dir)
                        * control_config.mouse_translate_sensitivity;
                }
            }
        }

        transform.orbit = transform.pivot + transform.radius() * polar_vector.unit_vector();
    }

    *camera_transform = smoother
        .smooth_transform(control_config.smoothing_weight, transform)
        .orbit_look_at_pivot_transform();
}
