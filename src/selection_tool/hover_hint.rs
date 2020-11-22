use crate::{mesh::create_pos_norm_mesh_pbr_bundle, VoxelCursorRayImpact};

use bevy::{
    app::prelude::*, asset::prelude::*, ecs::prelude::*, pbr::prelude::*, render::prelude::*,
};
use building_blocks::{
    core::axis::SignedAxis3,
    mesh::{OrientedCubeFace, PosNormMesh, Quad},
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
    hover_hints: Query<(Entity, &HoverHintTag)>,
) {
    if let Some((hint_entity, _)) = hover_hints.iter().next() {
        // Delete the old hint.
        commands.despawn(hint_entity);
    }

    if let Some(impact) = &cursor_voxel.maybe_impact {
        let normal: mint::Vector3<f32> = impact.impact.normal.normalize().into();
        let normal = Point3f::from(normal).round().in_voxel();
        if let Some(normal_axis) = SignedAxis3::from_vector(normal) {
            create_hover_hint_entity(
                impact.point,
                normal_axis,
                material.0.clone(),
                commands,
                &mut *meshes,
            );
        }
    }
}

pub struct HoverHintTag;

pub struct HoverHintMaterial(pub Handle<StandardMaterial>);

fn create_hover_hint_entity(
    voxel_point: Point3i,
    normal_axis: SignedAxis3,
    material: Handle<StandardMaterial>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
) -> Entity {
    let quad = Quad::for_voxel_face(voxel_point, normal_axis);
    let face = OrientedCubeFace::canonical(normal_axis);
    let mut mesh = PosNormMesh::default();
    face.add_quad_to_pos_norm_mesh(&quad, &mut mesh);

    commands
        .spawn(create_pos_norm_mesh_pbr_bundle(mesh, material, meshes))
        .with(HoverHintTag)
        .current_entity()
        .unwrap()
}
