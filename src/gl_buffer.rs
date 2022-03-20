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

@file    gl_buffer.rs
@brief   An OpenGL Buffer representation
 */

//a Imports
use std::rc::Rc;
use model3d::{BufferClient, BufferData};
use crate::{Renderable, RenderContext};

//a GlBuffer
//tp GlBuffer
/// A simple structure provides a reference-counted OpenGl buffer;
/// when the last reference is dropped it will drop the OpenGl buffer
/// that it contains, if any
///
/// Its actual buffer is created from vertex data or from indices;
/// from vertex data it is created *only* on the first invocation
/// (from a [model3d::BufferData]) as subsequent 'creations' will be
/// duplicates - the reference count should ont be changed either as
/// it is the *same* BufferData instance that is invoking the creation
///
/// For indices a buffer is created for the [model3d::BufferView], as
/// the buffer in this case must be an OpenGL ELEMENT_ARRAY_BUFFER;
/// this could perhaps be optimized to reduce the number of OpenGL
/// buffers with much more code.
#[derive(Debug, Clone)]
pub struct GlBuffer {
    /// The OpenGL Buffer
    gl   : Rc<gl::types::GLuint>,
}

//ip Default for GlBuffer
impl Default for GlBuffer {
    fn default() -> Self {
        let gl = Rc::new(0);
        Self { gl }
    }
}

//ip BufferClient for GlBuffer
impl BufferClient<Renderable> for GlBuffer {
    /// Create a client
    ///
    /// This may be called multiple times for the same [BufferData]; if the
    /// gl buffer is 0 then create, else it already exists with the same data
    fn create(&mut self, data: &BufferData<Renderable>, _render_context: &mut RenderContext) {
        if self.is_none() {
            println!("Buffer create data");
            self.of_data(data)
        }
    }
}

//ip GlBuffer
impl GlBuffer {
    //ap gl_buffer
    /// Get the gl_buffer associated with the data
    pub fn gl_buffer(&self) -> gl::types::GLuint {
        *self.gl
    }

    //ap as_ptr
    /// Get a pointer to this gl
    pub fn as_ptr(&self) -> *const gl::types::GLuint {
        Rc::as_ptr(&self.gl)
    }

    //mp is_none
    /// Return true if the buffer is not initialized
    pub fn is_none(&self) -> bool {
        *self.gl == 0
    }

    //mp of_data
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    pub fn of_data(&mut self, data:&BufferData<Renderable>) {
        assert!(self.is_none());
        let mut gl : gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, (&mut gl) as *mut gl::types::GLuint );
            gl::BindBuffer(gl::ARRAY_BUFFER, gl);
            gl::BufferData(gl::ARRAY_BUFFER,
                           data.byte_length as gl::types::GLsizeiptr,
                           data.as_ptr() as *const gl::types::GLvoid,
                           gl::STATIC_DRAW );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0 ); // unbind to protect
        }
        self.gl = Rc::new(gl);
    }

    //mp of_indices
    /// Create the OpenGL ELEMENT_ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    pub fn of_indices(&mut self, view:&model3d::BufferView<Renderable>) {
        assert!(self.is_none());
        let mut gl : gl::types::GLuint = 0;
        let ele_size = {
            use model3d::BufferElementType::*;
            match view.ele_type {
                Int8 => 1,
                Int16 => 2,
                Int32 => 4,
                _ => panic!("Indices BufferView must have an int element type")
            }
        };
        let byte_length = ele_size * view.count;
        unsafe {
            // stops the indices messing up other VAO
            gl::BindVertexArray(0);
            let buffer = view.data.as_ptr().add(view.byte_offset as usize);
            gl::GenBuffers(1, (&mut gl) as *mut gl::types::GLuint );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, gl );
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           byte_length as gl::types::GLsizeiptr,
                           buffer as *const gl::types::GLvoid,
                           gl::STATIC_DRAW );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0 ); // unbind to protect
        }
        self.gl = Rc::new(gl);
    }

    //mp uniform_buffer
    /// Create the OpenGL 
    pub fn uniform_buffer<F:Sized>(&mut self, data:&[F]) {
        assert!(self.is_none());
        let buffer = data.as_ptr();
        let byte_length = std::mem::size_of::<F>() * data.len();
        let mut gl : gl::types::GLuint = 0;
        unsafe {
            gl::BindVertexArray(0);
            gl::GenBuffers(1, (&mut gl) as *mut gl::types::GLuint );
            gl::BindBuffer(gl::UNIFORM_BUFFER, gl );
            gl::BufferData(gl::UNIFORM_BUFFER,
                           byte_length as gl::types::GLsizeiptr,
                           buffer as *const gl::types::GLvoid,
                           gl::STATIC_DRAW );
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0 ); // unbind to protect
            println!("Uniform buffer {} bound @{:?}+{}", gl, buffer, byte_length);
        }
        self.gl = Rc::new(gl);
    }

    //zz All done
}

//ip Drop for GlBuffer
impl Drop for GlBuffer {
    //fp drop
    /// If an OpenGL buffer has been created for this then delete it
    fn drop(&mut self) {
        if Rc::strong_count(&self.gl)==1 && !self.is_none() {
            unsafe {
                gl::DeleteBuffers(1, self.as_ptr() );
            }
        }
    }
}

//ip Display for GlBuffer
impl std::fmt::Display for GlBuffer {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f,"GL({})", self.gl)
    }
}

