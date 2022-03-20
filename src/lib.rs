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

@file    lib.rs
@brief   Graphical model library
 */

//a Documentation
#![warn(missing_docs)]

// Document code examples cannot be executed
// so don't require them right now
// 
// #![warn(rustdoc::missing_doc_code_examples)]

/*!
# OpenGL Model library

This library provides structures for OpenGL shaders and model3d
clients that allow model3d objects to be instantiable within OpenGL,
and to be drawn as model3d instances.

Each [model3d::BufferData] becomes a [GlBuffer], which is an
reference-counted OpenGL buffer. A [BufferView] is either a [VertexBuffer] (a subset of a
[GlBuffer]) or an [IndexBuffer] (a specific [GlBuffer] that is
created with just the index data).

[Vertices] is then an [IndexBuffer] and [VertexBuffer]s for the vertex
positions and other attributes.

From these a [model3d::Instantiable] can be made for an object,
specific to this library (using its [Vertices] and buffer types).

Shader programms are also supported - these are compiled from GLSL
source, and are expected to have mappings givenn to map
[model3d::VertexAttr] to particular attribute locations (and similarly
for other uniforms). These programs expose an implementation of
[ShaderClass], which provides access to these attributes and uniforms.

A [ShaderInstantiable] can then be created for a
[model3d::Instantiable] and a particular instance of [ShaderClass];
this will have appropraite OpenGL VAOs constructed, and then they may
be drawn given a particular [model3d::Instance].

The draw process for a [ShaderInstantiable] is to set appropriate
uniforms and run through the render recipe provided by model3d for the
[model3d::Instantiable], setting more uniforms and invoking draw calls for the
primitives.

!*/

//a Imports and exports
pub use model3d::{Mat3, Mat4, Quat, Vec3, Vec4, Transformation};

mod gl_buffer;
mod buffer;
mod texture;
mod material;
mod vertices;
mod shader_instantiable;
mod renderable;
mod utils;
mod traits;
mod shader;
mod program;

pub use gl_buffer::GlBuffer;
pub use utils::{get_shaderiv, get_programiv, check_errors, get_shader_error};
pub use buffer::{IndexBuffer, VertexBuffer, BufferView};
pub use texture::Texture;
pub use material::Material;
pub use vertices::Vertices;
pub use shader_instantiable::ShaderInstantiable;
pub use renderable::{Renderable, RenderContext};
pub use traits::ShaderClass;
pub use shader::GlShader;
pub use program::UniformId;
pub use program::Program as GlProgram;

