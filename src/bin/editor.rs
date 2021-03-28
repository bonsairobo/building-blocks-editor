use building_blocks_editor::{BevyPlugins, EditorPlugin};

use bevy::{
    app::prelude::*,
    render::prelude::*,
    wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions},
    window::WindowDescriptor,
};

fn main() {
    let mut window_desc = WindowDescriptor::default();
    window_desc.width = 1600.0;
    window_desc.height = 900.0;
    window_desc.title = "Building Blocks Editor".to_string();

    App::build()
        // Core bevy stuff.
        .insert_resource(window_desc)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.4)))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WgpuOptions {
            features: WgpuFeatures {
                // The Wireframe plugin requires NonFillPolygonMode feature
                features: vec![WgpuFeature::NonFillPolygonMode],
            },
            ..Default::default()
        })
        .add_plugins(BevyPlugins)
        // Editor stuff.
        .add_plugin(EditorPlugin)
        .run();
}
