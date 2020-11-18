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
use building_blocks::mesh::PosNormMesh;

pub fn create_pos_norm_mesh_entity(
    mesh: PosNormMesh,
    material: Handle<StandardMaterial>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
) -> Entity {
    assert_eq!(mesh.positions.len(), mesh.normals.len());
    let num_vertices = mesh.positions.len();

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.set_attribute(
        "Vertex_Position",
        VertexAttributeValues::Float3(mesh.positions),
    );
    render_mesh.set_attribute("Vertex_Normal", VertexAttributeValues::Float3(mesh.normals));
    render_mesh.set_attribute(
        "Vertex_Uv",
        // TODO: real texturing
        VertexAttributeValues::Float2(vec![[0.0; 2]; num_vertices]),
    );
    render_mesh.set_indices(Some(Indices::U32(mesh.indices)));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(render_mesh),
            material,
            ..Default::default()
        })
        .current_entity()
        .unwrap()
}
