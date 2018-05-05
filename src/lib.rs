extern crate gl;
extern crate image;
extern crate cgmath;
extern crate floating_duration;

pub mod common;
pub mod shader;
pub mod macros;
pub mod input;

mod _1_1_2_hello_window;
pub use self::_1_1_2_hello_window::*;
mod _1_2_1_hello_triangle;
pub use self::_1_2_1_hello_triangle::*;
mod _1_2_2_hello_triangle_indexed;
pub use self::_1_2_2_hello_triangle_indexed::*;
mod _1_7_2_camera_keyboard;
pub use self::_1_7_2_camera_keyboard::*;
mod _1_7_3_camera_mouse;
pub use self::_1_7_3_camera_mouse::*;
