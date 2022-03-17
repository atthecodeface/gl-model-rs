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
use model3d::{VertexAttr, BufferClient, VerticesClient};

use crate::{IndexBuffer, VertexBuffer, Renderable, RenderContext};

//a Vertices
//tp Vertices
/// This is a set of OpenGL vertices with GlBuffer's of all of its contents
///
/// This is part of the RenderContext, and so has a different lifetime
/// to the model3d objects and vertices.
/// This is created by invoking create_client on an object
///
/// A set of vertices using one or more [BufferData] through [BufferView]s.
///
/// In the old days the attr would have a GlBuffer generated for them in gl_create
/// Here we can clone it if it already exists
///
/// if the shader requiress it then the appropriate bind can be performed
///    pub fn gl_bind_attribute(&self, attr:gl::types::GLuint) {
///            gl::BindBuffer(gl::ARRAY_BUFFER, self.data.gl_buffer());
///            gl::EnableVertexAttribArray(attr);
///            gl::VertexAttribPointer(attr, // index
///                                    self.count, // size
///                                    self.gl_type, // types
///                                    gl::FALSE, // normalized
///                                    self.stride, // stride
///                                    std::mem::transmute::<usize, *const std::os::raw::c_void>(self.offset)
///            );
#[derive(Debug, Clone)]
pub struct Vertices {
    indices    : IndexBuffer,
    position   : VertexBuffer,
    attrs      : Vec<(VertexAttr, VertexBuffer)>,
}

impl Vertices {
    pub fn borrow(&self) -> (&IndexBuffer, &VertexBuffer, &Vec<(VertexAttr, VertexBuffer)>) {
        (&self.indices, &self.position, &self.attrs)
    }
}

impl std::fmt::Display for Vertices {
    fn fmt(&self, fmt:&mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "ind:{}", self.indices)?;
        writeln!(fmt, "pos:{}", self.position)
    }
}

impl Default for Vertices {
    /// Create a none
    fn default() -> Self {
        let indices  = IndexBuffer::default();
        let position = VertexBuffer::default();
        let attrs = Vec::new();
        Self { indices, position, attrs }
    }

}

impl model3d::VerticesClient<Renderable> for Vertices {
    //mp create
    /// Create based on Model3D vertices
    fn create(vertices: &model3d::Vertices<Renderable>, render_context: &mut RenderContext) -> Self {
        let indices  = vertices.borrow_indices().borrow_client().as_index_buffer().clone();
        let position = vertices.borrow_position().borrow_client().as_vertex_buffer().clone();
        let mut attrs = Vec::new();
        for (attr, buffer) in vertices.iter_attrs() {
            attrs.push( (*attr, buffer.borrow_client().as_vertex_buffer().clone() ) );
        }
        Self { indices, position, attrs }
    }
}
/*
//ip Vertices
impl <'a>  Vertices<'a> {
    pub fn new(indices: &'a BufferView<'a>, position:&'a BufferView<'a>) -> Self {
        Self { indices,
               position,
               indices_gl_id : 0,
               normal: None,
               tex_coords : None,
               joints : None,
               weights : None,
               tangent : None,
               color : None,
        }
    }

    //mp gl_create_indices
    fn gl_create_indices(&mut self) {
        let ele_size = 4;  // BUG - must get size of self.ele_type
        let byte_length = ele_size * self.indices.count;
        unsafe {
            let buffer = self.indices.data.as_ptr().add(self.indices.offset as usize);
            gl::GenBuffers(1, (&mut self.indices_gl_id) as *mut gl::types::GLuint );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.indices_gl_id );
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           byte_length as gl::types::GLsizeiptr,
                           buffer as *const gl::types::GLvoid,
                           gl::STATIC_DRAW );
        }
    }

    //mp gl_create
    /// Create the underlying buffers
    pub fn gl_create(&mut self) {
        unsafe {
            // stops the indices messing up other VAO
            gl::BindVertexArray(0);
        }
        self.gl_create_indices();
        self.position.gl_create(false);
        self.normal.map(|x| x.gl_create(false));
        self.tex_coords.map(|x| x.gl_create(false));
        self.joints.map(|x| x.gl_create(false));
        self.weights.map(|x| x.gl_create(false));
        self.tangent.map(|x| x.gl_create(false));
        self.color.map(|x| x.gl_create(false));
    }

    //mp gl_bind_to_shader
    /// Create the VAO, if that has not already been done
    pub fn gl_bind_to_shader <S:ShaderProgramClass>(&self, shader:&S) -> gl::types::GLuint {
        let mut gl_vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut gl_vao);
            gl::BindVertexArray(gl_vao);
        }
        self.indices.gl_bind(true);
        shader.gl_bind_attr_view("vPosition", Some(self.position));
        shader.gl_bind_attr_view("vNormal",    self.normal);
        shader.gl_bind_attr_view("vTexCoords", self.tex_coords);
        shader.gl_bind_attr_view("vJoints",    self.joints);
        shader.gl_bind_attr_view("vWeights",   self.weights);
        shader.gl_bind_attr_view("vTangent",   self.tangent);
        shader.gl_bind_attr_view("vColor",     self.color);
        unsafe {
            gl::BindVertexArray(0);
        }
        gl_vao
    }
}

    def hier_debug(self, hier:Hierarchy) -> Hierarchy:
        self.indices.hier_debug(hier)
        for (san,an) in self.attribute_mapping.items():
            if hasattr(self, an):
                mbv = getattr(self,an)
                if mbv is not None: mbv.hier_debug(hier, an)
                pass
            pass
        return hier
    #f All done
    pass

    //mp gl_bind_attribute
    /// Bind this view to a particular attribute
    pub fn gl_bind_attribute(&self, attr:gl::types::GLuint) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.data.gl_buffer());
            gl::EnableVertexAttribArray(attr);
            gl::VertexAttribPointer(attr, // index
                                    self.count, // size
                                    self.gl_type, // types
                                    gl::FALSE, // normalized
                                    self.stride, // stride
                                    std::mem::transmute::<usize, *const std::os::raw::c_void>(self.offset) // ptr
            );
        }
    }
*/
