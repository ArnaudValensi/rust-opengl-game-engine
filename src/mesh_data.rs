use vertex::Vertex;

#[derive(Debug)]
#[repr(C)]
pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i32>,
}

impl MeshData {
    pub fn new() -> Self {
        let vertices: Vec<Vertex> = Vec::new();
        let indices: Vec<i32> = Vec::new();

        Self {
            vertices,
            indices,
        }
    }
}
