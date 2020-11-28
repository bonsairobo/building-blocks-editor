mod controller;
mod input;
mod plugin;
mod smoother;
mod transform;
mod vector;

pub use controller::ControlConfig;
pub use input::InputConfig;
pub use plugin::ThirdPersonCameraPlugin;

use input::InputProcessor;
use smoother::TransformSmoother;
use transform::ThirdPersonTransform;

use bevy::math::prelude::*;

/// The component tracking the state of a third person camera entity.
///
/// Currently the `ThirdPersonCameraPlugin` only expects one such entity to exist.
pub struct ThirdPersonCamera {
    control_config: ControlConfig,
    input_processor: InputProcessor,
    transform: ThirdPersonTransform,
    smoother: TransformSmoother,
}

impl ThirdPersonCamera {
    pub fn new(control_config: ControlConfig, position: Vec3, target: Vec3) -> Self {
        Self {
            control_config,
            input_processor: InputProcessor::default(),
            transform: ThirdPersonTransform::from_position_and_target(position, target),
            smoother: TransformSmoother::default(),
        }
    }
}
