use cgmath::Matrix4;
use shader::Shader;
use std::ffi::{CStr, CString};
use texture::Texture;

#[derive(Debug, Clone)]
pub struct Material {
    shader: Shader,
    texture: Texture,
}

impl Default for Material {
    fn default() -> Self {
        // build and compile our shader program
        // ------------------------------------
        let shader = Shader::new("src/shaders/7.2.camera.vs", "src/shaders/7.2.camera.fs");

        // load and create a texture
        // -------------------------
        // texture 1
        // ---------
        let texture = Texture::new("resources/textures/container.jpg");

        // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
        // -------------------------------------------------------------------------------------------
        unsafe {
            shader.useProgram();
            shader.setInt(c_str!("texture1"), texture.get_id() as i32);
        }

        Self { shader, texture }
    }
}

impl Material {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn bind(&self) {
        unsafe {
            self.shader.useProgram();
            self.texture.bind();
        }
    }

    pub fn set_matrix4(&self, name: &str, matrix: &Matrix4<f32>) {
        let c_string = CString::new(name).unwrap();
        let c_str = c_string.as_c_str();

        unsafe {
            self.shader.setMat4(c_str, matrix);
        }
    }

    pub fn set_integer(&self, name: &str, value: i32) {
        let c_string = CString::new(name).unwrap();
        let c_str = c_string.as_c_str();

        unsafe {
            self.shader.setInt(c_str, value);
        }
    }
}
