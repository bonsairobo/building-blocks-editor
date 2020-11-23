use bevy::transform::components::Transform;
use building_blocks::core::{prelude::*, SignedAxis3};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VoxelFace {
    pub point: Point3i,
    pub face: SignedAxis3,
}

pub fn offset_transform(offset: Point3f) -> Transform {
    Transform::from_translation(offset.into())
}
