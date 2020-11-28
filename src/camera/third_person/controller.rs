use crate::{camera::cursor_ray::CursorRayCalculator, geometry::Plane, CursorPosition};

use super::{
    input::ProcessedInput, smoother::TransformSmoother, transform::ThirdPersonTransform,
    InputConfig, ThirdPersonCamera,
};

use bevy::{
    app::prelude::*,
    ecs::prelude::*,
    input::{mouse::MouseWheel, prelude::*},
    transform::components::Transform,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ControlConfig {
    pub min_radius: f32,
    pub max_radius: f32,
    pub smoothing_weight: f32,
    pub input_config: InputConfig,
}

pub fn third_person_camera_control_system(
    mut mouse_wheel_reader: Local<MouseWheelReader>,
    mouse_wheel: Res<Events<MouseWheel>>,
    mouse_buttons: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    cursor_ray_calc: Res<CursorRayCalculator>,
    mut cameras: Query<(&mut ThirdPersonCamera, &mut Transform)>,
) {
    let (mut camera, mut camera_transform) = if let Some((camera, tfm)) = cameras.iter_mut().next()
    {
        (camera, tfm)
    } else {
        return;
    };
    let ThirdPersonCamera {
        control_config,
        input_processor,
        transform,
        smoother,
    } = &mut *camera;

    // We cast cursor rays against this plane so the user can drag along it in order to translate
    // the camera.
    let drag_plane = Plane {
        origin: transform.target,
        // Avoid having the eye vector be coplanar, which causes headaches for the floor
        // dragging controls.
        normal: transform.eye_vec.unit_vector(),
    };
    // Interpret device input into control data.
    let mouse_wheel_events = mouse_wheel_reader.reader.iter(&mouse_wheel);
    let input = input_processor.process_input(
        &control_config.input_config,
        cursor_position.0,
        &*mouse_buttons,
        mouse_wheel_events,
        &drag_plane,
        &*cursor_ray_calc,
    );

    *camera_transform = update_transform(control_config, &input, smoother, transform);
}

fn update_transform(
    control_config: &ControlConfig,
    input: &ProcessedInput,
    smoother: &mut TransformSmoother,
    transform: &mut ThirdPersonTransform,
) -> Transform {
    // Translate the target.
    transform.target += input.target_translation;
    // Rotate around the target.
    transform.add_yaw(input.delta_yaw);
    transform.add_pitch(input.delta_pitch);
    transform.eye_vec.assert_not_looking_up();
    // Scale the camera's distance from the target.
    transform.radius = (transform.radius * input.radius_scalar)
        .min(control_config.max_radius)
        .max(control_config.min_radius);
    // Apply a smoothing filter to the transform.
    smoother
        .smooth_transform(control_config.smoothing_weight, &transform)
        .transform()
}

#[derive(Default)]
pub struct MouseWheelReader {
    reader: EventReader<MouseWheel>,
}
