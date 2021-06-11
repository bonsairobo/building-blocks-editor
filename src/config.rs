use serde::Deserialize;

use smooth_bevy_cameras::controllers::{
    orbit::OrbitCameraController, unreal::UnrealCameraController,
};

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
    Unreal(UnrealCameraController),
    Orbit(OrbitCameraController),
}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig::Orbit(Default::default())
    }
}
