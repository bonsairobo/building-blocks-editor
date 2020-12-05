use super::{array_material::ArrayMaterial, light::Light, render_graph::FORWARD_PIPELINE_HANDLE};

use bevy::asset::Handle;
use bevy::ecs::Bundle;
use bevy::render::{
    draw::Draw,
    mesh::Mesh,
    pipeline::{RenderPipeline, RenderPipelines},
    render_graph::base::MainPass,
};
use bevy::transform::prelude::{GlobalTransform, Transform};

/// A component bundle for "triplanar mesh" entities
#[derive(Bundle)]
pub struct TriplanarMeshBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ArrayMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for TriplanarMeshBundle {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                FORWARD_PIPELINE_HANDLE.typed(),
            )]),
            mesh: Default::default(),
            material: Default::default(),
            main_pass: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

/// A component bundle for "light" entities
#[derive(Debug, Bundle, Default)]
pub struct LightBundle {
    pub light: Light,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
