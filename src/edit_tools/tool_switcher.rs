use super::{CurrentTool, DragFaceState};

use bevy::{ecs::prelude::*, input::prelude::*};

pub fn tool_switcher_system(keyboard: Res<Input<KeyCode>>, mut current_tool: ResMut<CurrentTool>) {
    if keyboard.just_pressed(KeyCode::D) {
        println!("Switching to DragFace tool");
        *current_tool = CurrentTool::DragFace(DragFaceState::SelectionReady);
    } else if keyboard.just_pressed(KeyCode::T) {
        println!("Switching to Terraformer tool");
        *current_tool = CurrentTool::Terraform;
    }
}
