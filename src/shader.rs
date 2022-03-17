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

This library provides types for shader-specific objects.

An object instance that may be drawn by a shader is called a ShaderDrawable. This is an instance of a ShaderInstantiable. A ShaderInstantiable is a shader-specific derivation of an ObjectInstantiable.

An ObjectInstantiableData consists of an array of BoneSets, an array
of [Mat4] for the object's Meshes, and an array of Mesh that the
object consists of. The ObjectInstantiableData is derived from an
Object - which in turn is Hierarchy of ObjectNodes, each of the which
may have a BoneSet, a transformation (relative to its parent) and a
mesh. The ObjectInstantiableData is, in effect, a flattened set of
ObjectNode hierarchies. In flattening the nodes BoneSets are cloned in
to a linear array of BoneSets; each node's mesh has a relative-to-root
transformation matrix generated and placed in a linear array, and the
index in to

!*/

//a Imports
use gl;
use std;
use std::ffi::CStr;

use crate::utils;

//a GlShader
//tp GlShader
pub struct GlShader {
    /// The GL ID of the shader
    id: gl::types::GLuint,
}

//ip GlShader
impl GlShader {
    //fp from_source
    /// Create a shader of a particular kind from source
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = utils::create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }
        Ok(Self { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Self, String> {
        Self::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Self, String> {
        Self::from_source(source, gl::FRAGMENT_SHADER)
    }

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

