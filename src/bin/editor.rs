use building_blocks_editor::{
    BVTPlugin, CameraPlugin, SdfVoxelTypeInfo, SmoothMeshPlugin, VoxelPickingPlugin,
};

use bevy::{ecs::IntoSystem, prelude::*, window::WindowMode};
use bevy_building_blocks::{
    default_chunk_map, ChunkCacheConfig, MapIoPlugin, VoxelEditor, VoxelMap, VoxelPalette,
};

fn main() {
    let mut window_desc = WindowDescriptor::default();
    // window_desc.mode = WindowMode::BorderlessFullscreen;
    window_desc.title = "Building Blocks Editor".to_string();

    let mut app_builder = App::build();
    app_builder
        .add_startup_system(initialize_editor.system())
        .add_resource(window_desc)
        .add_plugins(DefaultPlugins)
        .add_plugin(MapIoPlugin::<SdfVoxel>::new(
            VOXEL_CHUNK_SHAPE,
            ChunkCacheConfig::default(),
        ))
        .add_plugin(BVTPlugin::<SdfVoxel>::default())
        .add_plugin(SmoothMeshPlugin::<SdfVoxel>::new())
        .add_system(example_system.system())
        .add_plugin(VoxelPickingPlugin)
        .add_plugin(CameraPlugin)
        .add_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)));

    app_builder.run();
}

const VOXEL_CHUNK_SHAPE: Point3i = PointN([16; 3]);

fn initialize_editor(commands: &mut Commands) {
    // TODO: load from file
    commands.insert_resource(VoxelMap {
        voxels: default_chunk_map::<SdfVoxel>(VOXEL_CHUNK_SHAPE),
        palette: VoxelPalette {
            infos: vec![
                SdfVoxelTypeInfo { is_empty: true },
                SdfVoxelTypeInfo { is_empty: false },
            ],
        },
    });

    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.100, 150.0, 100.0)),
        ..Default::default()
    });
}

use bevy::ecs;
use building_blocks::prelude::*;
use building_blocks_editor::{SdfVoxel, SdfVoxelType, VoxelDistance, EMPTY_SDF_VOXEL};

fn example_system(
    mut voxel_editor: VoxelEditor<SdfVoxel>,
    // time: Res<Time>,
    // mut prev_extent: ecs::Local<PreviousExtent>,
) {
    // First clear the previous extent.
    // if let Some(prev_extent) = prev_extent.0.take() {
    //     voxel_editor.edit_extent_and_touch_neighbors(prev_extent, |_p, voxel| {
    //         *voxel = EMPTY_SDF_VOXEL;
    //     });
    // }

    // Write some extent that changes with time.
    // let s = 2.0 * time.seconds_since_startup as f32;
    let s = 3.14f32 / 4.0;
    let circle_point = PointN([50.0 * s.cos(), 50.0, 50.0 * s.sin()]).in_voxel();
    let write_extent = Extent3i::from_two_corners(PointN([0, 0, 0]), circle_point);
    voxel_editor.edit_extent_and_touch_neighbors(write_extent, |_p, voxel| {
        *voxel = SdfVoxel::new(SdfVoxelType(1), VoxelDistance::encode(-10.0));
    });

    // *prev_extent = PreviousExtent(Some(write_extent));
}

#[derive(Default)]
struct PreviousExtent(Option<Extent3i>);
