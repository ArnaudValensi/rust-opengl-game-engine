use std::sync::atomic::{self, AtomicUsize};
use gl;
use image;
use image::GenericImage;
use std::path::Path;
use std::os::raw::c_void;

static OBJECT_COUNTER: AtomicUsize = atomic::ATOMIC_USIZE_INIT;

#[derive(Debug, Clone)]
pub struct Texture {
    pub path: String,
    id: u32,
    index: u32,
}

impl Texture {
    pub fn new(path: &str) -> Self {
        let index = OBJECT_COUNTER.fetch_add(1, atomic::Ordering::SeqCst) as u32;

        let id = unsafe {
            let mut id = 0;

            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            // load image, create texture and generate mipmaps
            let img = image::open(&Path::new(path)).expect("Failed to load texture");
            let data = img.raw_pixels();
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGB as i32,
                           img.width() as i32,
                           img.height() as i32,
                           0,
                           gl::RGB,
                           gl::UNSIGNED_BYTE,
                           &data[0] as *const u8 as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);

            id
        };

        Texture {
            id,
            path: String::default(),
            index,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.index);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn get_id(&self) -> u32 {
        self.index
    }
}
