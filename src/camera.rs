mod controller;
mod cursor_ray;
mod orbit_transform;

pub use controller::{mouse_camera_control_system, CameraControlConfig, MouseCameraController};
pub use cursor_ray::{CursorRay, CursorRayCalculator, CursorRayCameraTag};

use cursor_ray::CursorRayPlugin;

use bevy::{
    app::prelude::*, ecs::prelude::*, math::prelude::*, render::prelude::*, transform::prelude::*,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(mouse_camera_control_system)
            .add_plugin(CursorRayPlugin);
    }
}

pub fn create_camera_entity(
    commands: &mut Commands,
    control_config: CameraControlConfig,
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
