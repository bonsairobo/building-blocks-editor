use crate::{geometry::ray_from_window_point, VoxelBVT};

use bevy::{prelude::*, render::camera::Camera};
use building_blocks::search::{
    collision::{voxel_ray_cast, VoxelRayImpact},
    ncollide3d::query::Ray as NCRay,
};

/// Manages the `VoxelCursorRayImpact` resource. Each frame, a ray is cast at the `VoxelBVT`, and
/// the resulting impact is stored.
pub struct VoxelPickingPlugin;

impl Plugin for VoxelPickingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(VoxelCursorRayImpact::default())
            .add_system(voxel_picking_system.system());
    }
}

/// The closest voxel that the window cursor is touching.
#[derive(Default)]
pub struct VoxelCursorRayImpact {
    pub maybe_impact: Option<VoxelRayImpact>,
}

fn voxel_picking_system(
    bvt: Res<VoxelBVT>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    cameras: Query<(&Camera, &Transform)>,
    windows: Res<Windows>,
    mut cursor_position: Local<CursorPosition>,
    mut picked_voxel: ResMut<VoxelCursorRayImpact>,
) {
    if let Some((camera, camera_tfm)) = cameras.iter().next() {
        let mut cursor_reader = EventReader::default();
        if let Some(event) = cursor_reader.latest(&cursor_moved_events) {
            cursor_position.0 = event.position;
        }

        let window = windows.get(camera.window).unwrap();
        let pick_ray = NCRay::from(ray_from_window_point(
            cursor_position.0,
            (window.width(), window.height()),
            camera_tfm.compute_matrix(),
            camera.projection_matrix,
        ));
        picked_voxel.maybe_impact = voxel_ray_cast(&*bvt, pick_ray, std::f32::INFINITY, |_| true);
    }
}

#[derive(Default)]
struct CursorPosition(Vec2);
