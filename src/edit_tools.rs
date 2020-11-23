mod drag_face;
mod hover_hint;
mod plugin;

pub use plugin::EditToolsPlugin;

use drag_face::DragFaceTool;

pub enum CurrentTool {
    DragFace(DragFaceTool),
    Copy,  // TODO
    Cut,   // TODO
    Paste, // TODO
}
