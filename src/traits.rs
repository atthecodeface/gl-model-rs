pub trait Texture : Sized + std::fmt::Debug {
}
pub trait ShaderClass {
    fn attr_names(&self) -> &[(gl::types::GLuint, model3d::VertexAttr)];
}

