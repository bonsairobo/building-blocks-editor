use crate::{
    create_camera_entity, empty_compressible_sdf_chunk_map,
    voxel_renderer::{ArrayMaterial, MeshGeneratorPlugin, MeshMaterial, SmoothVoxelPbrPlugin},
    BVTPlugin, CameraControlConfig, CameraPlugin, ChunkCacheConfig, CursorPositionPlugin,
    EditToolsPlugin, ImmediateModePlugin, MapIoPlugin, SdfVoxelMap, SdfVoxelPalette, VoxelEditor,
    VoxelMaterial, VoxelPickingPlugin, VoxelTypeInfo, CHUNK_SHAPE,
};

use bevy::{
    app::{prelude::*, PluginGroupBuilder},
    asset::{prelude::*, AssetPlugin},
    core::CorePlugin,
    ecs::prelude::*,
    input::InputPlugin,
    math::prelude::*,
    pbr::{Light, LightBundle, PbrPlugin},
    render::{prelude::*, texture::AddressMode, RenderPlugin},
    transform::{components::Transform, TransformPlugin},
    wgpu::WgpuPlugin,
    window::WindowPlugin,
    winit::WinitPlugin,
};

/// The first-party plugins that we need from Bevy.
pub struct BevyPlugins;

impl PluginGroup for BevyPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(CorePlugin::default());
        group.add(TransformPlugin::default());
        group.add(InputPlugin::default());
        group.add(WindowPlugin::default());
        group.add(AssetPlugin::default());
        group.add(RenderPlugin::default());
        group.add(PbrPlugin::default());
        group.add(WinitPlugin::default());
        group.add(WgpuPlugin::default());
    }
}

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let app = app
            .add_startup_system(create_lights.system())
            // Core stuff that always runs.
            .add_plugin(CursorPositionPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(ImmediateModePlugin)
            .add_plugin(SmoothVoxelPbrPlugin)
            // This plugin should run systems in the LAST stage.
            .add_plugin(MapIoPlugin::new(CHUNK_SHAPE, ChunkCacheConfig::default()));

        // Editor scheduling.
        add_editor_schedule(app);
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum EditorState {
    Loading,
    Editing,
}

fn add_editor_schedule(app: &mut AppBuilder) {
    app.add_state(EditorState::Loading)
        // Load assets.
        .add_system_set(
            SystemSet::on_enter(EditorState::Loading).with_system(start_loading.system()),
        )
        .add_system_set(
            SystemSet::on_update(EditorState::Loading).with_system(wait_for_assets_loaded.system()),
        )
        // Initialize editor systems.
        .add_system_set(
            PluginAdder(SystemSet::on_enter(EditorState::Editing))
                .enter_with_plugin::<EditToolsPlugin>()
                .finish()
                .with_system(VoxelPickingPlugin::initialize.system())
                .with_system(MeshGeneratorPlugin::initialize.system())
                .with_system(BVTPlugin::initialize.system())
                .with_system(initialize_editor.system()),
        )
        // Update editor systems.
        .add_system_set(
            PluginAdder(SystemSet::on_update(EditorState::Editing))
                .update_with_plugin::<MeshGeneratorPlugin>()
                .update_with_plugin::<EditToolsPlugin>()
                .update_with_plugin::<BVTPlugin>()
                .update_with_plugin::<VoxelPickingPlugin>()
                .finish(),
        );
}

struct LoadingTexture(Handle<Texture>);

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        // TODO: load voxel map from file
        .insert_resource(SdfVoxelMap {
            voxels: empty_compressible_sdf_chunk_map(CHUNK_SHAPE),
            palette: SdfVoxelPalette {
                infos: vec![
                    VoxelTypeInfo {
                        is_empty: true,
                        material: VoxelMaterial::NULL,
                    },
                    VoxelTypeInfo {
                        is_empty: false,
                        material: VoxelMaterial(0),
                    },
                    VoxelTypeInfo {
                        is_empty: false,
                        material: VoxelMaterial(1),
                    },
                    VoxelTypeInfo {
                        is_empty: false,
                        material: VoxelMaterial(2),
                    },
                    VoxelTypeInfo {
                        is_empty: false,
                        material: VoxelMaterial(3),
                    },
                ],
            },
        });
    commands.insert_resource(LoadingTexture(asset_server.load("albedo.png")));
}

fn wait_for_assets_loaded(
    mut commands: Commands,
    loading_texture: Res<LoadingTexture>,
    mut array_materials: ResMut<Assets<ArrayMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut state: ResMut<State<EditorState>>,
) {
    if let Some(texture) = textures.get_mut(&loading_texture.0) {
        println!("Done loading mesh texture");
        prepare_materials_texture(texture);
        let mut material = ArrayMaterial::from(loading_texture.0.clone());
        material.roughness = 0.8;
        material.reflectance = 0.2;
        commands.insert_resource(MeshMaterial(array_materials.add(material)));
        state.set(EditorState::Editing).unwrap();
    }
}

fn prepare_materials_texture(texture: &mut Texture) {
    let num_layers = 4;
    texture.reinterpret_stacked_2d_as_array(num_layers);
    texture.sampler.address_mode_u = AddressMode::Repeat;
    texture.sampler.address_mode_v = AddressMode::Repeat;
}

use crate::VoxelType;
use building_blocks::prelude::*;

fn initialize_editor(mut commands: Commands, mut voxel_editor: VoxelEditor) {
    // TODO: remove this once we can create voxels out of thin air
    println!("Initializing voxels");
    let write_extent = Extent3i::from_min_and_shape(PointN([0, 0, 0]), PointN([64, 64, 64]));
    voxel_editor.edit_extent_and_touch_neighbors(write_extent, |_p, (voxel_type, dist)| {
        *voxel_type = VoxelType(2);
        *dist = Sd8::from(-10.0);
    });

    initialize_camera(&mut commands);
}

fn create_lights(mut commands: Commands) {
    for p in [
        Vec3::new(-100.0, 100.0, -100.0),
        Vec3::new(-100.0, 100.0, 100.0),
        Vec3::new(100.0, 100.0, -100.0),
        Vec3::new(100.0, 100.0, 100.0),
    ]
    .iter()
    {
        commands.spawn_bundle(LightBundle {
            transform: Transform::from_translation(*p),
            light: Light {
                intensity: 40000.0,
                range: 800.0,
                ..Default::default()
            },
            ..Default::default()
        });
    }
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

pub struct PluginAdder(pub SystemSet);

impl PluginAdder {
    pub fn enter_with_plugin<P: StatePlugin>(self) -> Self {
        Self(P::add_enter_systems(self.0))
    }

    pub fn update_with_plugin<P: StatePlugin>(self) -> Self {
        Self(P::add_update_systems(self.0))
    }

    pub fn exit_with_plugin<P: StatePlugin>(self) -> Self {
        Self(P::add_exit_systems(self.0))
    }

    pub fn finish(self) -> SystemSet {
        self.0
    }
}

pub trait StatePlugin {
    fn add_enter_systems(set: SystemSet) -> SystemSet {
        set
    }
    fn add_update_systems(set: SystemSet) -> SystemSet {
        set
    }
    fn add_exit_systems(set: SystemSet) -> SystemSet {
        set
    }
}
