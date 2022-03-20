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

//a Imports
use crate::{GlBuffer, BufferView, Texture, Material, Vertices};

//a Renderable
//tp Renderable
/// Contains nothing right now
pub struct Renderable {
}

//tp RenderContext
/// The OpenGL context is global, and so no data is required for the [RenderContext]
pub struct RenderContext {
}

//ip model3d::Renderable for Renderable
impl model3d::Renderable for Renderable {
    type Context  = RenderContext;
    type Buffer   = GlBuffer;
    type View     = BufferView;
    type Texture  = Texture;
    type Material = Material;
    type Vertices = Vertices;
}

