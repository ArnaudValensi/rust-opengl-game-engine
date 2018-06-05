use cgmath::{ Vector3, Vector2 };
use cgmath::prelude::*;

// NOTE: without repr(C) the compiler may reorder the fields or use different padding/alignment than C.
// Depending on how you pass the data to OpenGL, this may be bad. In this case it's not strictly
// necessary though because of the `offset!` macro used below in setupMesh()
#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    // position
    pub Position: Vector3<f32>,
    // normal
    pub Normal: Vector3<f32>,
    // texCoords
    pub TexCoords: Vector2<f32>,
    // tangent
    pub Tangent: Vector3<f32>,
    // bitangent
    pub Bitangent: Vector3<f32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            Position: Vector3::zero(),
            Normal: Vector3::zero(),
            TexCoords: Vector2::zero(),
            Tangent: Vector3::zero(),
            Bitangent: Vector3::zero(),
        }
    }
}
