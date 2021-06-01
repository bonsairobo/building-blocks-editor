use building_blocks_editor::{BevyPlugins, Config, EditorPlugin};

use bevy::{
    app::prelude::*,
    render::{prelude::*, wireframe::WireframeConfig},
    wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions},
    window::WindowDescriptor,
};

fn main() -> Result<(), ron::Error> {
    let config = Config::read_file("config.ron")?;

    let window_desc = WindowDescriptor {
        width: 1600.0,
        height: 900.0,
        title: "Building Blocks Editor".to_string(),
        ..Default::default()
    };

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
        .insert_resource(WireframeConfig { global: true })
        .add_plugins(BevyPlugins::new(config))
        // Editor stuff.
        .add_plugin(EditorPlugin)
        .run();

    Ok(())
}
