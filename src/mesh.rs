#![allow(non_snake_case)]
#![allow(dead_code)]

use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;

use gl;

use vertex::Vertex;
use mesh_data::MeshData;

#[derive(Debug, Clone)]
pub struct Texture {
    pub id: u32,
    pub type_: String,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    /*  Mesh Data  */
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i32>,
    pub textures: Vec<Texture>,
    pub VAO: u32,

    /*  Render data  */
    VBO: u32,
    EBO: u32,
}

impl Mesh {
    pub fn new(mesh_data: MeshData, textures: Vec<Texture>) -> Mesh {
        let mut mesh = Mesh {
            vertices: mesh_data.vertices,
            indices: mesh_data.indices,
            textures,
            VAO: 0,
            VBO: 0,
            EBO: 0,
        };

        unsafe { mesh.setupMesh() }
        mesh
    }

    pub unsafe fn Draw(&self) {
        // draw mesh
        gl::BindVertexArray(self.VAO);
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
        gl::BindVertexArray(0);

        // always good practice to set everything back to defaults once configured.
        gl::ActiveTexture(gl::TEXTURE0);
    }

    unsafe fn setupMesh(&mut self) {
        // create buffers/arrays
        gl::GenVertexArrays(1, &mut self.VAO);
        gl::GenBuffers(1, &mut self.VBO);
        gl::GenBuffers(1, &mut self.EBO);

        gl::BindVertexArray(self.VAO);
        // load data into vertex buffers
        gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO);
        // A great thing about structs with repr(C) is that their memory layout is sequential for all its items.
        // The effect is that we can simply pass a pointer to the struct and it translates perfectly to a glm::vec3/2 array which
        // again translates to 3/2 floats which translates to a byte array.
        let size = (self.vertices.len() * size_of::<Vertex>()) as isize;
        let data = &self.vertices[0] as *const Vertex as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
        let size = (self.indices.len() * size_of::<i32>()) as isize;
        let data = &self.indices[0] as *const i32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        // set the vertex attribute pointers
        let size = size_of::<Vertex>() as i32;
        // vertex positions
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, position) as *const c_void);
        // vertex colors index
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribIPointer(1, 1, gl::UNSIGNED_BYTE, size, offset_of!(Vertex, color_index) as *const c_void);
        // vertex normals
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, normal) as *const c_void);

        gl::BindVertexArray(0);
    }
}
