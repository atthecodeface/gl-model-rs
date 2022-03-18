use crate::UniformId;
pub trait Texture : Sized + std::fmt::Debug {
}
pub trait ShaderClass {
    fn attr_names(&self) -> &[(gl::types::GLuint, model3d::VertexAttr)];
    fn uniform(&self, uniform_id:UniformId) -> Option<gl::types::GLint>;
}

