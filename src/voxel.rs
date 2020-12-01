mod chunk_processing;
mod geometry;
mod picking;
mod sdf_voxel;

// Systems that facilitate voxel post-processing.
pub use chunk_processing::{BVTPlugin, SmoothMeshPlugin, VoxelBVT};
pub use geometry::{offset_transform, VoxelFace};
pub use picking::{VoxelCursor, VoxelCursorRayImpact, VoxelPickingPlugin};
pub use sdf_voxel::{SdfVoxel, SdfVoxelType, SdfVoxelTypeInfo, VoxelDistance, EMPTY_SDF_VOXEL};

use building_blocks::core::{Point3i, PointN};

pub const VOXEL_CHUNK_SHAPE: Point3i = PointN([16; 3]);
