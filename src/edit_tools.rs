mod drag_face;
mod hover_hint;
mod plugin;

pub use plugin::EditToolsPlugin;

use drag_face::DragFaceTool;

pub enum CurrentTool {
    MakeSelection, // TODO: extract from DragFaceTool
    MoveSelection, // TODO
    Copy,          // TODO
    Cut,           // TODO
    Paste,         // TODO
    DragFace(DragFaceTool),
    PaintMaterial, // TODO
}
