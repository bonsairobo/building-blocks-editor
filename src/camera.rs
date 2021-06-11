mod cursor_ray;
mod orbit_controller;
mod orbit_transform;
mod unreal_controller;

use crate::config::CameraConfig;

pub use cursor_ray::{CursorRay, CursorRayCalculator, CursorRayCameraTag};
pub use orbit_controller::{
    orbit_camera_control_system, orbit_camera_default_input_map, CameraEvent,
    OrbitCameraControlConfig, OrbitCameraController,
};
pub use unreal_controller::{
    unreal_camera_control_system, UnrealCameraControlConfig, UnrealCameraController,
};

use cursor_ray::CursorRayPlugin;

use bevy::{
    app::prelude::*, ecs::prelude::*, math::prelude::*, render::prelude::*, transform::prelude::*,
};

pub fn create_camera_entity(
    commands: &mut Commands,
    config: CameraConfig,
    eye: Vec3,
    target: Vec3,
) -> Entity {
    match config {
        CameraConfig::Unreal(config) => create_unreal_camera_entity(commands, config, eye, target),
        CameraConfig::Orbit(config) => create_orbit_camera_entity(commands, config, eye, target),
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(unreal_camera_control_system.system())
            .add_system(orbit_camera_default_input_map.system())
            .add_system(orbit_camera_control_system.system())
            .add_plugin(CursorRayPlugin)
            .add_event::<CameraEvent>();
    }
}

pub fn create_unreal_camera_entity(
    commands: &mut Commands,
    control_config: UnrealCameraControlConfig,
    eye: Vec3,
    target: Vec3,
) -> Entity {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(eye).looking_at(target, Vec3::Y),
            ..Default::default()
        })
        .insert(CursorRayCameraTag)
        .insert(UnrealCameraController::new(control_config, eye, target))
        .id()
}

pub fn create_orbit_camera_entity(
    commands: &mut Commands,
    control_config: OrbitCameraControlConfig,
    eye: Vec3,
    target: Vec3,
) -> Entity {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(eye).looking_at(target, Vec3::Y),
            ..Default::default()
        })
        .insert(CursorRayCameraTag)
        .insert(OrbitCameraController::new(control_config, target, eye))
        .id()
}
