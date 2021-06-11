use serde::Deserialize;

use smooth_bevy_cameras::{OrbitCameraControlConfig, UnrealCameraControlConfig};

#[derive(Clone, Copy, Deserialize, Default)]
pub struct Config {
    pub wireframes: bool,
    pub camera: CameraConfig,
}

impl Config {
    pub fn read_file(path: &str) -> Result<Self, ron::Error> {
        let reader = std::fs::File::open(path)?;

        ron::de::from_reader(reader)
    }
}

#[derive(Clone, Copy, Deserialize)]
pub enum CameraConfig {
    Unreal(UnrealCameraControlConfig),
    Orbit(OrbitCameraControlConfig),
}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig::Unreal(UnrealCameraControlConfig {
            mouse_rotate_sensitivity: 0.002,
            mouse_translate_sensitivity: 0.1,
            trackpad_translate_sensitivity: 0.1,
            smoothing_weight: 0.9,
        })
    }
}
