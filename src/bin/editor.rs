use building_blocks_editor::{
    BVTPlugin, CameraPlugin, CursorPositionPlugin, EditToolsPlugin, ImmediateModePlugin,
    LightBundle, SdfVoxelTypeInfo, SmoothMeshPlugin, TriplanarRenderPlugin, VoxelPickingPlugin,
    VOXEL_CHUNK_SHAPE,
};

use bevy::{ecs::IntoSystem, prelude::*};
use bevy_building_blocks::{
    default_chunk_map, ChunkCacheConfig, MapIoPlugin, VoxelEditor, VoxelMap, VoxelPalette,
};

fn main() {
    let mut window_desc = WindowDescriptor::default();
    window_desc.title = "Building Blocks Editor".to_string();

    let mut app_builder = App::build();
    app_builder
        .add_startup_system(initialize_editor.system())
        .add_startup_stage("init_voxels")
        .add_startup_system_to_stage("init_voxels", initialize_voxels.system())
        .add_resource(window_desc)
        .add_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(TriplanarRenderPlugin)
        .add_plugin(ImmediateModePlugin)
        .add_plugin(CursorPositionPlugin)
        .add_plugin(MapIoPlugin::<SdfVoxel>::new(
            VOXEL_CHUNK_SHAPE,
            ChunkCacheConfig::default(),
        ))
        .add_plugin(BVTPlugin::<SdfVoxel>::default())
        .add_plugin(SmoothMeshPlugin::<SdfVoxel>::new())
        .add_plugin(CameraPlugin)
        .add_plugin(VoxelPickingPlugin)
        .add_plugin(EditToolsPlugin);

    app_builder.run();
}

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

use building_blocks::prelude::*;
use building_blocks_editor::{SdfVoxel, SdfVoxelType, VoxelDistance};

fn initialize_voxels(mut voxel_editor: VoxelEditor<SdfVoxel>) {
    let write_extent = Extent3i::from_corners(PointN([0, 0, 0]), PointN([50, 50, 50]));
    voxel_editor.edit_extent_and_touch_neighbors(write_extent, |_p, voxel| {
        *voxel = SdfVoxel::new(SdfVoxelType(1), VoxelDistance::encode(-10.0));
    });
}
