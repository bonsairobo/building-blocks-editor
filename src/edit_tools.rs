mod drag_face;
mod plugin;
mod selection;

pub use plugin::EditToolsPlugin;

use drag_face::DragFaceState;

pub enum CurrentTool {
    Copy, // TODO
    Cut,  // TODO
    DragFace(DragFaceState),
    MoveSelection, // TODO
    PaintMaterial, // TODO
    Paste,         // TODO
    Tiler,         // TODO: tile the current selection by dragging; replaces DragFace
}

// TODO: resize selection cursor

// TODO: smart tools; given some map palette and constraints, you can carve out section of map, and
// tiles get placed automatically, e.g. walls, floors, doors, and stairs are detected
