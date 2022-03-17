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

//a Documentation

/*!

A shader program consists of a number of [Shader]s linked together

!*/

//a Imports
use gl;
use std;
use std::ffi::CStr;
use std::ffi::CString;

use crate::utils;
use crate::GlShader;
use crate::ShaderClass;

//a Program
//tp Program
/// Program
pub struct Program {
    /// The GL ID of the program
    id: gl::types::GLuint,
    /// attribute names
    attr_names : Vec<(gl::types::GLuint, model3d::VertexAttr)>,
}

///ip Program
impl Program {
    //fp compile_program
    /// Compile a program from a slice of kind/source pairs
    pub fn compile_program(srcs:&[(gl::types::GLenum, &str)]) ->  Result<Self, String> {
        let mut shaders = Vec::new();
        for (kind, src) in srcs {
            shaders.push( GlShader::from_source(&CString::new(*src).unwrap(), *kind)? );
        }
        Self::from_shaders(&shaders)
    }
    
    //mp add_attr_name
    pub fn add_attr_name(&mut self, name:&str, vertex_attr:model3d::VertexAttr) -> Result<&mut Self, String> {
        // let attr_index = gl::GetUniformLocation( self.id, CString::new(name).unwrap().as_ptr() );
        let attr_index = unsafe {gl::GetAttribLocation( self.id, CString::new(name).unwrap().as_ptr() ) };
        if attr_index < 0 {
            Err(format!("Unable to find attribute {} in program", name))
        } else {
            self.attr_names.push( (attr_index as gl::types::GLuint, vertex_attr) );
            Ok(self)
        }
    }

    //fp from_shaders
    /// Create a program from a slice of shaders; link the shaders together
    pub fn from_shaders(shaders: &[GlShader]) -> Result<Self, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = utils::create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        let attr_names = Vec::new();
        Ok(Program {
            id: program_id,
            attr_names,
        }
        )
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

//ip Drop for Program
impl Drop for Program {
    //fp drop
    /// Drop requires the GLProgram to be deleted
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }

    //zz All done
}

//ip ShaderClass for Program
impl ShaderClass for Program {
    fn attr_names(&self) -> &[(gl::types::GLuint, model3d::VertexAttr)]
    {
        &self.attr_names
    }
}

//ip ShaderProgramClass for Program
/*
impl ShaderProgramClass for Program {
    //fp get_attr
    /// Get an attribute from the Program
    fn get_attr(&self, key: &str) -> Option<u32> {
        if key == "vPosition" {
            Some(0)
        } else {
            None
        }
    }

    //fp get_uniform
    /// No uniforms are supported so return None for all requests
    fn get_uniform(&self, _: &str) -> Option<u32> {
        None
    }
}
 */
