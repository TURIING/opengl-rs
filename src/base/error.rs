use std::ffi::NulError;
use std::string::FromUtf8Error;

use image::ImageError;
use thiserror::Error as ThisError;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, ThisError)]
pub enum GLError {
    #[error("An error occurred while compiling shader code.")]
    ShaderError(#[from] ShaderError),

    #[error("Glfw init error.")]
    GlfwError(#[from] glfw::InitError),

    #[error("An error occurred while loading the model.")]
    ModelError(#[from] ModelError),
}

#[allow(dead_code)]
#[derive(Debug, ThisError)]
pub enum ModelError {
    #[error("Unknown texture type.")]
    UnkownTextureType(String),

    #[error("An error occurred while loading the model.")]
    ModelLoadError(#[from] tobj::LoadError),

    #[error("Error occurred while reading image.")]
    TextureLoadError(#[from] ImageError),
}

#[allow(clippy::enum_variant_names)]
#[allow(dead_code)]
#[derive(Debug, ThisError)]
pub enum ShaderError {
    #[error("Error while compiling shader: {0}")]
    CompilationError(String),

    #[error("Error while linking shaders: {0}")]
    LinkingError(String),

    #[error("Cannot find the location index of the property variable named {0}.")]
    AttributeLocationParseError(String),

    #[error("Cannot find the location index of the uniform variable named {0}.")]
    UniformLocationParseError(String),

    #[error{"{0}"}]
    Utf8Error(#[from] FromUtf8Error),
    
    #[error{"{0}"}]
    NulError(#[from] NulError),

    #[error("An error occurred while reading the file.")]
    IoError(#[from] std::io::Error),
}