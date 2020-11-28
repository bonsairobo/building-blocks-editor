mod drag_face;
mod plugin;
mod selection;

pub use plugin::EditToolsPlugin;

use drag_face::DragFaceState;

pub enum CurrentTool {
    Erase, // TODO: drag the selection to erase
    DragFace(DragFaceState),
    PaintMaterial, // TODO
    Tile,          // TODO: tile the current buffer by dragging; replaces DragFace
    Brush,         // TODO: organic brushes
}

// TODO: undo/redo history

// TODO: 3D selection; like the drag face tool, but you drag to size the 3rd dimension of the
// selection. Move the selection by dragging a face. Allow visibility masking so you can only see
// the voxels in the selection.

// TODO: copy current selection to buffer

// TODO: render SDF

// TODO: smart tools; given some map palette and constraints, you can carve out section of map, and
// tiles get placed automatically, e.g. walls, floors, doors, and stairs are detected
