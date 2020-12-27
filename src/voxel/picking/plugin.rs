use super::{
    ray_impact::voxel_cursor_impact_system,
    voxel_cursor::{voxel_clicking_system, VoxelCursorState},
    VoxelCursorRayImpact,
};

use bevy::ecs::prelude::*;

/// Manages the `VoxelCursorRayImpact` and `VoxelCursorState` resources.
pub struct VoxelPickingPlugin;

impl VoxelPickingPlugin {
    pub fn initialize(commands: &mut Commands) {
        commands
            .insert_resource(VoxelCursorRayImpact::default())
            .insert_resource(VoxelCursorState::default());
    }

    pub fn update_in_stage(stage: &mut SystemStage) {
        stage
            .add_system(voxel_cursor_impact_system.system())
            .add_system(voxel_clicking_system.system());
    }
}
