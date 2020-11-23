use bevy::math::prelude::*;

pub use building_blocks::search::ncollide3d::{na, query::Ray as NCRay};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ray3 {
    pub point: Vec3,
    pub direction: Vec3,
}

impl Ray3 {
    pub fn new(point: Vec3, direction: Vec3) -> Self {
        Self { point, direction }
    }

    pub fn is_parallel_to(&self, other: &Self) -> bool {
        vectors_are_parallel(self.direction, other.direction)
    }
}

pub fn vectors_are_parallel(v1: Vec3, v2: Vec3) -> bool {
    approx::relative_eq!(v1.angle_between(v2), 0.0)
}

impl From<Ray3> for NCRay<f32> {
    fn from(ray: Ray3) -> Self {
        NCRay::new(
            mint::Point3::from(ray.point).into(),
            mint::Vector3::from(ray.direction).into(),
        )
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

pub fn closest_points_on_two_lines(l1: &Ray3, l2: &Ray3) -> Option<(Vec3, Vec3)> {
    // The key insight is that the vector between the two points must be perpendicular to both
    // lines. So we end up with this linear system:
    //
    // t1 * V1 - t2 * V2 + t3 * (V1 x V2) = P2 - P1

    // If the lines are parallel, then there are infinitely many solutions, so return None.
    if l1.is_parallel_to(l2) {
        return None;
    }

    let col1 = l1.direction;
    let col2 = -l2.direction;
    let col3 = l1.direction.cross(l2.direction);
    let rhs = l2.point - l1.point;

    let mat = Mat3::from_cols(col1, col2, col3);
    let t = mat.inverse() * rhs;

    Some((l1.point + t.x * l1.direction, l2.point + t.y * l2.direction))
}

// ████████╗███████╗███████╗████████╗
// ╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝
//    ██║   █████╗  ███████╗   ██║
//    ██║   ██╔══╝  ╚════██║   ██║
//    ██║   ███████╗███████║   ██║
//    ╚═╝   ╚══════╝╚══════╝   ╚═╝

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sanity_test_closest_points_on_two_lines() {
        let l1 = Ray3::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let l2 = Ray3::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

        let (p1, p2) = closest_points_on_two_lines(&l1, &l2).unwrap();

        assert_eq!(p1, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(p2, Vec3::new(0.0, 1.0, 0.0));
    }
}
