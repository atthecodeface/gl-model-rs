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

@file    buffer.rs
@brief   An OpenGL buffer representation Part of geometry library
 */

//a Notes
//
//

//a Imports
use model3d::{ViewClient, BufferElementType, VertexAttr};

use crate::GlBuffer;
use crate::{Renderable, RenderContext};

//a VertexBuffer
//tp VertexBuffer
/// 
/// A subset of a data buffer for use with OpenGL vertex data.
///
/// A data buffer may contain a lot of data per vertex, such as
/// position, normal, tangent, color etc.  A [VertexBuffer] is
/// then a subset of this data - perhaps picking out just the
/// position, for example, for a set of vertices
///
/// OpenGL will have one copy of the data for all the [VertexBuffer]
#[derive(Debug, Clone)]
pub struct VertexBuffer {
    /// Ref-counted buffer
    gl_buffer   : GlBuffer,
    /// Number of elements per vertex - 1 to 4
    pub count: u32,
    /// The type of each element
    pub ele_type : BufferElementType,
    /// Offset from start of buffer to first byte of data
    pub byte_offset : u32,
    /// Stride of data in the buffer - 0 for count*sizeof(ele_type)
    pub stride : u32,
}

//ip VertexBuffer
impl VertexBuffer {
    //ap gl_buffer
    /// Get the gl_buffer associated with the data, assuming its
    /// `gl_create` method has been invoked at least once
    pub fn gl_buffer(&self) -> gl::types::GLuint {
        self.gl_buffer.gl_buffer()
    }

    //mp of_view
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    fn of_view(&mut self, view:&model3d::BufferView<Renderable>, render_context:&mut RenderContext) {
        view.data.create_client(render_context);
        self.count = view.count;
        self.ele_type = view.ele_type;
        self.byte_offset = view.byte_offset;
        self.stride = view.stride;
        self.gl_buffer = view.data.borrow_client().clone();
    }

    //fp gl_element_type
    fn gl_element_type(&self) -> gl::types::GLuint {
        use model3d::BufferElementType::*;
        match self.ele_type {
            Float32 => gl::FLOAT,
            Float16 => gl::HALF_FLOAT,
            Int8    =>  gl::BYTE,
            Int16   =>  gl::SHORT,
            Int32   =>  gl::INT,
        }
    }

    //fp bind_to_vao
    /// Bind the buffer as a vertex attribute to the current VAO
    pub fn bind_to_vao(&self, attr_index:gl::types::GLuint) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_buffer());
            gl::EnableVertexAttribArray(attr_index);
            gl::VertexAttribPointer(attr_index,
                                    self.count as i32, // size
                                    self.gl_element_type(),
                                    gl::FALSE, // normalized
                                    self.stride as i32, // stride
                                    std::mem::transmute::<usize, *const std::os::raw::c_void>(self.byte_offset as usize) // ptr
                                    );
        }
    }

    //zz All done
}

//ip Default for VertexBuffer
impl Default for VertexBuffer {
    fn default() -> Self {
        let gl_buffer = GlBuffer::default();
        let count  = 0;
        let ele_type = BufferElementType::Float32;
        let byte_offset = 0;
        let stride = 0;
        Self {
            gl_buffer,
            count, ele_type, byte_offset, stride
        }
    }
}

//ip Display for VertexBuffer
impl std::fmt::Display for VertexBuffer {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f,"Vert({}+{}:#{} {:?} @{})",
               self.gl_buffer.gl_buffer(),
               self.byte_offset,
               self.count,
               self.ele_type,
               self.stride
        )
    }
}

//ip DefaultIndentedDisplay for VertexBuffer
impl indent_display::DefaultIndentedDisplay for VertexBuffer {}

//a IndexBuffer
//tp IndexBuffer
/// 
/// A subset of a data buffer for use with OpenGL index data.
///
/// An IndexBuffer directly owns the OpenGL buffer which is an
/// ElementArray rather than vertex data
#[derive(Debug, Clone)]
pub struct IndexBuffer {
    /// Ref-counted buffer
    gl_buffer   : GlBuffer,
    /// Number of elements per index - 1 to 4
    pub count: u32,
    /// The type of each element
    pub ele_type : BufferElementType,
}

