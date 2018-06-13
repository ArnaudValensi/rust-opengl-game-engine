use std::ffi::CStr;
use cgmath::Matrix4;
use shader::Shader;
use texture::Texture;

pub struct Material {
   shader: Shader,
   texture: Texture,
}

impl Material {
    pub fn new(shader: Shader, texture: Texture) -> Self {
        Self {
            shader,
            texture,
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.shader.useProgram();
            self.texture.bind();
        }
    }

    pub fn set_matrix4(&self, name: &CStr, matrix: &Matrix4<f32>) {
      unsafe {
         self.shader.setMat4(name, matrix);
      }
    }

    pub fn set_integer(&self, name: &CStr, value: i32) {
      unsafe {
         self.shader.setInt(name, value);
      }
    }
}
