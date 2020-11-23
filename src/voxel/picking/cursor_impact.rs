use crate::{voxel::geometry::VoxelFace, CursorRay, VoxelBVT};

use bevy::ecs::prelude::*;
use building_blocks::{
    core::{prelude::*, SignedAxis3},
    search::{
        collision::{voxel_ray_cast, VoxelRayImpact},
        ncollide3d::query::Ray as NCRay,
    },
};

/// The closest voxel that the window cursor is touching.
#[derive(Default)]
pub struct VoxelCursorRayImpact {
    pub maybe_impact: Option<VoxelRayImpact>,
    pub face: Option<SignedAxis3>,
}

impl VoxelCursorRayImpact {
    pub fn get(&self) -> Option<(&VoxelRayImpact, &SignedAxis3)> {
        match (self.maybe_impact.as_ref(), self.face.as_ref()) {
            (Some(i), Some(f)) => Some((i, f)),
            _ => None,
        }
    }

    pub fn get_voxel_face(&self) -> Option<VoxelFace> {
        self.get().map(|(impact, face)| VoxelFace {
            point: impact.point,
            face: *face,
        })
    }
}

/// Each frame, a ray is cast at the `VoxelBVT`, and the resulting impact is stored.
pub fn voxel_cursor_impact_system(
    bvt: Res<VoxelBVT>,
    cursor_ray: Res<CursorRay>,
    mut voxel_cursor_impact: ResMut<VoxelCursorRayImpact>,
) {
    voxel_cursor_impact.maybe_impact = None;
    voxel_cursor_impact.face = None;

    if let CursorRay(Some(ray)) = cursor_ray.clone() {
        if let Some(impact) = voxel_ray_cast(&*bvt, NCRay::from(ray), std::f32::INFINITY, |_| true)
        {
            let normal = Point3f::from(impact.impact.normal.normalize())
                .round()
                .in_voxel();
            if let Some(normal_axis) = SignedAxis3::from_vector(normal) {
                voxel_cursor_impact.face = Some(normal_axis);
            }
            voxel_cursor_impact.maybe_impact = Some(impact);
        }
    }
}
