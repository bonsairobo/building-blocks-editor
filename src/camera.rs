mod controller;
mod cursor_ray;
mod orbit_transform;

pub use controller::{mouse_camera_control_system, ControlConfig, MouseCameraController};
pub use cursor_ray::{CursorRay, CursorRayCalculator};

use cursor_ray::{CursorRayCameraTag, CursorRayPlugin};

use bevy::{
    app::prelude::*, ecs::prelude::*, math::prelude::*, render::entity::Camera3dBundle,
    transform::components::Transform,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(initialize_camera.system())
            .add_system(mouse_camera_control_system.system())
            .add_plugin(CursorRayPlugin);
    }
}

fn initialize_camera(commands: &mut Commands) {
    let eye = Vec3::new(100.0, 100.0, 100.0);
    let target = Vec3::new(0.0, 0.0, 0.0);
    let control_config = ControlConfig {
        mouse_rotate_sensitivity: 0.002,
        mouse_translate_sensitivity: 0.1,
        trackpad_translate_sensitivity: 0.1,
        smoothing_weight: 0.9,
    };
    create_camera_entity(commands, control_config, eye, target);
}

fn create_camera_entity(
    commands: &mut Commands,
    control_config: ControlConfig,
    eye: Vec3,
    target: Vec3,
) -> Entity {
    let mut camera_components = Camera3dBundle::default();
    camera_components.transform =
        Transform::from_matrix(Mat4::face_toward(eye, target, Vec3::unit_y()));

    commands
        .spawn(camera_components)
        .with(CursorRayCameraTag)
        .with(MouseCameraController::new(control_config, eye, target))
        .current_entity()
        .unwrap()
}
