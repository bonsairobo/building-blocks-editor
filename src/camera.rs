mod cursor_ray;

use crate::config::CameraConfig;

pub use cursor_ray::{CursorRay, CursorRayCalculator, CursorRayCameraTag};

use cursor_ray::CursorRayPlugin;

use bevy::{app::prelude::*, ecs::prelude::*, math::prelude::*};
use smooth_bevy_cameras::{
    OrbitCameraBundle, OrbitCameraControlConfig, SmoothCamerasPlugin, UnrealCameraBundle,
    UnrealCameraControlConfig,
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
        app.add_plugin(SmoothCamerasPlugin)
            .add_plugin(CursorRayPlugin);
    }
}

pub fn create_unreal_camera_entity(
    commands: &mut Commands,
    control_config: UnrealCameraControlConfig,
    eye: Vec3,
    target: Vec3,
) -> Entity {
    commands
        .spawn_bundle(UnrealCameraBundle::new(
            Default::default(),
            control_config,
            eye,
            target,
        ))
        .insert(CursorRayCameraTag)
        .id()
}

pub fn create_orbit_camera_entity(
    commands: &mut Commands,
    control_config: OrbitCameraControlConfig,
    eye: Vec3,
    target: Vec3,
) -> Entity {
    commands
        .spawn_bundle(OrbitCameraBundle::new(
            Default::default(),
            control_config,
            eye,
            target,
        ))
        .insert(CursorRayCameraTag)
        .id()
}
