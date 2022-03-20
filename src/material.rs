//a Imports
use crate::{Renderable, RenderContext};


//a Material
//tp Material
/// A null material for now
#[derive(Debug)]
pub struct Material (u32);
impl std::fmt::Display for Material {
    fn fmt(&self, fmt:&mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

//ip Default for Material
impl Default for Material {
    fn default() -> Self { Self (0) }
}

//ip MaterialClient for Material
impl model3d::MaterialClient<Renderable> for Material {
    fn create(&mut self, _material:&dyn model3d::Material<Renderable>, _render_context:&mut RenderContext) {}
    fn drop(&mut self, _material:&dyn model3d::Material<Renderable>, _render_context:&mut RenderContext) {}
}
