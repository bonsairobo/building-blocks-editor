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
    pub pressed_face: Option<VoxelFace>,
    /// If the mouse is pressed, this is the first voxel face that was pressed since the last
    /// release.
    pub press_start_face: Option<VoxelFace>,
    /// If the mouse was just pressed, this is the voxel where it was pressed.
    pub just_pressed_face: Option<VoxelFace>,
    /// If the mouse was just released, this is the voxel where it was released.
    pub just_released_face: Option<VoxelFace>,
    /// If the mouse was just released on the same voxel as `press_start`, then this is that voxel
    /// face.
    pub just_clicked_face: Option<VoxelFace>,

    // These are the same as the fields on `Input<MouseButton>`, but they're time-consistent with
    // the rest of the states on this struct.
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
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

        states.pressed = mouse_input.pressed(button);
        states.just_pressed = mouse_input.just_pressed(button);
        states.just_released = mouse_input.just_released(button);

        states.pressed_face = None;
        states.just_pressed_face = None;
        states.just_released_face = None;
        states.just_clicked_face = None;

        if let Some((impact, normal)) = voxel_cursor_impact.get() {
            let voxel_face = VoxelFace {
                point: impact.point,
                normal: *normal,
            };

            if mouse_input.just_pressed(MouseButton::Left) {
                states.just_pressed_face = Some(voxel_face);
                states.press_start_face = Some(voxel_face);
            }
            if mouse_input.just_released(MouseButton::Left) {
                states.just_released_face = Some(voxel_face);

                if states.press_start_face == Some(voxel_face) {
                    states.just_clicked_face = Some(voxel_face);
                }
            }

            if mouse_input.pressed(MouseButton::Left) {
                states.pressed_face = Some(voxel_face);
            } else {
                states.press_start_face = None;
            }
        }
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
