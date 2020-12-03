use bevy::asset::Handle;
use bevy::reflect::TypeUuid;
use bevy::render::{color::Color, renderer::RenderResources, shader::ShaderDefs, texture::Texture};

#[derive(Debug, RenderResources, ShaderDefs, TypeUuid)]
#[uuid = "87f03b0d-bdd1-4e62-9718-4f55f4ede154"]
pub struct ArrayMaterial {
    pub albedo: Color,
    #[shader_def]
    pub albedo_texture: Option<Handle<Texture>>,
}

impl Default for ArrayMaterial {
    fn default() -> Self {
        Self {
            albedo: Color::rgb(1.0, 1.0, 1.0),
            albedo_texture: None,
        }
    }
}

impl From<Color> for ArrayMaterial {
    fn from(color: Color) -> Self {
        Self {
            albedo: color,
            ..Default::default()
        }
    }
}

impl From<Handle<Texture>> for ArrayMaterial {
    fn from(texture: Handle<Texture>) -> Self {
        Self {
            albedo_texture: Some(texture),
            ..Default::default()
        }
    }
}
