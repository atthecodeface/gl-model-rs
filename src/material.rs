use crate::{Renderable, RenderContext};
#[derive(Debug)]
pub struct Material (u32);
impl std::fmt::Display for Material {
    fn fmt(&self, fmt:&mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

impl Default for Material {
    fn default() -> Self { Self (0) }
}
impl model3d::MaterialClient<Renderable> for Material {
    fn create(&mut self, material:&dyn model3d::Material<Renderable>, _render_context:&mut RenderContext) {}
    fn drop(&mut self, material:&dyn model3d::Material<Renderable>, _render_context:&mut RenderContext) {}
}
