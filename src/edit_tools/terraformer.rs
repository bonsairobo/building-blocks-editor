use super::{CurrentTool, SnapshottingVoxelEditor, events::TerraformerEvents};

use crate::{voxel::EMPTY_VOXEL_TYPE, CursorRay, VoxelCursor, VoxelType};

use bevy::{ecs::prelude::*, input::{mouse::MouseWheel, prelude::*}, prelude::*};
use building_blocks::{core::prelude::*, storage::Sd8};

pub struct Terraformer {
    edit_radius: u32,
    voxel_type: VoxelType,
    dist_from_camera: Option<f32>,
}

impl Default for Terraformer {
    fn default() -> Self {
        Self {
            edit_radius: 10,
            voxel_type: VoxelType(1),
            dist_from_camera: None,
        }
    }
}

pub fn terraformer_default_input_map(
    mut events: EventWriter<TerraformerEvents>,
    keyboard: Res<Input<KeyCode>>,
) {
    // Adjust the edit radius.
    if keyboard.just_pressed(KeyCode::Up) {
        events.send(TerraformerEvents::ChangeEditRadius(
            1,
        ))
    } else if keyboard.just_pressed(KeyCode::Down) {
        events.send(TerraformerEvents::ChangeEditRadius(
            -1,
        ))
    }

    // Adjust the voxel type to create.
    if keyboard.just_pressed(KeyCode::Key1) {
        events.send(TerraformerEvents::ChangeVoxelType(1));
    } else if keyboard.just_pressed(KeyCode::Key2) {
        events.send(TerraformerEvents::ChangeVoxelType(2));
    } else if keyboard.just_pressed(KeyCode::Key3) {
        events.send(TerraformerEvents::ChangeVoxelType(3));
    } else if keyboard.just_pressed(KeyCode::Key4) {
        events.send(TerraformerEvents::ChangeVoxelType(4));
    }

    
}

pub fn terraformer_system(
    current_tool: Res<CurrentTool>,
    mut terraformer: ResMut<Terraformer>,
    mut voxel_editor: SnapshottingVoxelEditor,
    voxel_cursor: VoxelCursor,
    cursor_ray: Res<CursorRay>,
    keyboard: Res<Input<KeyCode>>,
    mut events: EventReader<TerraformerEvents>,
) {
    if let CurrentTool::Terraform = *current_tool {
    } else {
        return;
    }

    events.iter().for_each(|event| {
        if let TerraformerEvents::ChangeEditRadius(delta) = event {
            terraformer.edit_radius = ((*delta as i32 + terraformer.edit_radius as i32) as u32).max(1);
        }

        // Adjust the voxel type to create.
        if let TerraformerEvents::ChangeVoxelType(voxel_type) = event {
            terraformer.voxel_type = VoxelType(*voxel_type);
        }
    });

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
    if keyboard.pressed(KeyCode::Z) {
        lock_edit_dist_from_camera = true;
        edit_sphere(
            TerraformOperation::MakeSolid,
            edit_center,
            terraformer.edit_radius,
            terraformer.voxel_type,
            &mut voxel_editor,
        );
    } else if keyboard.pressed(KeyCode::X) {
        lock_edit_dist_from_camera = true;
        edit_sphere(
            TerraformOperation::RemoveSolid,
            edit_center,
            terraformer.edit_radius,
            EMPTY_VOXEL_TYPE,
            &mut voxel_editor,
        );
    }

    if keyboard.just_released(KeyCode::Z) || keyboard.just_released(KeyCode::X) {
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
    voxel_type: VoxelType,
    voxel_editor: &mut SnapshottingVoxelEditor,
) {
    let fradius = radius as f32;
    let sign = match operation {
        TerraformOperation::MakeSolid => -1,
        TerraformOperation::RemoveSolid => 1,
    };
    voxel_editor.edit_extent_and_touch_neighbors(
        centered_extent(center, radius),
        |p: Point3i, (v_type, v_dist): (&mut VoxelType, &mut Sd8)| {
            let p_radius = (p - center).norm();

            // Change the SDF faster closer to the center.
            const SDF_GROWTH_FACTOR: f32 = 20.0;
            let sdf_delta = sign
                * (SDF_GROWTH_FACTOR * (1.0 - p_radius / fradius))
                    .max(0.0)
                    .round() as i16;
            let new_dist = v_dist.0 as i16 + sdf_delta;

            v_dist.0 = new_dist.max(std::i8::MIN as i16).min(std::i8::MAX as i16) as i8;

            if sdf_delta < 0 && v_dist.0 < 0 {
                // Only set to the brush type if the voxel is solid.
                *v_type = voxel_type;
            } else if sdf_delta > 0 && v_dist.0 >= 0 {
                *v_type = EMPTY_VOXEL_TYPE;
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
