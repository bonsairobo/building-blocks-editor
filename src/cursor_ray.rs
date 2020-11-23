use crate::geometry::{ray_from_window_point, Ray3};

use bevy::{
    app::prelude::*, ecs::prelude::*, math::prelude::*, render::camera::Camera,
    transform::components::Transform, window::prelude::*,
};

/// A ray, cast from the camera's position in the direction of the window's cursor.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CursorRay(pub Option<Ray3>);

pub struct CursorRayPlugin;

impl Plugin for CursorRayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(CursorRay::default())
            .add_system(cursor_ray_system.system());
    }
}

fn cursor_ray_system(
    cursor_moved_events: Res<Events<CursorMoved>>,
    cameras: Query<(&Camera, &Transform)>,
    windows: Res<Windows>,
    mut cursor_position: Local<CursorPosition>,
    mut cursor_ray: ResMut<CursorRay>,
) {
    if let Some((camera, camera_tfm)) = cameras.iter().next() {
        let mut cursor_reader = EventReader::default();
        if let Some(event) = cursor_reader.latest(&cursor_moved_events) {
            cursor_position.0 = event.position;
        }

        let window = windows.get(camera.window).unwrap();
        *cursor_ray = CursorRay(Some(ray_from_window_point(
            cursor_position.0,
            (window.width(), window.height()),
            camera_tfm.compute_matrix(),
            camera.projection_matrix,
        )));
    } else {
        *cursor_ray = CursorRay::default();
    }
}

#[derive(Default)]
struct CursorPosition(Vec2);
