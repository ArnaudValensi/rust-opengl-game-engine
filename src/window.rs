extern crate gl;
extern crate glutin;

use self::glutin::{
    ContextBuilder,
    CursorState,
    EventsLoop,
    GlContext,
    GlWindow,
    WindowBuilder,
    // GlProfile, GlRequest,
};

pub struct Window {
    pub gl_window: GlWindow,
    pub running: bool,
    pub events_loop: EventsLoop,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        let mut events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_title("BigSeed")
            .with_dimensions(width, height);
        let context = ContextBuilder::new().with_vsync(true);
        // .with_gl_profile(GlProfile::Core)
        // .with_gl(GlRequest::Latest);
        // .with_multisampling(config.multisampling);
        let gl_window = GlWindow::new(window, context, &events_loop).unwrap();

        // FIXME: On Mac 10.14 (Mojave) we need to resize the window after creation.
        // This is related to this issue https://github.com/tomaka/glutin/issues/1069
        events_loop.poll_events(|_| {});
        let (width, height): (u32, u32) =
            gl_window.get_outer_size().expect("Window no longer exists");
        gl_window.resize(width, height);

        gl_window
            .set_cursor_state(CursorState::Grab)
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
        }
    }

    pub fn center_mouse_cursor(gl_window: &GlWindow) {
        let hidpi_factor = gl_window.hidpi_factor() as i32;
        let window_size = gl_window.get_inner_size().unwrap();
        let posx = window_size.0 as i32 / hidpi_factor / 2;
        let posy = window_size.1 as i32 / hidpi_factor / 2;

        gl_window
            .set_cursor_position(posx, posy)
            .expect("could not center mouse cursor");
    }
}
