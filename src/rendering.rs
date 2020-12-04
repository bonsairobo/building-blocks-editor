pub mod render_graph;

mod array_material;
mod entity;
mod light;

pub use array_material::*;
pub use entity::*;
pub use light::*;

use render_graph::add_triplanar_render_graph;

pub mod prelude {
    pub use super::{array_material::ArrayMaterial, entity::*, light::Light};
}

use bevy::app::prelude::*;
use bevy::asset::AddAsset;
use bevy::reflect::RegisterTypeBuilder;
use bevy::render::{render_graph::RenderGraph, shader};

#[derive(Default)]
pub struct TriplanarRenderPlugin;

impl Plugin for TriplanarRenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<ArrayMaterial>()
            .register_type::<Light>()
            .add_system_to_stage(
                stage::POST_UPDATE,
                shader::asset_shader_defs_system::<ArrayMaterial>,
            )
            .init_resource::<AmbientLight>();
        let resources = app.resources();
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
        add_triplanar_render_graph(&mut render_graph, resources);
    }
}