//ip Default for IndexBuffer
impl Default for IndexBuffer {
    fn default() -> Self {
        let gl_buffer = GlBuffer::default();
        let count  = 0;
        let ele_type = BufferElementType::Int8;
        Self {
            gl_buffer,
            count, ele_type
        }
    }
}

//ip IndexBuffer
impl IndexBuffer {
    //ap gl_buffer
    /// Get the gl_buffer associated with the data, assuming its
    /// `gl_create` method has been invoked at least once
    pub fn gl_buffer(&self) -> gl::types::GLuint {
        self.gl_buffer.gl_buffer()
    }

    //mp of_view
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    fn of_view(view:&model3d::BufferView<Renderable>, _render_context:&mut RenderContext) -> Self {
        let mut gl_buffer = GlBuffer::default();
        gl_buffer.of_indices(view);
        let count = view.count;
        let ele_type = view.ele_type;
        println!("Create indices buffer {} of view {:?}#{}", gl_buffer, view.ele_type, view.count);
        Self {
            gl_buffer,
            count, ele_type
        }
    }

    //fp bind_to_vao
    /// Bind the index buffer to the current VAO
    pub fn bind_to_vao(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,
                           self.gl_buffer() );
        }
    }
    
    //zz All done
}

//ip Display for IndexBuffer
impl std::fmt::Display for IndexBuffer {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f,"Ind({}#{} {:?})",
               self.gl_buffer.gl_buffer(),
               self.count,
               self.ele_type,
        )
    }
}

//ip DefaultIndentedDisplay for IndexBuffer
impl indent_display::DefaultIndentedDisplay for IndexBuffer {}

//a BufferView
//tp BufferView
/// 
/// A view of data with either vertices of indices
#[derive(Debug, Clone)]
pub enum BufferView {
    /// Vertex buffer
    VertexBuffer(VertexBuffer),
    /// Index buffer
    IndexBuffer(IndexBuffer),
}

//ip Default for BufferView
impl Default for BufferView {
    fn default() -> Self {
        Self::VertexBuffer(VertexBuffer::default())
    }
}

//ip BufferView
impl BufferView {
    //fp as_index_buffer
    /// Return the [IndexBuffer] that this [BufferView] is of - if it
    /// is not a view of indices then panic
    pub fn as_index_buffer(&self) -> &IndexBuffer {
        match self {
            Self::IndexBuffer(index_buffer) => index_buffer,
            _ => panic!("Attempt to borrow a VertexBuffer as an IndexBuffer")
        }
    }

    //fp as_vertex_buffer
    /// Return the [VertexBuffer] that this [BufferView] is of - if it
    /// is not a view of vertex attributess then panic
    pub fn as_vertex_buffer(&self) -> &VertexBuffer {
        match self {
            Self::VertexBuffer(vertex_buffer) => vertex_buffer,
            _ => panic!("Attempt to borrow an IndexBuffer as an VertexBuffer")
        }
    }
}

//ip ViewClient for BufferView
impl ViewClient<Renderable> for BufferView {
    //mp create
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    fn create(&mut self, view:&model3d::BufferView<Renderable>, attr:VertexAttr, render_context:&mut RenderContext) {
        if attr == VertexAttr::Indices {
            let index_buffer = IndexBuffer::of_view(view, render_context);
            *self = BufferView::IndexBuffer(index_buffer);
        } else {
            match self {
                BufferView::IndexBuffer(_) => panic!("Vertex buffer is already an index buffer"),
                BufferView::VertexBuffer(vb) => {
                    vb.of_view(view, render_context);
                },
            }
        }
    }

    //zz All done
}

//ip Display for BufferView
impl std::fmt::Display for BufferView {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::IndexBuffer(index_buffer) => index_buffer.fmt(f),
            Self::VertexBuffer(vertex_buffer) => vertex_buffer.fmt(f),
        }
    }
}

//ip DefaultIndentedDisplay for BufferView
impl indent_display::DefaultIndentedDisplay for BufferView {}

