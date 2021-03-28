pub mod render_graph;

mod entity;
mod material;
mod mesh_generator;

pub use entity::*;
pub use material::*;
pub use mesh_generator::*;

use render_graph::add_voxel_render_graph;

use bevy::app::prelude::*;
use bevy::asset::{AddAsset, Assets, Handle};
use bevy::ecs::system::IntoSystem;
use bevy::render::{prelude::Color, shader};

#[derive(Default)]
pub struct VoxelRenderPlugin;

impl Plugin for VoxelRenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<ArrayMaterial>().add_system_to_stage(
            CoreStage::PostUpdate,
            shader::asset_shader_defs_system::<ArrayMaterial>.system(),
        );
        add_voxel_render_graph(app.world_mut());

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
