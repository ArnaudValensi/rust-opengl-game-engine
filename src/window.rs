extern crate glutin;
extern crate gl;

use self::glutin::{
    WindowBuilder,
    ContextBuilder,
    EventsLoop,
    GlWindow,
    GlContext,
    CursorState,
};

pub struct Window {
    pub gl_window: GlWindow,
    pub running: bool,
    pub events_loop: EventsLoop,
    cursor_locked: bool,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_title("Hello, world!")
            .with_dimensions(width, height);
        let context = ContextBuilder::new()
            .with_vsync(true);
        let gl_window = GlWindow::new(window, context, &events_loop).unwrap();

        gl_window
            .set_cursor_state(CursorState::Grab)
            .ok()
            .expect("could not grab mouse cursor");

        unsafe {
            gl_window.make_current().unwrap();
        }

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        Window::center_mouse_cursor(&gl_window);

        Window {
            events_loop,
            gl_window,
            running: true,
            cursor_locked: true,
        }
    }

    pub fn set_cursor_locked(&mut self, lock: bool) {
        self.cursor_locked = lock;
    }

    pub fn center_mouse_cursor(gl_window: &GlWindow) {
        let hidpi_factor = gl_window.hidpi_factor() as i32;
        let window_size = gl_window.get_inner_size().unwrap();
        let posx = window_size.0 as i32 / hidpi_factor / 2;
        let posy = window_size.1 as i32 / hidpi_factor / 2;

        gl_window
        .set_cursor_position(posx, posy)
        .ok()
        .expect("could not center mouse cursor");
    }
}
