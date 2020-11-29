use super::orbit_transform::{OrbitTransform, Smoother};

mod controller;
mod input;
mod plugin;

pub use controller::ControlConfig;
pub use input::InputConfig;
pub use plugin::ThirdPersonCameraPlugin;

use input::InputProcessor;

use bevy::math::prelude::*;

/// The component tracking the state of a third person camera entity.
///
/// Currently the `ThirdPersonCameraPlugin` only expects one such entity to exist.
pub struct ThirdPersonCamera {
    control_config: ControlConfig,
    input_processor: InputProcessor,
    transform: OrbitTransform,
    smoother: Smoother,
}

impl ThirdPersonCamera {
    pub fn new(control_config: ControlConfig, position: Vec3, target: Vec3) -> Self {
        Self {
            control_config,
            input_processor: InputProcessor::default(),
            transform: OrbitTransform::from_pivot_and_orbit(target, position),
            smoother: Smoother::default(),
        }
    }
}
