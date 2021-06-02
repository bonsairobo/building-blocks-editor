use serde::Deserialize;

#[derive(Clone, Copy, Deserialize, Default)]
pub struct Config {
    pub wireframes: bool,
    pub camera: CameraType,
}

impl Config {
    pub fn read_file(path: &str) -> Result<Self, ron::Error> {
        let reader = std::fs::File::open(path)?;

        ron::de::from_reader(reader)
    }
}

#[derive(Clone, Copy, Deserialize)]
pub enum CameraType {
    Unreal,
    Orbit,
}

impl Default for CameraType {
    fn default() -> Self {
        CameraType::Unreal
    }
}
