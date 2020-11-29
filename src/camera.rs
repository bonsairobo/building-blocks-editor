mod cursor_ray;
mod first_person;
mod orbit_transform;
mod third_person;

pub use cursor_ray::{CursorRay, CursorRayCalculator};
pub use first_person::{ControlConfig, FirstPersonCamera, FirstPersonCameraPlugin};
// pub use third_person::{ControlConfig, InputConfig};

use cursor_ray::{CursorRayCameraTag, CursorRayPlugin};
// use third_person::{ThirdPersonCamera, ThirdPersonCameraPlugin};

use bevy::{
    app::prelude::*, ecs::prelude::*, math::prelude::*, render::entity::Camera3dBundle,
    transform::components::Transform,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FirstPersonCameraPlugin)
            // .add_plugin(ThirdPersonCameraPlugin)
            .add_plugin(CursorRayPlugin)
            .add_startup_system(initialize_camera.system());
    }
}

fn initialize_camera(commands: &mut Commands) {
    let eye = Vec3::new(100.0, 100.0, 100.0);
    let target = Vec3::new(0.0, 0.0, 0.0);
    let control_config = ControlConfig {
        mouse_rotate_sensitivity: 0.005,
        mouse_translate_sensitivity: 0.5,
        trackpad_translate_sensitivity: 0.5,
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
        .with(FirstPersonCamera::new(control_config, eye, target))
        // .with(ThirdPersonCamera::new(control_config, eye, target))
        .current_entity()
        .unwrap()
}
