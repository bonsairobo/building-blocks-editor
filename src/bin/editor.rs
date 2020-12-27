use building_blocks_editor::EditorPlugin;

use bevy::{app::prelude::*, render::prelude::*, window::WindowDescriptor, DefaultPlugins};

fn main() {
    let mut window_desc = WindowDescriptor::default();
    window_desc.width = 800.0;
    window_desc.height = 600.0;
    window_desc.title = "Building Blocks Editor".to_string();

    App::build()
        // Core bevy stuff.
        .add_resource(window_desc)
        .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.4)))
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins) // TODO: remove bevy_pbr
        // Editor stuff.
        .add_plugin(EditorPlugin)
        .run();
}
