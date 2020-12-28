use bevy_building_blocks::Voxel;

use building_blocks::{mesh::SignedDistance, storage::IsEmpty};
use serde::{Deserialize, Serialize};
use smooth_voxel_renderer::{MaterialLayer, MaterialVoxel};

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
    const MAX_DISTANCE: f32 = 4.0;
    const MIN_DISTANCE: f32 = -4.0;
    // Should be 1/32 sub-voxel precision.
    const SDF_PRECISION: f32 = (Self::MAX_DISTANCE - Self::MIN_DISTANCE) / 256.0;

    pub fn encode(distance: f32) -> Self {
        Self(
            (distance / Self::SDF_PRECISION)
                .min(std::i8::MAX as f32)
                .max(std::i8::MIN as f32) as i8,
        )
    }

    /// The inverse of `encode`.
    pub fn decode(self) -> f32 {
        self.0 as f32 * Self::SDF_PRECISION
    }

    pub fn min() -> Self {
        Self(std::i8::MIN)
    }

    pub fn max() -> Self {
        Self(std::i8::MAX)
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
    pub material: VoxelMaterial,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoxelMaterial(pub u8);

impl VoxelMaterial {
    pub const NULL: Self = Self(std::u8::MAX);
}

impl IsEmpty for &SdfVoxelTypeInfo {
    fn is_empty(&self) -> bool {
        self.is_empty
    }
}

impl MaterialVoxel for &SdfVoxelTypeInfo {
    fn material(&self) -> MaterialLayer {
        MaterialLayer(self.material.0)
    }
}
