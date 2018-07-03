extern crate gl;
extern crate glutin;
extern crate imgui;
extern crate imgui_opengl_renderer;

use specs::{System, Read};
use window::Window;
use time::Time;
use self::glutin::GlContext;
use std::rc::Rc;
use std::cell::RefCell;
use self::imgui::*;
use self::imgui_opengl_renderer::Renderer;

pub struct GuiRendering {
    window: Rc<RefCell<Window>>,
    ui_renderer: Renderer,
    imgui: ImGui,
}

impl GuiRendering {
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        let (imgui, ui_renderer) = {
            let gl_window = &window.borrow().gl_window;
            let mut imgui = imgui::ImGui::init();

            imgui.set_ini_filename(None);
            let ui_renderer = imgui_opengl_renderer::Renderer::new(
                &mut imgui, |symbol| gl_window.get_proc_address(symbol) as _
            );

            (imgui, ui_renderer)
        };

        Self {
            window,
            ui_renderer,
            imgui,
        }
    }
}

impl<'a> System<'a> for GuiRendering {
    type SystemData = (
        Read<'a, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (time,) = data;

        let delta_time_in_seconds = time.get_delta_time_in_seconds();
        let gl_window = &self.window.borrow().gl_window;

        let size_pixels = gl_window.get_inner_size().unwrap();
        let hdipi = gl_window.hidpi_factor();
        let size_points = (
            (size_pixels.0 as f32 / hdipi) as u32,
            (size_pixels.1 as f32 / hdipi) as u32,
        );

        let ui = self.imgui.frame(size_points, size_pixels, delta_time_in_seconds);

        ui.window(im_str!("Hello world"))
            .size((600.0, 200.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.text(im_str!("Hello world!"));
                ui.separator();
                let mouse_pos = ui.imgui().mouse_pos();
                ui.text(im_str!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos.0,
                    mouse_pos.1
                ));
            });

        self.ui_renderer.render(ui);
    }
}
