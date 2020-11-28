use crate::{
    camera::cursor_ray::{CursorRayCalculator, WindowCameraData},
    geometry::{ray_plane_intersection, Plane, Ray3, RayPlaneIntersection},
};

use bevy::{
    input::{mouse::MouseWheel, prelude::*},
    math::prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct InputConfig {
    pub rotate_sensitivity_x: f32,
    pub rotate_sensitivity_y: f32,
    pub zoom_sensitivity: f32,
}

#[derive(Debug)]
pub struct ProcessedInput {
    pub radius_scalar: f32,
    pub delta_yaw: f32,
    pub delta_pitch: f32,
    pub target_translation: Vec3,
}

#[derive(Default)]
pub struct InputProcessor {
    prev_cursor_position: Vec2,
}

impl InputProcessor {
    pub fn process_input<'a>(
        &mut self,
        config: &InputConfig,
        cursor_position: Vec2,
        mouse_buttons: &Input<MouseButton>,
        mouse_wheel_events: impl IntoIterator<Item = &'a MouseWheel>,
        drag_plane: &Plane,
        cursor_ray_calc: &CursorRayCalculator,
    ) -> ProcessedInput {
        let radius_scalar = self.get_camera_radius_scalar_from_mouse_wheel_events(
            config.zoom_sensitivity,
            mouse_wheel_events,
        );

        let mut delta_yaw = 0.0;
        let mut delta_pitch = 0.0;
        let mut target_translation = Vec3::zero();

        let cursor_delta = cursor_position - self.prev_cursor_position;

        if mouse_buttons.pressed(MouseButton::Right) {
            delta_yaw = -cursor_delta.x * config.rotate_sensitivity_x;
            delta_pitch = -cursor_delta.y * config.rotate_sensitivity_y;
        }

        if let CursorRayCalculator(Some(window_cam)) = cursor_ray_calc {
            if mouse_buttons.pressed(MouseButton::Left) {
                target_translation = floor_drag_translation(
                    drag_plane,
                    window_cam,
                    cursor_position,
                    self.prev_cursor_position,
                );
            }
        }

        self.prev_cursor_position = cursor_position;

        ProcessedInput {
            radius_scalar,
            delta_yaw,
            delta_pitch,
            target_translation,
        }
    }

    fn get_camera_radius_scalar_from_mouse_wheel_events<'a>(
        &mut self,
        zoom_sensitivity: f32,
        events: impl IntoIterator<Item = &'a MouseWheel>,
    ) -> f32 {
        let mut radius_scalar = 1.0;
        for event in events.into_iter() {
            radius_scalar *= 1.0 - event.y * zoom_sensitivity;
        }

        radius_scalar
    }
}

fn floor_drag_translation(
    drag_plane: &Plane,
    window_cam: &WindowCameraData,
    cursor_pos: Vec2,
    prev_cursor_pos: Vec2,
) -> Vec3 {
    // It's actually important for correctness that we recalculate the rays using the current camera
    // transforms (as opposed to remembering the ray from the previous frame).
    let prev_screen_ray = window_cam.ray(prev_cursor_pos);
    let screen_ray = window_cam.ray(cursor_pos);

    let translation = _floor_drag_translation(drag_plane, &prev_screen_ray, &screen_ray);

    // Rotate the translation into the XZ (floor) plane.
    let floor_normal = Vec3::unit_y();
    let rot_axis = drag_plane.normal.cross(floor_normal);
    let angle = drag_plane.normal.angle_between(floor_normal);
    let rot = Quat::from_axis_angle(rot_axis, angle);

    rot * translation
}

fn _floor_drag_translation(drag_plane: &Plane, prev_screen_ray: &Ray3, screen_ray: &Ray3) -> Vec3 {
    if let RayPlaneIntersection::SinglePoint(p_now) = ray_plane_intersection(screen_ray, drag_plane)
    {
        if let RayPlaneIntersection::SinglePoint(p_prev) =
            ray_plane_intersection(prev_screen_ray, drag_plane)
        {
            return p_prev - p_now;
        }
    }

    Vec3::zero()
}
