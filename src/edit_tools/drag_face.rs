use super::{hover_hint::HoverHintMaterial, CurrentTool};

use crate::{
    geometry::{closest_points_on_two_lines, Ray3},
    mesh::create_single_quad_mesh_pbr_bundle,
    voxel::{offset_transform, ClickedVoxel, VoxelFace},
    CursorRay, ImmediateModeTag, SdfVoxel, SdfVoxelType, VoxelCursorRayImpact, VoxelDistance,
};

use bevy::{
    asset::prelude::*, ecs::prelude::*, input::prelude::*, pbr::prelude::*, render::prelude::*,
};
use bevy_building_blocks::VoxelEditor;
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
        normal: SignedAxis3,
    },
    DraggingFace {
        quad_extent: Extent3i,
        normal: SignedAxis3,
        previous_drag_point: Point3i,
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
    mut voxel_editor: VoxelEditor<SdfVoxel>,
    material: Res<HoverHintMaterial>,
    voxel_cursor_impact: Res<VoxelCursorRayImpact>,
    clicked_voxel: Res<ClickedVoxel>,
    cursor_ray: Res<CursorRay>,
) {
    if let CurrentTool::DragFace(tool) = &mut *current_tool {
        match *tool {
            DragFaceTool::Start => {
                if let Some(face_selection) = clicked_voxel
                    .for_button(MouseButton::Left)
                    .just_clicked_face
                {
                    *tool = DragFaceTool::FirstCornerSelected { face_selection };
                }
            }
            DragFaceTool::FirstCornerSelected {
                face_selection:
                    VoxelFace {
                        point: corner1,
                        normal: n1,
                    },
            } => {
                let mut valid_selection = false;
                if let Some(VoxelFace {
                    point: corner2,
                    normal: n2,
                }) = voxel_cursor_impact.get_voxel_face()
                {
                    if n1 == n2
                        && corner1.axis_component(n1.axis) == corner2.axis_component(n2.axis)
                    {
                        let face = OrientedCubeFace::canonical(n1);
                        let quad_extent = Extent3i::from_corners(corner1, corner2);
                        let quad = face.quad_from_extent(&quad_extent);

                        create_quad_selection_hint_entity(
                            &quad,
                            &face,
                            material.0.clone(),
                            commands,
                            &mut *meshes,
                        );

                        if clicked_voxel
                            .for_button(MouseButton::Left)
                            .just_clicked_face
                            .is_some()
                        {
                            valid_selection = true;
                            *tool = DragFaceTool::SecondCornerSelected {
                                quad_extent,
                                normal: n1,
                            };
                        }
                    }
                }
                if !valid_selection
                    && clicked_voxel
                        .for_button(MouseButton::Left)
                        .just_clicked_face
                        .is_some()
                {
                    *tool = DragFaceTool::Start;
                }
            }
            DragFaceTool::SecondCornerSelected {
                quad_extent,
                normal,
            } => {
                let face = OrientedCubeFace::canonical(normal);
                let quad = face.quad_from_extent(&quad_extent);
                create_quad_selection_hint_entity(
                    &quad,
                    &face,
                    material.0.clone(),
                    commands,
                    &mut *meshes,
                );

                if let Some(voxel_face) = clicked_voxel
                    .for_button(MouseButton::Left)
                    .just_pressed_face
                {
                    if quad_extent.contains(&voxel_face.point) {
                        *tool = DragFaceTool::DraggingFace {
                            quad_extent,
                            normal,
                            previous_drag_point: voxel_face.point,
                        };
                    }
                }
            }
            DragFaceTool::DraggingFace {
                mut quad_extent,
                normal,
                previous_drag_point,
            } => {
                let face = OrientedCubeFace::canonical(normal);

                // Drag the quad using the cursor.
                if let CursorRay(Some(ray)) = &*cursor_ray {
                    // To drag the quad along it's normal axis, we need to project the cursor ray
                    // onto that axis, which is equivalent to finding the two closest points on two
                    // lines.
                    let axis_line = Ray3::new(
                        Point3f::from(previous_drag_point).into(),
                        face.mesh_normal().into(),
                    );
                    if let Some((p1, _p2)) = closest_points_on_two_lines(&axis_line, ray) {
                        let old_quad_extent = quad_extent;

                        // Move the quad to a new position along the axis.
                        let new_drag_point = Point3f::from(p1).in_voxel();
                        let new_axis_coord = *new_drag_point.axis_component(normal.axis);
                        *quad_extent.minimum.axis_component_mut(normal.axis) = new_axis_coord;

                        let previous_axis_coord = *previous_drag_point.axis_component(normal.axis);
                        let write_voxel =
                            if new_axis_coord * normal.sign > previous_axis_coord * normal.sign {
                                // We're dragging in the direction of the normal, so we should write
                                // solid voxels.
                                SdfVoxel::new(SdfVoxelType(1), VoxelDistance::min())
                            } else {
                                // We're dragging in the opposite direction of the normal, so we
                                // should write empty voxels.
                                SdfVoxel::new(SdfVoxelType(0), VoxelDistance::max())
                            };

                        // Write voxels in the extent between the old and new quad.
                        let fill_min = quad_extent.minimum.meet(&old_quad_extent.minimum);
                        let fill_max = quad_extent.max().join(&old_quad_extent.max());
                        let fill_extent = Extent3i::from_min_and_max(fill_min, fill_max);
                        voxel_editor.edit_extent_and_touch_neighbors(fill_extent, |_p, voxel| {
                            *voxel = write_voxel;
                        });

                        if clicked_voxel.for_button(MouseButton::Left).just_released {
                            // Done dragging.
                            *tool = DragFaceTool::Start;
                        } else {
                            // Still dragging.
                            *tool = DragFaceTool::DraggingFace {
                                quad_extent,
                                normal,
                                previous_drag_point: new_drag_point,
                            };
                        }
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
