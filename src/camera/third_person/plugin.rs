use super::controller::third_person_camera_control_system;

use bevy::{app::prelude::*, ecs::prelude::*};

/// Provides a control system for the camera entity with the `ThirdPersonCamera` component.
pub struct ThirdPersonCameraPlugin;

impl Plugin for ThirdPersonCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(third_person_camera_control_system.system());
    }
}
