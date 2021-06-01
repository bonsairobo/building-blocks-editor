use super::{VoxelCursorRayImpact, VoxelFace};

use bevy::{
    ecs::{prelude::*, system::SystemParam},
    input::prelude::*,
};

#[derive(Default)]
pub struct VoxelCursorStates {
    left_states: VoxelCursorButtonState,
    middle_states: VoxelCursorButtonState,
    right_states: VoxelCursorButtonState,
}

/// A set of convenience methods that combine state about the `VoxelCursorRayImpact` and `Input<MouseButton>`. Relies on some
/// memory kept by the `voxel_clicking_system`.
#[derive(SystemParam)]
pub struct VoxelCursor<'a> {
    pub impact: Res<'a, VoxelCursorRayImpact>,
    pub mouse_input: Res<'a, Input<MouseButton>>,
    state: Res<'a, VoxelCursorStates>,
}

#[derive(Default)]
struct VoxelCursorButtonState {
    /// If the mouse is pressed, this is the first voxel face that was pressed since the last release.
    pub press_start_face: Option<VoxelFace>,
}

impl<'a> VoxelCursor<'a> {
    /// The voxel face currently pressed by the mouse.
    pub fn voxel_pressed(&self, button: MouseButton) -> Option<VoxelFace> {
        if self.mouse_input.pressed(button) {
            self.voxel_face()
        } else {
            None
        }
    }

    /// If the mouse was just pressed, this is the voxel where it was pressed.
    pub fn voxel_just_pressed(&self, button: MouseButton) -> Option<VoxelFace> {
        if self.mouse_input.just_pressed(button) {
            self.voxel_face()
        } else {
            None
        }
    }

    /// If the mouse was just released, this is the voxel where it was released.
    pub fn voxel_just_released(&self, button: MouseButton) -> Option<VoxelFace> {
        if self.mouse_input.just_released(button) {
            self.voxel_face()
        } else {
            None
        }
    }

    /// If the mouse is pressed, this is the first voxel face that was pressed since the last release.
    pub fn press_start_face(&self, button: MouseButton) -> Option<VoxelFace> {
        self.state_for_button(button).press_start_face
    }

    /// If the mouse was just released on the same voxel as `press_start`, then this is that voxel face.
    pub fn voxel_just_clicked(&self, button: MouseButton) -> Option<VoxelFace> {
        let just_released = self.voxel_just_released(button);
        if just_released == self.state_for_button(button).press_start_face {
            just_released
        } else {
            None
        }
    }

    /// The voxel face that the cursor is currently on.
    pub fn voxel_face(&self) -> Option<VoxelFace> {
        self.impact.get_voxel_face()
    }

    fn state_for_button(&self, button: MouseButton) -> &VoxelCursorButtonState {
        match button {
            MouseButton::Left => &self.state.left_states,
            MouseButton::Middle => &self.state.middle_states,
            MouseButton::Right => &self.state.right_states,
            x => panic!("Button {:?} not supported", x),
        }
    }
}

/// Remembers which voxel the cursor was on when mouse buttons were pressed.
pub fn voxel_clicking_system(
    voxel_cursor_impact: Res<VoxelCursorRayImpact>,
    mouse_input: Res<Input<MouseButton>>,
    mut state: ResMut<VoxelCursorStates>,
) {
    for button in [MouseButton::Left, MouseButton::Middle, MouseButton::Right]
        .iter()
        .cloned()
    {
        let state = match button {
            MouseButton::Left => &mut state.left_states,
            MouseButton::Middle => &mut state.middle_states,
            MouseButton::Right => &mut state.right_states,
            x => panic!("Button {:?} not supported", x),
        };

        if let Some((impact, normal)) = voxel_cursor_impact.get() {
            if mouse_input.just_pressed(button) {
                state.press_start_face = Some(VoxelFace {
                    point: impact.point,
                    normal: *normal,
                });
            }
        }
    }
}
