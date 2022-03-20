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

@file    primitive.rs
@brief   Part of OpenGL library
 */

//a Imports
use model3d::{VertexAttr};

use crate::{IndexBuffer, VertexBuffer, Renderable, RenderContext};

//a Vertices
//tp Vertices
/// This is a set of OpenGL vertices with [crate::GlBuffer] for all of its contents
///
/// This is part of the [RenderContext], and so has a different
/// lifetime to the model3d objects and vertices. It is created by
/// invoking create_client on a [model3d::Object]
#[derive(Debug, Clone)]
pub struct Vertices {
    indices    : IndexBuffer,
    position   : VertexBuffer,
    attrs      : Vec<(VertexAttr, VertexBuffer)>,
}

impl Vertices {
    //fp borrow
    /// Borrow the indices, positions, and the array of other attributes
    pub fn borrow(&self) -> (&IndexBuffer, &VertexBuffer, &Vec<(VertexAttr, VertexBuffer)>) {
        (&self.indices, &self.position, &self.attrs)
    }
}

//ip Display for Vertices
impl std::fmt::Display for Vertices {
    fn fmt(&self, fmt:&mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "ind:{}", self.indices)?;
        writeln!(fmt, "pos:{}", self.position)
    }
}

//ip Default for Vertices
impl Default for Vertices {
    /// Create a none
    fn default() -> Self {
        let indices  = IndexBuffer::default();
        let position = VertexBuffer::default();
        let attrs = Vec::new();
        Self { indices, position, attrs }
    }

}

//ip VerticesClient for Vertices
impl model3d::VerticesClient<Renderable> for Vertices {
    //mp create
    /// Create based on [model3d::Vertices]
    fn create(vertices: &model3d::Vertices<Renderable>, _render_context: &mut RenderContext) -> Self {
        let indices  = vertices.borrow_indices().borrow_client().as_index_buffer().clone();
        let position = vertices.borrow_position().borrow_client().as_vertex_buffer().clone();
        let mut attrs = Vec::new();
        for (attr, buffer) in vertices.iter_attrs() {
            attrs.push( (*attr, buffer.borrow_client().as_vertex_buffer().clone() ) );
        }
        Self { indices, position, attrs }
    }
}
