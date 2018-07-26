extern crate gl;
extern crate glutin;
extern crate imgui;
extern crate imgui_opengl_renderer;

use specs::{System, Read, ReadStorage, Join};
use window::Window;
use time::Time;
use input::input::Input;
use components::transform::Transform;
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
        Read<'a, Input>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (time, input, tranform_storage) = data;

        let delta_time_in_seconds = time.get_delta_time_in_seconds();
        let gl_window = &self.window.borrow().gl_window;

        let size_pixels = gl_window.get_inner_size().unwrap();
        let hdipi = gl_window.hidpi_factor();
        let size_points = (
            (size_pixels.0 as f32 / hdipi) as u32,
            (size_pixels.1 as f32 / hdipi) as u32,
        );

        update_mouse(&mut self.imgui, &input);

        let ui = self.imgui.frame(size_points, size_pixels, delta_time_in_seconds);

        let mut open = true;
        ui.show_demo_window(&mut open);
        // ui.show_metrics_window(&mut open);
        // ui.show_default_style_editor();
        // ui.window(im_str!("Hello world"))
        //     .size((600.0, 200.0), ImGuiCond::FirstUseEver)
        //     .build(|| {
        //         ui.text(im_str!("Hello world!"));
        //         ui.separator();
        //         let mouse_pos = ui.imgui().mouse_pos();
        //         ui.text(im_str!(
        //             "Mouse Position: ({:.1},{:.1})",
        //             mouse_pos.0,
        //             mouse_pos.1
        //         ));
        //     });

        ui.window(im_str!("Inspector"))
        .size((200.0, 600.0), ImGuiCond::FirstUseEver)
        .build(|| {
            for (transform,) in (&tranform_storage,).join() {
                ui.text(im_str!("{}", transform.name));
            }

            let mut slected_item: i32 = 0;
            ui.combo(
                im_str!("combo"),
                &mut slected_item,
                &[
                    im_str!("aaaa"),
                    im_str!("bbbb"),
                    im_str!("cccc"),
                    im_str!("dddd"),
                    im_str!("eeee"),
                ],
                -1,
            );

            println!("slected_item: {:#?}", slected_item);
        });

        self.ui_renderer.render(ui);
    }
}

fn update_mouse(imgui: &mut ImGui, input: &Input) {
    let mouse_position = input.get_mouse_position();
    let scale = imgui.display_framebuffer_scale();

    imgui.set_mouse_pos(
        mouse_position.0 as f32 / scale.0,
        mouse_position.1 as f32 / scale.1,
    );

    imgui.set_mouse_down(
        &[
            input.get_mouse_left(),
            input.get_mouse_right(),
            input.get_mouse_middle(),
            false,
            false,
        ],
    );

    imgui.set_mouse_wheel(input.get_mouse_wheel() / scale.1 * 0.05);
}
