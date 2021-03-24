use crate::camera::MouseCameraController;

use super::{selection::SelectionState, CurrentTool, SnapshottingVoxelEditor};

use crate::{
    camera::CursorRay,
    geometry::{closest_points_on_two_lines, Ray3},
    picking::VoxelCursor,
    VoxelType,
};

use bevy::{ecs::prelude::*, input::prelude::*};
use building_blocks::{
    core::{prelude::*, SignedAxis3},
    mesh::OrientedCubeFace,
    storage::Sd8,
};

#[derive(Clone, Copy)]
pub enum DragFaceState {
    SelectionReady,
    DraggingFace {
        quad_extent: Extent3i,
        normal: SignedAxis3,
        previous_drag_point: Point3i,
    },
}

pub fn drag_face_tool_system(
    mut current_tool: ResMut<CurrentTool>,
    mut voxel_editor: SnapshottingVoxelEditor,
    mut selection_state: ResMut<SelectionState>,
    mut mouse_camera_controllers: Query<&mut MouseCameraController>,
    voxel_cursor: VoxelCursor,
    cursor_ray: Res<CursorRay>,
) {
    let state = if let CurrentTool::DragFace(state) = &mut *current_tool {
        state
    } else {
        return;
    };

    match *state {
        DragFaceState::SelectionReady => {
            let (quad_extent, normal) = if let SelectionState::SelectionReady {
                quad_extent,
                normal,
            } = *selection_state
            {
                (quad_extent, normal)
            } else {
                return;
            };

            if let Some(voxel_face) = voxel_cursor.voxel_just_pressed(MouseButton::Left) {
                if quad_extent.contains(voxel_face.point) {
                    if let Some(mut controller) = mouse_camera_controllers.iter_mut().next() {
                        controller.disable();
                    }
                    *state = DragFaceState::DraggingFace {
                        quad_extent,
                        normal,
                        previous_drag_point: voxel_face.point,
                    };
                    *selection_state = SelectionState::Invisible;
                }
            }
        }
        DragFaceState::DraggingFace {
            mut quad_extent,
            normal,
            previous_drag_point,
        } => {
            let face = OrientedCubeFace::canonical(normal);

            // Drag the quad using the cursor.
            if let CursorRay(Some(ray)) = &*cursor_ray {
                // To drag the quad along it's normal axis, we need to project the cursor ray
                // onto that axis, which is equivalent to finding the two closest points on two
                // lines.
                let axis_line = Ray3::new(
                    Point3f::from(previous_drag_point).into(),
                    face.mesh_normal().into(),
                );
                if let Some((p1, _p2)) = closest_points_on_two_lines(&axis_line, ray) {
                    let old_quad_extent = quad_extent;

                    // Move the quad to a new position along the axis.
                    let new_drag_point = Point3f::from(p1).in_voxel();

                    if new_drag_point != previous_drag_point {
                        let new_axis_coord = new_drag_point.axis_component(normal.axis);
                        *quad_extent.minimum.axis_component_mut(normal.axis) = new_axis_coord;

                        let previous_axis_coord = previous_drag_point.axis_component(normal.axis);
                        let write_voxel =
                            if new_axis_coord * normal.sign > previous_axis_coord * normal.sign {
                                // We're dragging in the direction of the normal, so we should write solid voxels.
                                (VoxelType(2), Sd8::NEG_ONE)
                            } else {
                                // We're dragging in the opposite direction of the normal, so we should write empty voxels.
                                (VoxelType(0), Sd8::NEG_ONE)
                            };

                        // Write voxels in the extent between the old and new quad.
                        let fill_min = quad_extent.minimum.meet(old_quad_extent.minimum);
                        let fill_max = quad_extent.max().join(old_quad_extent.max());
                        let fill_extent = Extent3i::from_min_and_max(fill_min, fill_max);
                        voxel_editor.edit_extent_and_touch_neighbors(
                            fill_extent,
                            |_p, (v_type, v_dist)| {
                                *v_type = write_voxel.0;
                                *v_dist = write_voxel.1;
                            },
                        );

                        *state = DragFaceState::DraggingFace {
                            quad_extent,
                            normal,
                            previous_drag_point: new_drag_point,
                        };
                    }
                }
            }

            if voxel_cursor.mouse_input.just_released(MouseButton::Left) {
                // Done dragging.
                if let Some(mut controller) = mouse_camera_controllers.iter_mut().next() {
                    controller.enable();
                }
                voxel_editor.finish_edit();
                *state = DragFaceState::SelectionReady;
                *selection_state = SelectionState::SelectingFirstCorner;
            }
        }
    }
}
