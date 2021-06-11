mod cursor_ray;

use crate::config::CameraConfig;

pub use cursor_ray::{CursorRay, CursorRayCalculator, CursorRayCameraTag};

use cursor_ray::CursorRayPlugin;

use bevy::{app::prelude::*, ecs::prelude::*, math::prelude::*};
use smooth_bevy_cameras::{
    controllers::{orbit::*, unreal::*},
    LookTransformPlugin,
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
        app.add_plugin(LookTransformPlugin)
            .add_plugin(UnrealCameraPlugin)
            .add_plugin(OrbitCameraPlugin)
            .add_plugin(CursorRayPlugin);
    }
}

pub fn create_unreal_camera_entity(
    commands: &mut Commands,
    controller: UnrealCameraController,
    eye: Vec3,
    target: Vec3,
) -> Entity {
    commands
        .spawn_bundle(UnrealCameraBundle::new(
            controller,
            Default::default(),
            eye,
            target,
        ))
        .insert(CursorRayCameraTag)
        .id()
}

pub fn create_orbit_camera_entity(
    commands: &mut Commands,
    controller: OrbitCameraController,
    eye: Vec3,
    target: Vec3,
) -> Entity {
    commands
        .spawn_bundle(OrbitCameraBundle::new(
            controller,
            Default::default(),
            eye,
            target,
        ))
        .insert(CursorRayCameraTag)
        .id()
}
