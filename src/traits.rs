//a Imports
use crate::UniformId;

//tt Texture
/// Some trait for a texture
pub trait Texture : Sized + std::fmt::Debug {
}

//tt ShaderClass
/// Trait required for a shader program
pub trait ShaderClass {
    /// Borrow a slice of attribute / program attribute location pairings
    fn attributes(&self) -> &[(gl::types::GLuint, model3d::VertexAttr)];

    /// Attempt to retrieve a uniform from a [UniformId] - return None
    /// if the shader program does not have that uniform
    fn uniform(&self, uniform_id:UniformId) -> Option<gl::types::GLint>;
}

