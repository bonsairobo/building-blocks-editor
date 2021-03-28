use serde::Deserialize;

#[derive(Clone, Copy, Deserialize)]
pub struct Config {
    pub wireframes: bool,
}

impl Config {
    pub fn read_file(path: &str) -> Result<Self, ron::Error> {
        let reader = std::fs::File::open(path)?;

        ron::de::from_reader(reader)
    }
}
