use cgmath::Vector3;
use cgmath::prelude::*;

// NOTE: without repr(C) the compiler may reorder the fields or use different padding/alignment than C.
// Depending on how you pass the data to OpenGL, this may be bad. In this case it's not strictly
// necessary though because of the `offset!` macro used below in setupMesh()
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub color_index: u8,
    pub normal: Vector3<f32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::zero(),
            color_index: 0,
            normal: Vector3::zero(),
        }
    }
}
