use std::ffi::CStr;
use cgmath::Matrix4;
use shader::Shader;
use texture::Texture;

pub struct Material {
   shader: Shader,
   texture: Texture,
}

impl Material {
    pub fn new() -> Self {
      // build and compile our shader program
      // ------------------------------------
      let shader = Shader::new(
          "src/shaders/7.2.camera.vs",
          "src/shaders/7.2.camera.fs"
      );

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
