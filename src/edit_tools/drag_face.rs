use super::{hover_hint::HoverHintMaterial, CurrentTool};

use crate::{
    geometry::{closest_points_on_two_lines, Ray3},
    mesh::create_single_quad_mesh_pbr_bundle,
    voxel::{offset_transform, ClickedVoxel, VoxelFace},
    CursorRay, ImmediateModeTag, VoxelCursorRayImpact,
};

use bevy::{
    asset::prelude::*, ecs::prelude::*, input::prelude::*, pbr::prelude::*, render::prelude::*,
};
use building_blocks::{
    core::{prelude::*, SignedAxis3},
    mesh::{OrientedCubeFace, UnorientedQuad},
};

#[derive(Clone, Copy)]
pub enum DragFaceTool {
    Start,
    FirstCornerSelected {
        face_selection: VoxelFace,
    },
    SecondCornerSelected {
        quad_extent: Extent3i,
        axis: SignedAxis3,
    },
    DraggingFace {
        quad_extent: Extent3i,
        axis: SignedAxis3,
        initial_drag_point: Point3f,
    },
}

impl Default for DragFaceTool {
    fn default() -> Self {
        DragFaceTool::Start
    }
}

pub fn drag_face_tool_system(
    commands: &mut Commands,
    mut current_tool: ResMut<CurrentTool>,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<HoverHintMaterial>,
    voxel_cursor_impact: Res<VoxelCursorRayImpact>,
    clicked_voxel: Res<ClickedVoxel>,
    cursor_ray: Res<CursorRay>,
) {
    if let CurrentTool::DragFace(tool) = &mut *current_tool {
        match *tool {
            DragFaceTool::Start => {
                if let Some(face_selection) =
                    clicked_voxel.for_button(MouseButton::Left).just_clicked
                {
                    *tool = DragFaceTool::FirstCornerSelected { face_selection };
                }
            }
            DragFaceTool::FirstCornerSelected {
                face_selection:
                    VoxelFace {
                        point: corner1,
                        face: face1,
                    },
            } => {
                let mut valid_selection = false;
                if let Some(VoxelFace {
                    point: corner2,
                    face: face2,
                }) = voxel_cursor_impact.get_voxel_face()
                {
                    if face1 == face2
                        && corner1.axis_component(face1.axis) == corner2.axis_component(face2.axis)
                    {
                        let face = OrientedCubeFace::canonical(face1);
                        let quad_extent = Extent3i::from_corners(corner1, corner2);
                        let quad = face.quad_from_extent(&quad_extent);

                        create_quad_selection_hint_entity(
                            &quad,
                            &face,
                            material.0.clone(),
                            commands,
                            &mut *meshes,
                        );

                        if clicked_voxel.just_clicked(MouseButton::Left) {
                            valid_selection = true;
                            *tool = DragFaceTool::SecondCornerSelected {
                                quad_extent,
                                axis: face1,
                            };
                        }
                    }
                }
                if !valid_selection && clicked_voxel.just_clicked(MouseButton::Left) {
                    *tool = DragFaceTool::Start;
                }
            }
            DragFaceTool::SecondCornerSelected { quad_extent, axis } => {
                let face = OrientedCubeFace::canonical(axis);
                let quad = face.quad_from_extent(&quad_extent);
                create_quad_selection_hint_entity(
                    &quad,
                    &face,
                    material.0.clone(),
                    commands,
                    &mut *meshes,
                );

                if let Some(voxel_face) = clicked_voxel.for_button(MouseButton::Left).just_pressed {
                    if quad_extent.contains(&voxel_face.point) {
                        *tool = DragFaceTool::DraggingFace {
                            quad_extent,
                            axis,
                            initial_drag_point: voxel_face.point.into(),
                        };
                    }
                }
            }
            DragFaceTool::DraggingFace {
                mut quad_extent,
                axis,
                initial_drag_point,
            } => {
                let face = OrientedCubeFace::canonical(axis);

                // Drag the quad using the cursor.
                if let CursorRay(Some(ray)) = &*cursor_ray {
                    let axis_line = Ray3::new(initial_drag_point.into(), face.mesh_normal().into());
                    if let Some((p1, _p2)) = closest_points_on_two_lines(&axis_line, ray) {
                        let new_axis_coord =
                            *Point3f::from(p1).in_voxel().axis_component(axis.axis);
                        *quad_extent.minimum.axis_component_mut(axis.axis) = new_axis_coord;

                        *tool = DragFaceTool::DraggingFace {
                            quad_extent,
                            axis,
                            initial_drag_point,
                        };
                    }
                }

                let quad = face.quad_from_extent(&quad_extent);
                create_quad_selection_hint_entity(
                    &quad,
                    &face,
                    material.0.clone(),
                    commands,
                    &mut *meshes,
                );
            }
        }
    }
}

// TODO: extract the quad selection workflow into its own system somehow (sub-workflow)

fn create_quad_selection_hint_entity(
    quad: &UnorientedQuad,
    face: &OrientedCubeFace,
    material: Handle<StandardMaterial>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
) -> Entity {
    commands
        .spawn(create_single_quad_mesh_pbr_bundle(
            &face, &quad, material, meshes,
        ))
        .with(offset_transform(face.mesh_normal() * 0.1))
        .with(ImmediateModeTag)
        .current_entity()
        .unwrap()
}
