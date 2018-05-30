use cgmath::{ Vector3, Vector2 };

#[derive(Debug)]
#[repr(C)]
pub struct MeshData {
    pub vertices: Vec<Vector3<f32>>,
    pub triangles: Vec<i32>,
    pub uvs: Vec<Vector2<f32>>,
}

impl MeshData {
    pub fn new() -> Self {
        let vertices: Vec<Vector3<f32>> = Vec::new();
        let triangles: Vec<i32> = Vec::new();
        let uvs: Vec<Vector2<f32>> = Vec::new();

        Self {
            vertices,
            triangles,
            uvs,
        }
    }
}
