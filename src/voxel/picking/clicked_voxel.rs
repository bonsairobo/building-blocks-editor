use super::VoxelCursorRayImpact;

use crate::voxel::VoxelFace;

use bevy::{ecs::prelude::*, input::prelude::*};

/// Stores states about how the mouse is interacting with `VoxelFace`s.
#[derive(Default)]
pub struct ClickedVoxel {
    left_states: MouseButtonClickedVoxel,
    middle_states: MouseButtonClickedVoxel,
    right_states: MouseButtonClickedVoxel,
}

#[derive(Default)]
pub struct MouseButtonClickedVoxel {
    /// The voxel face currently pressed by the mouse.
    pub pressed: Option<VoxelFace>,
    /// If the mouse is pressed, this is the first voxel face that was pressed since the last
    /// release.
    pub press_start: Option<VoxelFace>,
    /// If the mouse was just pressed, this is the voxel where it was pressed.
    pub just_pressed: Option<VoxelFace>,
    /// If the mouse was just released, this is the voxel where it was released.
    pub just_released: Option<VoxelFace>,
    /// If the mouse was just released on the same voxel as `press_start`, then this is that voxel
    /// face.
    pub just_clicked: Option<VoxelFace>,
}

impl ClickedVoxel {
    /// Get the `MouseButtonClickedVoxel` for `button`.
    pub fn for_button(&self, button: MouseButton) -> &MouseButtonClickedVoxel {
        match button {
            MouseButton::Left => &self.left_states,
            MouseButton::Middle => &self.middle_states,
            MouseButton::Right => &self.right_states,
            x => panic!("Button {:?} not supported", x),
        }
    }

    fn for_button_mut(&mut self, button: MouseButton) -> &mut MouseButtonClickedVoxel {
        match button {
            MouseButton::Left => &mut self.left_states,
            MouseButton::Middle => &mut self.middle_states,
            MouseButton::Right => &mut self.right_states,
            x => panic!("Button {:?} not supported", x),
        }
    }

    fn update_for_button(
        &mut self,
        button: MouseButton,
        voxel_cursor_impact: &VoxelCursorRayImpact,
        mouse_input: &Input<MouseButton>,
    ) {
        let states = self.for_button_mut(button);

        states.pressed = None;
        states.just_pressed = None;
        states.just_released = None;
        states.just_clicked = None;

        if let Some((impact, normal)) = voxel_cursor_impact.get() {
            let voxel_face = VoxelFace {
                point: impact.point,
                normal: *normal,
            };

            if mouse_input.just_pressed(MouseButton::Left) {
                states.just_pressed = Some(voxel_face);
                states.press_start = Some(voxel_face);
            }
            if mouse_input.just_released(MouseButton::Left) {
                states.just_released = Some(voxel_face);

                if states.press_start == Some(voxel_face) {
                    states.just_clicked = Some(voxel_face);
                }
            }

            if mouse_input.pressed(MouseButton::Left) {
                states.pressed = Some(voxel_face);
            } else {
                states.press_start = None;
            }
        }
    }

    // Following methods are for convenience, since systems shouldn't use both `Input<MouseButton>`
    // and `ClickedVoxel`, otherwise they risk inconsistent event ordering.

    pub fn just_clicked(&self, button: MouseButton) -> bool {
        self.for_button(button).just_clicked.is_some()
    }

    pub fn just_released(&self, button: MouseButton) -> bool {
        self.for_button(button).just_released.is_some()
    }

    pub fn just_pressed(&self, button: MouseButton) -> bool {
        self.for_button(button).just_pressed.is_some()
    }

    pub fn pressed(&self, button: MouseButton) -> bool {
        self.for_button(button).pressed.is_some()
    }
}

pub fn clicked_voxel_system(
    voxel_cursor_impact: Res<VoxelCursorRayImpact>,
    mouse_input: Res<Input<MouseButton>>,
    mut clicked_voxel: ResMut<ClickedVoxel>,
) {
    for button in [MouseButton::Left, MouseButton::Middle, MouseButton::Right].iter() {
        clicked_voxel.update_for_button(*button, &*voxel_cursor_impact, &*mouse_input);
    }
}
