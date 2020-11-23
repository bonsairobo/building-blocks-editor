use super::{
    clicked_voxel::clicked_voxel_system, cursor_impact::voxel_cursor_impact_system, ClickedVoxel,
    VoxelCursorRayImpact,
};

use bevy::{app::prelude::*, ecs::prelude::*};

/// Manages the `VoxelCursorRayImpact` and `ClickedVoxel` resources.
pub struct VoxelPickingPlugin;

impl Plugin for VoxelPickingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(VoxelCursorRayImpact::default())
            .add_resource(ClickedVoxel::default())
            .add_system(voxel_cursor_impact_system.system())
            .add_system(clicked_voxel_system.system());
    }
}
