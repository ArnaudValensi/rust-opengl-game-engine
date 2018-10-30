extern crate gl;
extern crate glutin;
extern crate imgui;
extern crate imgui_opengl_renderer;

use self::glutin::GlContext;
use self::imgui::*;
use self::imgui_opengl_renderer::Renderer;
use cgmath::Vector3;
use components::transform::Transform;
use input::Input;
use specs::{Join, Read, ReadStorage, System};
use std::cell::RefCell;
use std::rc::Rc;
use time::Time;
use window::Window;

pub struct GuiRendering {
    window: Rc<RefCell<Window>>,
    ui_renderer: Renderer,
    imgui: ImGui,
    selected_entity_index: i32,
}

impl GuiRendering {
    pub fn new(window: Rc<RefCell<Window>>) -> Self {
        let (imgui, ui_renderer) = {
            let gl_window = &window.borrow().gl_window;
            let mut imgui = imgui::ImGui::init();

            imgui.set_ini_filename(None);
            let ui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |symbol| {
                gl_window.get_proc_address(symbol) as _
            });

            (imgui, ui_renderer)
        };

        Self {
            window,
            ui_renderer,
            imgui,
            selected_entity_index: -1,
        }
    }
}

impl<'a> System<'a> for GuiRendering {
    type SystemData = (Read<'a, Time>, Read<'a, Input>, ReadStorage<'a, Transform>);

    fn run(&mut self, data: Self::SystemData) {
        let (time, input, tranform_storage) = data;

        let delta_time_in_seconds = time.get_delta_time_in_seconds();
        let average_delta_time_in_seconds = time.get_average_delta_time_in_seconds();
        let gl_window = &self.window.borrow().gl_window;

        let size_pixels = gl_window.get_inner_size().unwrap();
        let hdipi = gl_window.hidpi_factor();
        let size_points = (
            (size_pixels.0 as f32 / hdipi) as u32,
            (size_pixels.1 as f32 / hdipi) as u32,
        );

        update_mouse(&mut self.imgui, &input);

        let ui = self
            .imgui
            .frame(size_points, size_pixels, delta_time_in_seconds);

        // let mut open = true;
        // ui.show_demo_window(&mut open);
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

        let selected_entity_index = &mut self.selected_entity_index;
        ui.window(im_str!("Inspector"))
            .size((370.0, 130.0), ImGuiCond::FirstUseEver)
            .build(|| {
                let mut tranform_names: Vec<ImString> = Vec::new();

                for (transform,) in (&tranform_storage,).join() {
                    tranform_names.push(ImString::new(transform.name.clone()));
                }

                let tranform_names: Vec<&ImStr> =
                    tranform_names.iter().map(|s| s.as_ref()).collect();

                ui.text(im_str!(
                    "{:.3}ms/frame ({:.1}FPS)",
                    average_delta_time_in_seconds * 1000.0,
                    1.0 / average_delta_time_in_seconds,
                ));
                ui.separator();

                ui.combo(
                    im_str!("Entity"),
                    selected_entity_index,
                    &tranform_names[..],
                    -1,
                );

                if *selected_entity_index != -1 {
                    let selected_transform = (&tranform_storage,)
                        .join()
                        .nth(*selected_entity_index as usize)
                        .unwrap()
                        .0;

                    let mut position: [f32; 3] = [
                        selected_transform.local_position.x,
                        selected_transform.local_position.y,
                        selected_transform.local_position.z,
                    ];
                    let rotation_vector: Vector3<f32> = selected_transform.to_euler_angles();
                    let mut rotation: [f32; 3] =
                        [rotation_vector.x, rotation_vector.y, rotation_vector.z];

                    ui.input_float3(im_str!("Position"), &mut position).build();
                    ui.input_float3(im_str!("Rotation"), &mut rotation).build();
                }
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

    imgui.set_mouse_down(&[
        input.get_mouse_left(),
        input.get_mouse_right(),
        input.get_mouse_middle(),
        false,
        false,
    ]);

    imgui.set_mouse_wheel(input.get_mouse_wheel() / scale.1 * 0.05);
}
