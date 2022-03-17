/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    renderable.rs
@brief   An OpenGL Renderable implementation for a 3D Model
 */

//a Renderable
use crate::{GlBuffer, BufferView, Texture, Material, Vertices};
pub struct Renderable {
}
pub struct RenderContext {
}
impl model3d::Renderable for Renderable {
    type Context  = RenderContext;
    type Buffer   = GlBuffer;
    type View     = BufferView;
    type Texture  = Texture;
    type Material = Material;
    type Vertices = Vertices;
}

// Vertex-attribute object
/*
pub type Vao = gl::types::GLuint;
pub type BufferData<'a> = model3d::BufferData<'a, GlBuf>;
pub type BufferView<'a> = model3d::BufferView<'a, GlBuf>;
pub trait GlBufferData {
    fn gl_create(&self, indices:bool);
}
impl <'a> GlBufferData for BufferData<'a> {
    fn gl_create_data(&self) {
    }
}

//a ShaderProgramClass
pub trait ShaderProgramClass {
    fn get_attr(&self, key:&str) -> Option<gl::types::GLuint>;
    fn get_uniform(&self, key:&str) -> Option<gl::types::GLuint>;
    fn gl_bind_attr_view(&self, key:&str, view:Option<&BufferView>) {
        if let Some(attr) = self.get_attr(key) {
            match view {
                None       => {unsafe{gl::DisableVertexAttribArray(attr);}},
                Some(view) => { /* view.gl_bind_attribute(attr) */; },
            }
        }
    }
}

mod utils;
mod shader;
mod program;
mod vertices;

pub use shader::GlShader;
pub use program::Program as GlProgram;

pub use vertices::Vertices;

//pub mod primitive;
//pub mod mesh;
//pub mod object;

//pub mod shader;
//pub use shader::{ShaderClass};
*/
