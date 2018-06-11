use cgmath::{ Vector3, Vector2 };
use cgmath::prelude::*;

// NOTE: without repr(C) the compiler may reorder the fields or use different padding/alignment than C.
// Depending on how you pass the data to OpenGL, this may be bad. In this case it's not strictly
// necessary though because of the `offset!` macro used below in setupMesh()
#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    // position
    pub position: Vector3<f32>,
    // normal
    pub normal: Vector3<f32>,
    // texCoords
    pub tex_coords: Vector2<f32>,
    // tangent
    pub tangent: Vector3<f32>,
    // bitangent
    pub bitangent: Vector3<f32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::zero(),
            normal: Vector3::zero(),
            tex_coords: Vector2::zero(),
            tangent: Vector3::zero(),
            bitangent: Vector3::zero(),
        }
    }
}
