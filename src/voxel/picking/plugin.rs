use super::{
    ray_impact::voxel_cursor_impact_system,
    voxel_cursor::{voxel_clicking_system, VoxelCursorState},
    VoxelCursorRayImpact,
};

use bevy::{app::prelude::*, ecs::prelude::*};

/// Manages the `VoxelCursorRayImpact` and `VoxelCursorState` resources.
pub struct VoxelPickingPlugin;

impl Plugin for VoxelPickingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(VoxelCursorRayImpact::default())
            .add_resource(VoxelCursorState::default())
            .add_system(voxel_cursor_impact_system.system())
            .add_system(voxel_clicking_system.system());
    }
}
