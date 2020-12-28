use super::{CurrentTool, SnapshottingVoxelEditor};

use crate::{CursorRay, SdfVoxel, SdfVoxelType, VoxelCursor, EMPTY_SDF_VOXEL};

use bevy::{ecs::prelude::*, input::prelude::*};
use building_blocks::core::prelude::*;

pub struct Terraformer {
    edit_radius: u32,
    voxel_type: SdfVoxelType,
    dist_from_camera: Option<f32>,
}

impl Default for Terraformer {
    fn default() -> Self {
        Self {
            edit_radius: 10,
            voxel_type: SdfVoxelType(1),
            dist_from_camera: None,
        }
    }
}

pub fn terraformer_system(
    current_tool: Res<CurrentTool>,
    mut terraformer: ResMut<Terraformer>,
    mut voxel_editor: SnapshottingVoxelEditor,
    voxel_cursor: VoxelCursor,
    cursor_ray: Res<CursorRay>,
    keyboard: Res<Input<KeyCode>>,
) {
    if let CurrentTool::Terraform = *current_tool {
    } else {
        return;
    }

    // Adjust the edit radius.
    if keyboard.just_pressed(KeyCode::Up) {
        terraformer.edit_radius += 1;
    } else if keyboard.just_pressed(KeyCode::Down) {
        terraformer.edit_radius = (terraformer.edit_radius - 1).max(1);
    }
    // Adjust the voxel type to create.
    if keyboard.just_pressed(KeyCode::Key1) {
        terraformer.voxel_type = SdfVoxelType(1);
    } else if keyboard.just_pressed(KeyCode::Key2) {
        terraformer.voxel_type = SdfVoxelType(2);
    } else if keyboard.just_pressed(KeyCode::Key3) {
        terraformer.voxel_type = SdfVoxelType(3);
    } else if keyboard.just_pressed(KeyCode::Key4) {
        terraformer.voxel_type = SdfVoxelType(4);
    }

    let cursor_ray = if let CursorRay(Some(ray)) = *cursor_ray {
        ray
    } else {
        return;
    };

    // Determine the sphere we should edit.
    let edit_center =
        cursor_ray.origin + terraformer.dist_from_camera.unwrap_or(20.0) * cursor_ray.direction;
    let edit_center = Point3f::from(edit_center).in_voxel();

    let mut lock_edit_dist_from_camera = false;
    if keyboard.pressed(KeyCode::Space) {
        lock_edit_dist_from_camera = true;
        edit_sphere(
            TerraformOperation::MakeSolid,
            edit_center,
            terraformer.edit_radius,
            terraformer.voxel_type,
            &mut voxel_editor,
        );
    } else if keyboard.pressed(KeyCode::Back) {
        lock_edit_dist_from_camera = true;
        edit_sphere(
            TerraformOperation::RemoveSolid,
            edit_center,
            terraformer.edit_radius,
            EMPTY_SDF_VOXEL.voxel_type,
            &mut voxel_editor,
        );
    }

    if keyboard.just_released(KeyCode::Space) || keyboard.just_released(KeyCode::Back) {
        voxel_editor.finish_edit();
    }

    if !lock_edit_dist_from_camera {
        terraformer.dist_from_camera = voxel_cursor
            .impact
            .maybe_impact
            .as_ref()
            .map(|i| i.impact.toi);
    }
}

fn edit_sphere(
    operation: TerraformOperation,
    center: Point3i,
    radius: u32,
    voxel_type: SdfVoxelType,
    voxel_editor: &mut SnapshottingVoxelEditor,
) {
    let fradius = radius as f32;
    let sign = match operation {
        TerraformOperation::MakeSolid => -1,
        TerraformOperation::RemoveSolid => 1,
    };
    voxel_editor.edit_extent_and_touch_neighbors(
        centered_extent(center, radius),
        |p: Point3i, v: &mut SdfVoxel| {
            let p_radius = (p - center).norm();

            // Change the SDF faster closer to the center.
            const SDF_GROWTH_FACTOR: f32 = 20.0;
            let sdf_delta = sign
                * (SDF_GROWTH_FACTOR * (1.0 - p_radius / fradius))
                    .max(0.0)
                    .round() as i16;
            let new_dist = v.distance.0 as i16 + sdf_delta;

            v.distance.0 = new_dist.max(std::i8::MIN as i16).min(std::i8::MAX as i16) as i8;

            if sdf_delta < 0 && v.distance.0 < 0 {
                // Only set to the brush type if the voxel is solid.
                v.voxel_type = voxel_type;
            } else if sdf_delta > 0 && v.distance.0 >= 0 {
                v.voxel_type = EMPTY_SDF_VOXEL.voxel_type;
            }
        },
    );
}

fn centered_extent(center: Point3i, radius: u32) -> Extent3i {
    let r = radius as i32;
    let min = center - PointN([r; 3]);
    let shape = PointN([2 * r + 1; 3]);

    Extent3i::from_min_and_shape(min, shape)
}

#[derive(Clone, Copy)]
enum TerraformOperation {
    MakeSolid,
    RemoveSolid,
}
