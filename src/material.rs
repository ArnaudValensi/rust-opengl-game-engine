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
    
    pub fn set_matrix4(&self, name: &CStr, mat: &Matrix4<f32>) {
        
    }
}
