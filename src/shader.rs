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

@file    shader.rs
@brief   Part of OpenGL support library
 */

//a Imports
use gl;
use std;
use std::ffi::CStr;

//a GlShader
//tp GlShader
/// An OpenGL shader, of any kind, which can be created from source.
///
/// A number of shaders are linked together to make a program; once
/// the program has been linked, the shader can be dropped.
pub struct GlShader {
    /// The GL ID of the shader
    id: gl::types::GLuint,
}

//ip GlShader
impl GlShader {
    //fp from_source
    /// Create a shader of a particular kind from source
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Self, String> {
        let id = 
            unsafe {
                let id = gl::CreateShader(kind);
                gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
                gl::CompileShader(id);
                id
            };

        if crate::get_shaderiv(id, gl::COMPILE_STATUS) == 0 {
            let err = crate::get_shader_error( id,
                                         |id| crate::get_shaderiv(id, gl::INFO_LOG_LENGTH),
                                         |id, len, buf| unsafe {
                                             gl::GetShaderInfoLog( id, len, std::ptr::null_mut(), buf)
                                         } );
            if kind == gl::VERTEX_SHADER {
                Err(format!("Vertex shader error {}", err))
            } else {
                Err(format!("Fragment shader error {}", err))
            }                
        } else {
            Ok(Self { id })
        }
    }

    //fp from_vert_source
    /// Create a [Self] from vertex GLS source
    pub fn from_vert_source(source: &CStr) -> Result<Self, String> {
        Self::from_source(source, gl::VERTEX_SHADER)
    }

    //fp from_vert_source
    /// Create a [Self] from fragment GLS source
    pub fn from_frag_source(source: &CStr) -> Result<Self, String> {
        Self::from_source(source, gl::FRAGMENT_SHADER)
    }

    //fp id
    /// Get the shader program id
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

//ip Drop for GlShader
impl Drop for GlShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

