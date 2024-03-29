extern crate gl;
extern crate glutin;

use self::glutin::{
    ContextBuilder,
    EventsLoop,
    GlContext,
    GlWindow,
    WindowBuilder,
    // GlProfile, GlRequest,
};

use self::glutin::dpi::{LogicalSize, LogicalPosition};

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
            .with_dimensions(LogicalSize::from((width, height)));
        let context = ContextBuilder::new().with_vsync(true);
        // .with_gl_profile(GlProfile::Core)
        // .with_gl(GlRequest::Latest);
        // .with_multisampling(config.multisampling);
        let gl_window = GlWindow::new(window, context, &events_loop).unwrap();

        // FIXME: On Mac 10.14 (Mojave) we need to resize the window after creation.
        // This is related to this issue https://github.com/tomaka/glutin/issues/1069
        events_loop.poll_events(|_| {});
        let logical_size =
            gl_window.get_outer_size()
            .expect("Window no longer exists");
        let physical_size = logical_size.to_physical(gl_window.get_hidpi_factor());
        gl_window.resize(physical_size);

        gl_window
            .grab_cursor(true)
            .expect("could not grab mouse cursor");

        gl_window.hide_cursor(true);

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
        let logical_size = gl_window.get_inner_size().unwrap();
        let cursor_position = LogicalPosition {
            x: logical_size.width / 2.0,
            y: logical_size.height / 2.0,
        };

        gl_window
            .set_cursor_position(cursor_position)
            .expect("could not center mouse cursor");
    }
}
