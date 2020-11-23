use crate::{
    mesh::create_single_quad_mesh_pbr_bundle, voxel::offset_transform, ImmediateModeTag,
    VoxelCursorRayImpact,
};

use bevy::{
    app::prelude::*, asset::prelude::*, ecs::prelude::*, pbr::prelude::*, render::prelude::*,
};
use building_blocks::{
    core::axis::SignedAxis3,
    mesh::{OrientedCubeFace, UnorientedQuad},
    prelude::*,
};

pub struct HoverHintPlugin;

impl Plugin for HoverHintPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(initialize_hover_hint.system())
            .add_system(hover_hint_system.system());
    }
}

fn initialize_hover_hint(commands: &mut Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut color = Color::BLUE;
    color.set_a(0.5);
    let material = HoverHintMaterial(materials.add(StandardMaterial::from(color)));
    commands.insert_resource(material);
}

fn hover_hint_system(
    cursor_voxel: Res<VoxelCursorRayImpact>,
    material: Res<HoverHintMaterial>,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if let Some((impact, normal_axis)) = cursor_voxel.get() {
        create_hover_hint_entity(
            impact.point,
            *normal_axis,
            material.0.clone(),
            commands,
            &mut *meshes,
        );
    }
}

pub struct HoverHintMaterial(pub Handle<StandardMaterial>);

fn create_hover_hint_entity(
    voxel_point: Point3i,
    normal_axis: SignedAxis3,
    material: Handle<StandardMaterial>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
) -> Entity {
    let quad = UnorientedQuad::from_voxel(voxel_point);
    let face = OrientedCubeFace::canonical(normal_axis);

    commands
        .spawn(create_single_quad_mesh_pbr_bundle(
            &face, &quad, material, meshes,
        ))
        .with(offset_transform(face.mesh_normal() * 0.1))
        .with(ImmediateModeTag)
        .current_entity()
        .unwrap()
}
