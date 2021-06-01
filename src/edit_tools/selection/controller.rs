use crate::picking::{VoxelCursor, VoxelFace};

use bevy::{
    ecs::prelude::*,
    input::prelude::*,
    prelude::{EventReader, EventWriter},
};
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

pub enum SelectionEvents {
    SelectFirstCorner(VoxelFace),
    HoverMove(VoxelFace),
    SelectSecondCorner(VoxelFace),
}

pub fn initialize_selection_controller(mut commands: Commands) {
    commands.insert_resource(SelectionState::SelectingFirstCorner);
}

pub fn selection_default_input_map(
    mut events: EventWriter<SelectionEvents>,
    mut selection_state: ResMut<SelectionState>,
    voxel_cursor: VoxelCursor,
) {
    match &mut *selection_state {
        SelectionState::SelectingFirstCorner => {
            if let Some(first_corner) = voxel_cursor.voxel_just_clicked(MouseButton::Left) {
                events.send(SelectionEvents::SelectFirstCorner(first_corner));
            }
        }
        SelectionState::SelectingSecondCorner { valid_hover, .. } => {
            if let Some(hover_face) = voxel_cursor.impact.get_voxel_face() {
                if let Some(previous_hover) = valid_hover {
                    if hover_face != *previous_hover {
                        events.send(SelectionEvents::HoverMove(hover_face));
                    }
                    if voxel_cursor.voxel_just_clicked(MouseButton::Left).is_some() {
                        events.send(SelectionEvents::SelectSecondCorner(hover_face));
                    }
                }
            }
        }
        _ => {}
    }
}

pub fn selection_control_system(
    mut selection_state: ResMut<SelectionState>,
    mut events: EventReader<SelectionEvents>,
) {
    for event in events.iter() {
        match event {
            SelectionEvents::SelectFirstCorner(first_corner) => {
                *selection_state = SelectionState::SelectingSecondCorner {
                    first_corner: *first_corner,
                    valid_hover: Some(*first_corner),
                };
            }
            SelectionEvents::HoverMove(hover_face) => {
                if let SelectionState::SelectingSecondCorner { first_corner, .. } = *selection_state
                {
                    if selection_corners_are_compatible(&first_corner, &hover_face) {
                        *selection_state = SelectionState::SelectingSecondCorner {
                            first_corner,
                            valid_hover: Some(*hover_face),
                        };
                    }
                }
            }
            SelectionEvents::SelectSecondCorner(hover_face) => {
                if let SelectionState::SelectingSecondCorner { first_corner, .. } =
                    &mut *selection_state
                {
                    if selection_corners_are_compatible(first_corner, &hover_face) {
                        *selection_state = SelectionState::SelectionReady {
                            quad_extent: Extent3i::from_corners(
                                first_corner.point,
                                hover_face.point,
                            ),
                            normal: first_corner.normal,
                        };
                    } else {
                        *selection_state = SelectionState::SelectingFirstCorner;
                    }
                }
            }
        }
    }
}

fn selection_corners_are_compatible(corner1: &VoxelFace, corner2: &VoxelFace) -> bool {
    corner1.normal == corner2.normal
        && corner1.point.axis_component(corner1.normal.axis)
            == corner2.point.axis_component(corner2.normal.axis)
}
