use super::SelectionState;

use crate::{voxel::offset_transform, ImmediateModeTag, VoxelCursorRayImpact};

use bevy::{
    asset::prelude::*,
    ecs::prelude::*,
    pbr::prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        pipeline::PrimitiveTopology,
        prelude::*,
    },
};
use building_blocks::mesh::{OrientedCubeFace, PosNormMesh, UnorientedQuad};

pub fn initialize_selection_view(
    commands: &mut Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut color = Color::YELLOW;
    color.set_a(0.5);
    let material = SelectionCursorMaterial(materials.add(StandardMaterial::from(color)));
    commands.insert_resource(material);
}

pub fn selection_view_system(
    selection_state: Res<SelectionState>,
    cursor_voxel: Res<VoxelCursorRayImpact>,
    material: Res<SelectionCursorMaterial>,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut quad_face = None;
    match *selection_state {
        SelectionState::SelectingFirstCorner => {
            if let Some(voxel_face) = cursor_voxel.get_voxel_face() {
                let quad = UnorientedQuad::from_voxel(voxel_face.point);
                let face = OrientedCubeFace::canonical(voxel_face.normal);
                quad_face = Some((quad, face));
            }
        }
        SelectionState::SelectingSecondCorner {
            first_corner,
            valid_hover,
        } => {
            if let Some(hover_face) = valid_hover {
                let face = OrientedCubeFace::canonical(first_corner.normal);
                let quad = face.quad_from_corners(first_corner.point, hover_face.point);
                quad_face = Some((quad, face));
            } else if let Some(voxel_face) = cursor_voxel.get_voxel_face() {
                let quad = UnorientedQuad::from_voxel(voxel_face.point);
                let face = OrientedCubeFace::canonical(voxel_face.normal);
                quad_face = Some((quad, face));
            }
        }
        SelectionState::SelectionReady {
            quad_extent,
            normal,
        } => {
            let face = OrientedCubeFace::canonical(normal);
            let quad = face.quad_from_extent(&quad_extent);
            quad_face = Some((quad, face));
        }
        SelectionState::Invisible => (),
    }

    if let Some((quad, face)) = quad_face {
        create_quad_selection_hint_entity(&quad, &face, material.0.clone(), commands, &mut *meshes);
    }
}

pub struct SelectionCursorMaterial(pub Handle<StandardMaterial>);

fn create_quad_selection_hint_entity(
    quad: &UnorientedQuad,
    face: &OrientedCubeFace,
    material: Handle<StandardMaterial>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
) -> Entity {
    commands
        .spawn(create_single_quad_mesh_bundle(
            &face, &quad, material, meshes,
        ))
        .with(offset_transform(face.mesh_normal() * HOVER_DISTANCE))
        .with(ImmediateModeTag)
        .current_entity()
        .unwrap()
}

const HOVER_DISTANCE: f32 = 0.2;

fn create_single_quad_mesh_bundle(
    face: &OrientedCubeFace,
    quad: &UnorientedQuad,
    material: Handle<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
) -> PbrBundle {
    let mut mesh = PosNormMesh::default();
    face.add_quad_to_pos_norm_mesh(quad, &mut mesh);

    let num_vertices = mesh.positions.len();

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.set_attribute(
        "Vertex_Position",
        VertexAttributeValues::Float3(mesh.positions),
    );
    render_mesh.set_attribute("Vertex_Normal", VertexAttributeValues::Float3(mesh.normals));
    // HACK: We have to provide UVs, even though we don't use them.
    render_mesh.set_attribute(
        "Vertex_Uv",
        VertexAttributeValues::Float2(vec![[0.0; 2]; num_vertices]),
    );
    render_mesh.set_indices(Some(Indices::U32(mesh.indices)));

    PbrBundle {
        mesh: meshes.add(render_mesh),
        material,
        ..Default::default()
    }
}
