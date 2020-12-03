use crate::rendering::{ArrayMaterial, TriplanarMeshBundle};

use bevy::{
    asset::prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        pipeline::PrimitiveTopology,
        prelude::*,
    },
};
use building_blocks::mesh::{OrientedCubeFace, PosNormMesh, UnorientedQuad};

pub fn create_voxel_pos_norm_mesh_bundle(
    mesh: PosNormMesh,
    material: Handle<ArrayMaterial>,
    meshes: &mut Assets<Mesh>,
) -> TriplanarMeshBundle {
    assert_eq!(mesh.positions.len(), mesh.normals.len());

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.set_attribute(
        "Vertex_Position",
        VertexAttributeValues::Float3(mesh.positions),
    );
    render_mesh.set_attribute("Vertex_Normal", VertexAttributeValues::Float3(mesh.normals));
    render_mesh.set_indices(Some(Indices::U32(mesh.indices)));

    TriplanarMeshBundle {
        mesh: meshes.add(render_mesh),
        material,
        ..Default::default()
    }
}

pub fn create_single_quad_mesh_bundle(
    face: &OrientedCubeFace,
    quad: &UnorientedQuad,
    material: Handle<ArrayMaterial>,
    meshes: &mut Assets<Mesh>,
) -> TriplanarMeshBundle {
    let mut mesh = PosNormMesh::default();
    face.add_quad_to_pos_norm_mesh(quad, &mut mesh);

    create_voxel_pos_norm_mesh_bundle(mesh, material, meshes)
}
