use crate::StatePlugin;

use super::{
    ray_impact::voxel_cursor_impact_system,
    voxel_cursor::{voxel_clicking_system, VoxelCursorStates},
    VoxelCursorRayImpact,
};

use bevy::ecs::prelude::*;

/// Manages the `VoxelCursorRayImpact` and `VoxelCursorStates` resources.
pub struct VoxelPickingPlugin;

impl VoxelPickingPlugin {
    pub fn initialize(mut commands: Commands) {
        commands.insert_resource(VoxelCursorRayImpact::default());
        commands.insert_resource(VoxelCursorStates::default());
    }
}

impl StatePlugin for VoxelPickingPlugin {
    fn add_enter_systems(set: SystemSet) -> SystemSet {
        set.with_system(Self::initialize.system())
    }

    fn add_update_systems(set: SystemSet) -> SystemSet {
        set.with_system(voxel_cursor_impact_system.system())
            .with_system(voxel_clicking_system.system())
    }
}
