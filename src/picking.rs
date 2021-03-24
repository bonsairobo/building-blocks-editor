mod plugin;
mod ray_impact;
mod voxel_cursor;

pub use plugin::VoxelPickingPlugin;
pub use ray_impact::VoxelCursorRayImpact;
pub use voxel_cursor::VoxelCursor;

use building_blocks::core::{Point3i, SignedAxis3};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VoxelFace {
    pub point: Point3i,
    pub normal: SignedAxis3,
}
