use bevy::math::prelude::*;
use building_blocks::partition::ncollide3d::query::Ray as NCRay;

pub struct Ray3 {
    pub point: Vec3,
    pub direction: Vec3,
}

impl Ray3 {
    pub fn new(point: Vec3, direction: Vec3) -> Self {
        Self { point, direction }
    }
}

impl From<Ray3> for NCRay<f32> {
    fn from(ray: Ray3) -> Self {
        let point = mint::Point3::from(ray.point).into();
        let direction = mint::Vector3::from(ray.direction).into();

        NCRay::new(point, direction)
    }
}

/// Constructs a 3D ray starting from the window camera's near plane, pointing toward the screen
/// space `point`.
pub fn ray_from_window_point(
    point: Vec2,
    screen_size: (u32, u32),
    camera_transform_matrix: Mat4,
    camera_projection_matrix: Mat4,
) -> Ray3 {
    let screen_size = Vec2::new(screen_size.0 as f32, screen_size.1 as f32);

    // Normalized device coordinates (NDC) describes cursor position from (-1, -1, -1) to (1, 1, 1).
    let cursor_pos_ndc: Vec3 = ((point / screen_size) * 2.0 - Vec2::from([1.0, 1.0])).extend(1.0);

    let (_, _, camera_position) = camera_transform_matrix.to_scale_rotation_translation();

    let ndc_to_world = camera_transform_matrix * camera_projection_matrix.inverse();
    let cursor_position = ndc_to_world.transform_point3(cursor_pos_ndc);

    let ray_direction = cursor_position - camera_position;

    Ray3::new(camera_position, ray_direction)
}
