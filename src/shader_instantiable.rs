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

@file    drawable.rs
@brief   Part of OpenGL library
 */

//a Notes
//
//

//a Imports
use geo_nd::{matrix};

use crate::{Mat4, Transformation, Renderable, Vertices, ShaderClass, UniformId};

//a Vao
/// The Vao *must* be owned by a ShaderInstantiable, which borrows
/// from the Instantiable, which owns the GL buffers for the indices
/// and vertices etc
///
/// Because of this the Vao cannot outlive the ShaderInstantiable, which
/// cannot outlive the GL buffer for the vertices and indices etc
struct Vao {
    gl_vao : u32,
}

impl Vao {
    //fp new
    pub fn new(shader_class:&dyn ShaderClass, vertices:&Vertices) -> Self {
        let (indices, position, attrs) = vertices.borrow();
        crate::check_errors().unwrap();
        let mut gl_vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut gl_vao);
            gl::BindVertexArray(gl_vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,
                           indices.gl_buffer() );
            println!("VAO {} {:?}", gl_vao, indices);
        }
        crate::check_errors().unwrap();
        for (index, vertex_attr) in shader_class.attr_names() {
            if *vertex_attr == model3d::VertexAttr::Position {
                println!(".. posn {} {}", *index, position);
                position.bind_to_vao(*index);
                crate::check_errors().unwrap();
            } else {
                for (va, buffer) in attrs {
                    if *vertex_attr == *va {
                        println!(".. {:?} {} {}", *vertex_attr, *index, buffer);
                        buffer.bind_to_vao(*index);
                    }
                    crate::check_errors().unwrap();
                }
            }
        }
        unsafe {
            gl::BindVertexArray(0);
        }
        crate::check_errors().unwrap();
        Self {gl_vao}
    }
    //fp bind_vao
    pub fn bind_vao(&self) {
        unsafe {
            gl::BindVertexArray(self.gl_vao);
        }
    }
}

//a ShaderInstantiable
//tp ShaderInstantiable
/// This is a shader-specific instantiable built from the vertices of an Instantiable
///
/// A shader requires a VAO that maps *some* of the vertex attribute
/// buffers to particular attribute UIDs in the shader program
///
/// It requires mapping of textures to texture things
///
/// Possibly it will also require some particullar Uniforms
///
/// An Instance can be renderd with a shader by using the RenderRecipe
/// from the Instantiable, using the matrix and bone positions in the
/// Instance, and using the VAOs and other data in the
/// ShaderInstantiable.
///
/// It borrows from the Instantiable and so does not need to its own GlBuffers
pub struct ShaderInstantiable<'a> {
    instantiable : &'a model3d::Instantiable<Renderable>,
    // vaos is 1-to-1 with instantiable::vertices, specific to this shader (class)
    vaos: Vec<Vao>,
    shader_class : &'a dyn ShaderClass,
}

//ip ShaderInstantiable
impl <'a> ShaderInstantiable<'a> {
    //fp new
    pub fn new(shader_class:&'a dyn ShaderClass, instantiable: &'a model3d::Instantiable<Renderable>) -> Self {
        let mut vaos = Vec::new();
        for v in &instantiable.vertices {
            vaos.push(Vao::new(shader_class, v));
        }
        Self { instantiable, vaos, shader_class }
    }

    // gl_draw
    pub fn gl_draw(&self, instance:&model3d::Instance<Renderable>) {
        // shader camera matrix (already set?)
            if let Some(u) = self.shader_class.uniform(UniformId::ModelMatrix) {
                let mat = instance.transformation.mat4();
                unsafe {gl::UniformMatrix4fv(u, 1, gl::FALSE, mat.as_ptr());}
            }
        for (i, p) in self.instantiable.render_recipe.primitives.iter().enumerate() {
            // set MeshMatrix (if different to last)
            if let Some(u) = self.shader_class.uniform(UniformId::MeshMatrix) {
                let m = self.instantiable.render_recipe.matrix_for_primitives[i];
                unsafe {gl::UniformMatrix4fv(u, 1, gl::FALSE, self.instantiable.render_recipe.matrices[m].as_ptr());}
            }
            // set material info to that for shader_instantiable p.material_index,(if different to last)
            // (if p.vertices_index different to last)
            self.vaos[p.vertices_index()].bind_vao();
            let byte_offset = p.byte_offset();
            use model3d::PrimitiveType::*;
            let gl_type = match p.primitive_type() {
                Points => gl::POINTS,
                Lines => gl::LINES,
                LineLoop => gl::LINE_LOOP,
                LineStrip => gl::LINE_STRIP,
                Triangles => gl::TRIANGLES,
                TriangleFan => gl::TRIANGLE_FAN,
                TriangleStrip => gl::TRIANGLE_STRIP,
            };
            unsafe {
                gl::DrawElements( gl_type,
                                  p.index_count() as i32,
                                  gl::UNSIGNED_BYTE, // index_type,
                                  std::mem::transmute(byte_offset) );
            }
        }
    }
        /*
        // for bone_set_pose in instance.bone_set_poses {
        //  bone_set_pose.update(tick)
        // }
            //for (t,m,b) in self.meshes:
            //if b>=0:
            //bma = self.bone_set_poses[b]
            //program.set_uniform_if("uBonesMatrices",
            //lambda u:GL.glUniformMatrix4fv(u, bma.max_index, False, bma.data))
            //program.set_uniform_if("uBonesScale",
            //lambda u: GL.glUniform1f(u, 1.0) )
            //pass
        //else:
            //program.set_uniform_if("uBonesScale",
            //lambda u: GL.glUniform1f(u, 0.0) )
            //pass
            # Provide mesh matrix and material uniforms
            program.set_uniform_if("uMeshMatrix",
                                   lambda u: GL.glUniformMatrix4fv(u, 1, False, t.mat4()) )

    instance bone matrices
    instance model matrix
    for (i, p) in render_recipe.primitives.iter().enumerate() {
        m = matrix_for_primitives[i];
        set uMeshMatrix to render_recipe.matrices[m] (if different to last)
        set material info to that for shader_instantiable p.material_index,(if different to last)
        set vao to that for shader_instantiable p.vertices_index,(if different to last)
        p.index_offset, // byte_offset?
        p.index_count,
        let gl_type = p.primitive_type as gl_type,
                gl::DrawElements( gl_type,
                                  p.index_count,
                                  index_type,
                                  std::mem::transmute(byte_offset) );

            }
*/

    //zz All done
}

