use crate::voxel::{VoxelCursor, VoxelFace};

use bevy::{ecs::prelude::*, input::prelude::*};
use building_blocks::core::{prelude::*, SignedAxis3};

#[derive(Clone, Copy)]
pub enum SelectionState {
    SelectingFirstCorner,
    SelectingSecondCorner {
        /// The first voxel face clicked.
        first_corner: VoxelFace,
        /// The voxel face that the cursor is currently hovering on, if it's a valid selection.
        valid_hover: Option<VoxelFace>,
    },
    SelectionReady {
        /// The quad of voxels selected.
        quad_extent: Extent3i,
        /// The normal direction of the selected face.
        normal: SignedAxis3,
    },
    Invisible,
}

pub fn selection_control_system(
    mut selection_state: ResMut<SelectionState>,
    voxel_cursor: VoxelCursor,
) {
    match &mut *selection_state {
        SelectionState::SelectingFirstCorner => {
            if let Some(first_corner) = voxel_cursor.voxel_just_clicked(MouseButton::Left) {
                *selection_state = SelectionState::SelectingSecondCorner {
                    first_corner,
                    valid_hover: Some(first_corner),
                };
            }
        }
        SelectionState::SelectingSecondCorner {
            first_corner,
            valid_hover,
        } => {
            *valid_hover = None;
            let mut valid_selection = false;
            if let Some(hover_face) = voxel_cursor.impact.get_voxel_face() {
                if selection_corners_are_compatible(first_corner, &hover_face) {
                    *valid_hover = Some(hover_face);
                    if voxel_cursor.voxel_just_clicked(MouseButton::Left).is_some() {
                        valid_selection = true;
                        *selection_state = SelectionState::SelectionReady {
                            quad_extent: Extent3i::from_corners(
                                first_corner.point,
                                hover_face.point,
                            ),
                            normal: first_corner.normal,
                        };
                    }
                }
            }
            if !valid_selection && voxel_cursor.voxel_just_clicked(MouseButton::Left).is_some() {
                *selection_state = SelectionState::SelectingFirstCorner;
            }
        }
        _ => (), // wait to get kicked back into the initial state another system
    }
}

fn selection_corners_are_compatible(corner1: &VoxelFace, corner2: &VoxelFace) -> bool {
    corner1.normal == corner2.normal
        && corner1.point.axis_component(corner1.normal.axis)
            == corner2.point.axis_component(corner2.normal.axis)
}
