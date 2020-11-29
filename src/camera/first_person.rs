use super::orbit_transform::{OrbitTransform, Smoother};

mod controller;

pub use controller::ControlConfig;

use controller::first_person_camera_control_system;

use bevy::{app::prelude::*, ecs::prelude::*, math::prelude::*};

pub struct FirstPersonCameraPlugin;

impl Plugin for FirstPersonCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(first_person_camera_control_system.system());
    }
}

/// A first-person camera controlled with the mouse in the same way as Unreal Engine's viewport
/// controller.
pub struct FirstPersonCamera {
    control_config: ControlConfig,
    transform: OrbitTransform,
    smoother: Smoother,
    prev_cursor_position: Vec2,
}

impl FirstPersonCamera {
    pub fn new(control_config: ControlConfig, position: Vec3, target: Vec3) -> Self {
        Self {
            control_config,
            transform: OrbitTransform::from_pivot_and_orbit(position, target),
            prev_cursor_position: Default::default(),
            smoother: Default::default(),
        }
    }
}
