#[derive(Debug)]
pub struct Texture (u32);
impl std::fmt::Display for Texture {
    fn fmt(&self, fmt:&mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}
impl model3d::TextureClient for Texture {
}
