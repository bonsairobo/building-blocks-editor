pub mod render_graph;

mod entity;
mod light;
mod material;
mod mesh_generator;

pub use entity::*;
pub use light::*;
pub use material::*;
pub use mesh_generator::*;

use bevy::app::prelude::*;
use bevy::asset::{AddAsset, Assets, Handle};
use bevy::ecs::system::IntoSystem;
use bevy::render::{prelude::Color, shader};
use render_graph::add_pbr_graph;

/// NOTE: this isn't PBR yet. consider this name "aspirational" :)
#[derive(Default)]
pub struct SmoothVoxelPbrPlugin;

impl Plugin for SmoothVoxelPbrPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<ArrayMaterial>()
            .register_type::<Light>()
            .add_system_to_stage(
                CoreStage::PostUpdate,
                shader::asset_shader_defs_system::<ArrayMaterial>.system(),
            )
            .init_resource::<AmbientLight>();
        add_pbr_graph(app.world_mut());

        // add default ArrayMaterial
        let mut materials = app
            .world_mut()
            .get_resource_mut::<Assets<ArrayMaterial>>()
            .unwrap();
        materials.set_untracked(
            Handle::<ArrayMaterial>::default(),
            ArrayMaterial {
                base_color: Color::PINK,
                unlit: true,
                ..Default::default()
            },
        );
    }
}
