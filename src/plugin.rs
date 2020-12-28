use crate::{
    create_camera_entity, CameraControlConfig, CameraPlugin, CursorPositionPlugin, EditToolsPlugin,
    ImmediateModePlugin, SdfVoxelTypeInfo, VoxelMaterial, VoxelPickingPlugin, VOXEL_CHUNK_SHAPE,
};

use bevy::{
    app::prelude::*,
    asset::prelude::*,
    ecs::prelude::*,
    math::prelude::*,
    render::{prelude::*, texture::AddressMode},
    transform::components::Transform,
};
use bevy_building_blocks::{
    empty_compressible_chunk_map, BVTPlugin, ChunkCacheConfig, MapIoPlugin, VoxelEditor, VoxelMap,
    VoxelPalette,
};
use smooth_voxel_renderer::{
    ArrayMaterial, LightBundle, MeshGeneratorPlugin, MeshMaterial, SmoothVoxelRenderPlugin,
};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let app = app
            // Core stuff that always runs.
            .add_plugin(CursorPositionPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(ImmediateModePlugin)
            .add_plugin(SmoothVoxelRenderPlugin)
            // This plugin should run systems in the LAST stage.
            .add_plugin(MapIoPlugin::<SdfVoxel>::new(
                VOXEL_CHUNK_SHAPE,
                ChunkCacheConfig::default(),
            ));

        // Editor scheduling.
        add_editor_schedule(app);
    }
}

#[derive(Clone)]
enum EditorState {
    Loading,
    Editing,
}

mod stages {
    pub const EDITOR_STAGE: &str = "editor";
}

fn add_editor_schedule(app: &mut AppBuilder) {
    let mut editor_state_stage = StateStage::<EditorState>::default();
    editor_state_stage
        .on_state_enter(EditorState::Loading, start_loading.system())
        .on_state_update(EditorState::Loading, wait_for_assets_loaded.system())
        .on_state_enter(EditorState::Editing, initialize_editor.system())
        .on_state_enter(
            EditorState::Editing,
            MeshGeneratorPlugin::<SdfVoxel>::initialize,
        )
        .on_state_enter(EditorState::Editing, BVTPlugin::<SdfVoxel>::initialize)
        .on_state_enter(EditorState::Editing, VoxelPickingPlugin::initialize)
        .enter_stage(EditorState::Editing, |stage: &mut SystemStage| {
            EditToolsPlugin::initialize_in_stage(stage);

            stage
        })
        .update_stage(EditorState::Editing, |stage: &mut SystemStage| {
            MeshGeneratorPlugin::<SdfVoxel>::update_in_stage(stage);
            EditToolsPlugin::update_in_stage(stage);
            BVTPlugin::<SdfVoxel>::update_in_stage(stage);
            VoxelPickingPlugin::update_in_stage(stage);

            stage
        });

    app.add_resource(State::new(EditorState::Loading))
        .add_stage_after(stage::UPDATE, stages::EDITOR_STAGE, editor_state_stage);
}

struct LoadingTexture(Handle<Texture>);

fn start_loading(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        // TODO: load voxel map from file
        .insert_resource(VoxelMap {
            voxels: empty_compressible_chunk_map::<SdfVoxel>(VOXEL_CHUNK_SHAPE),
            palette: VoxelPalette {
                infos: vec![
                    SdfVoxelTypeInfo {
                        is_empty: true,
                        material: VoxelMaterial::NULL,
                    },
                    SdfVoxelTypeInfo {
                        is_empty: false,
                        material: VoxelMaterial(0),
                    },
                    SdfVoxelTypeInfo {
                        is_empty: false,
                        material: VoxelMaterial(1),
                    },
                    SdfVoxelTypeInfo {
                        is_empty: false,
                        material: VoxelMaterial(2),
                    },
                    SdfVoxelTypeInfo {
                        is_empty: false,
                        material: VoxelMaterial(3),
                    },
                ],
            },
        })
        .insert_resource(LoadingTexture(asset_server.load("albedo.png")));
}

fn wait_for_assets_loaded(
    commands: &mut Commands,
    loading_texture: Res<LoadingTexture>,
    mut array_materials: ResMut<Assets<ArrayMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut state: ResMut<State<EditorState>>,
) {
    if let Some(texture) = textures.get_mut(&loading_texture.0) {
        println!("Done loading mesh texture");
        prepare_materials_texture(texture);
        commands.insert_resource(MeshMaterial(
            array_materials.add(ArrayMaterial::from(loading_texture.0.clone())),
        ));
        state.set_next(EditorState::Editing).unwrap();
    }
}

fn prepare_materials_texture(texture: &mut Texture) {
    let num_layers = 4;
    texture.reinterpret_stacked_2d_as_array(num_layers);
    texture.sampler.address_mode_u = AddressMode::Repeat;
    texture.sampler.address_mode_v = AddressMode::Repeat;
}

use crate::{SdfVoxel, SdfVoxelType, VoxelDistance};
use building_blocks::prelude::*;

fn initialize_editor(commands: &mut Commands, mut voxel_editor: VoxelEditor<SdfVoxel>) {
    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.100, 150.0, 100.0)),
        ..Default::default()
    });

    // TODO: remove this once we can create voxels out of thin air
    println!("Initializing voxels");
    let write_extent = Extent3i::from_min_and_shape(PointN([0, 0, 0]), PointN([64, 64, 64]));
    voxel_editor.edit_extent_and_touch_neighbors(write_extent, |_p, voxel| {
        *voxel = SdfVoxel::new(SdfVoxelType(2), VoxelDistance::encode(-10.0));
    });

    initialize_camera(commands);
}

fn initialize_camera(commands: &mut Commands) {
    let eye = Vec3::new(100.0, 100.0, 100.0);
    let target = Vec3::new(0.0, 0.0, 0.0);
    let control_config = CameraControlConfig {
        mouse_rotate_sensitivity: 0.002,
        mouse_translate_sensitivity: 0.1,
        trackpad_translate_sensitivity: 0.1,
        smoothing_weight: 0.9,
    };
    create_camera_entity(commands, control_config, eye, target);
}
