mod drag_face;
mod edit_timeline;
mod plugin;
mod selection;
mod undo;

pub use plugin::EditToolsPlugin;

use drag_face::DragFaceState;

pub enum CurrentTool {
    DragFace(DragFaceState),
    PaintMaterial, // TODO
    Tile,          // TODO: tile the current buffer by dragging; replaces DragFace
    Slope,         // TODO: select two edges to slope between
    Brush,         // TODO: organic brushes
}

// TODO: 3D selection; like the drag face tool, but you drag to size the 3rd dimension of the
// selection. Move the selection by dragging a face. Allow visibility masking so you can only see
// the voxels in the selection.

// TODO: copy current selection to buffer

// TODO: render SDF

// TODO: smart tools; given some map palette and constraints, you can carve out section of map, and
// tiles get placed automatically, e.g. walls, floors, doors, and stairs are detected
