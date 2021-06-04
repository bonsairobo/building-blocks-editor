mod controller;
mod cursor_ray;
mod orbit_transform;

mod orbit_controller;

use crate::{config::CameraType, Config};

pub use controller::{mouse_camera_control_system, CameraControlConfig, MouseCameraController};
pub use cursor_ray::{CursorRay, CursorRayCalculator, CursorRayCameraTag};

use cursor_ray::CursorRayPlugin;

use bevy::{
    app::prelude::*, ecs::prelude::*, math::prelude::*, render::prelude::*, transform::prelude::*,
};

pub fn create_camera_entity(
    commands: &mut Commands,
    control_config: CameraControlConfig,
    config: Config,
    eye: Vec3,
    target: Vec3,
) -> Entity {
    match config.camera {
        CameraType::Unreal => create_unreal_camera_entity(commands, control_config, eye, target),
        CameraType::Orbit => create_orbit_camera_entity(commands, eye, target),
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(mouse_camera_control_system.system())
            .add_plugin(CursorRayPlugin);
    }
}

pub fn create_unreal_camera_entity(
    commands: &mut Commands,
    control_config: CameraControlConfig,
    eye: Vec3,
    target: Vec3,
) -> Entity {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(eye).looking_at(target, Vec3::Y),
            ..Default::default()
        })
        .insert(CursorRayCameraTag)
        .insert(MouseCameraController::new(control_config, eye, target))
        .id()
}

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(orbit_controller::orbit_camera_default_input_map.system())
            .add_system(bevy_orbit_controls::OrbitCameraPlugin::mouse_motion_system.system())
            .add_system(bevy_orbit_controls::OrbitCameraPlugin::emit_zoom_events.system())
            .add_system(bevy_orbit_controls::OrbitCameraPlugin::zoom_system.system())
            .add_system(bevy_orbit_controls::OrbitCameraPlugin::update_transform_system.system())
            .add_event::<bevy_orbit_controls::CameraEvents>()
            .add_plugin(CursorRayPlugin);
    }
}

pub fn create_orbit_camera_entity(commands: &mut Commands, eye: Vec3, target: Vec3) -> Entity {
    let distance = (target - eye).length();
    let controller = bevy_orbit_controls::OrbitCamera {
        distance,
        center: target,
        pan_sensitivity: 10.0,
        ..Default::default()
    };

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(eye).looking_at(target, Vec3::Y),
            ..Default::default()
        })
        .insert(CursorRayCameraTag)
        .insert(controller)
        .id()
}
