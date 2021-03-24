use crate::{
    geometry::{ray_from_window_point, Ray3},
    CursorPosition,
};

use bevy::{
    app::prelude::*, ecs::prelude::*, math::prelude::*, render::camera::Camera,
    transform::components::Transform, window::prelude::*,
};

/// Designates the camera entity which should be used for calculating the cursor ray.
pub struct CursorRayCameraTag;

/// A ray, cast from the camera's position in the direction of the window's cursor. `None` if the
/// main camera is missing (camera with the `CursorRayCameraTag` component).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CursorRay(pub Option<Ray3>);

/// Manages the `CursorRay` and `CursorRayCalculator` resources.
pub struct CursorRayPlugin;

impl Plugin for CursorRayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(CursorRay::default())
            .insert_resource(CursorRayCalculator::default())
            .add_system(cursor_ray_system.system());
    }
}

fn cursor_ray_system(
    cameras: Query<(&CursorRayCameraTag, &Camera, &Transform)>,
    windows: Res<Windows>,
    cursor_position: Res<CursorPosition>,
    mut cursor_ray_calculator: ResMut<CursorRayCalculator>,
    mut cursor_ray: ResMut<CursorRay>,
) {
    if let Some((_, camera, camera_tfm)) = cameras.iter().next() {
        let window = windows.get(camera.window).unwrap();
        let data = WindowCameraData {
            screen_size: (window.width(), window.height()),
            camera_transform: camera_tfm.compute_matrix(),
            camera_projection: camera.projection_matrix,
        };
        *cursor_ray = CursorRay(Some(data.ray(cursor_position.current)));
        *cursor_ray_calculator = CursorRayCalculator(Some(data));
    } else {
        *cursor_ray = CursorRay::default();
        *cursor_ray_calculator = CursorRayCalculator::default();
    }
}

/// A resource to conveniently calculate rays based on arbitrary window coordinates.
/// `None` if the main camera is missing.
#[derive(Default)]
pub struct CursorRayCalculator(pub Option<WindowCameraData>);

pub struct WindowCameraData {
    screen_size: (f32, f32),
    camera_transform: Mat4,
    camera_projection: Mat4,
}

impl WindowCameraData {
    /// Calculates the ray from camera eye to cursor in window coordinates `point`.
    pub fn ray(&self, point: Vec2) -> Ray3 {
        ray_from_window_point(
            point,
            self.screen_size,
            self.camera_transform,
            self.camera_projection,
        )
    }
}
