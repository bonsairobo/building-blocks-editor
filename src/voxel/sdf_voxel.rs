use bevy_building_blocks::Voxel;

use building_blocks::{
    mesh::{MaterialVoxel, SignedDistance},
    storage::IsEmpty,
};
use serde::{Deserialize, Serialize};

/// The data actually stored in each point of the voxel map.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SdfVoxel {
    pub voxel_type: SdfVoxelType,
    pub distance: VoxelDistance,
}

impl SdfVoxel {
    pub fn new(voxel_type: SdfVoxelType, distance: VoxelDistance) -> Self {
        Self {
            voxel_type,
            distance,
        }
    }
}

impl Voxel for SdfVoxel {
    type TypeInfo = SdfVoxelTypeInfo;

    fn get_type_index(&self) -> usize {
        self.voxel_type.0 as usize
    }
}

impl Default for SdfVoxel {
    fn default() -> Self {
        EMPTY_SDF_VOXEL
    }
}

/// Identifies the type of voxel.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SdfVoxelType(pub u8);

/// Quantized distance from an isosurface.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoxelDistance(pub i8);

impl VoxelDistance {
    // This is mostly just experimental. I don't have a good rationale for this value.
    const SDF_QUANTIZE_FACTOR: f32 = 50.0;

    pub fn encode(distance: f32) -> Self {
        Self(
            (distance * Self::SDF_QUANTIZE_FACTOR)
                .min(std::i8::MAX as f32)
                .max(std::i8::MIN as f32) as i8,
        )
    }

    /// The inverse of `encode`.
    pub fn decode(self: Self) -> f32 {
        self.0 as f32 / Self::SDF_QUANTIZE_FACTOR
    }
}

impl SignedDistance for SdfVoxel {
    fn distance(&self) -> f32 {
        self.distance.decode()
    }
}

pub const EMPTY_SDF_VOXEL: SdfVoxel = SdfVoxel {
    voxel_type: SdfVoxelType(0),
    distance: VoxelDistance(std::i8::MAX),
};

/// Metadata about a specific type of voxel.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SdfVoxelTypeInfo {
    pub is_empty: bool,
}

impl IsEmpty for &SdfVoxelTypeInfo {
    fn is_empty(&self) -> bool {
        self.is_empty
    }
}

impl MaterialVoxel for &SdfVoxelTypeInfo {
    type Material = ();
    fn material(&self) -> Self::Material {}
}
