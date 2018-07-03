extern crate gl;
extern crate glutin;
extern crate imgui;
extern crate imgui_opengl_renderer;

use specs::{ReadStorage, System, Join, Read};
use cgmath::{Vector3, Matrix4, vec3, Point3};
use components::transform::Transform;
use components::mesh_render::MeshRender;
use components::camera::Camera;
use window::Window;
use time::Time;
use self::glutin::GlContext;
use std::rc::Rc;
use std::cell::RefCell;
use self::imgui::*;
use self::imgui_opengl_renderer::Renderer;

const CAMERA_UP: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub struct Render {
    window: Rc<RefCell<Window>>,
    ui_renderer: Renderer,
    imgui: ImGui,
}

impl Render {
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

impl<'a> System<'a> for Render {
    type SystemData = (
        ReadStorage<'a, Transform>,
        ReadStorage<'a, MeshRender>,
        ReadStorage<'a, Camera>,
        Read<'a, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (tranform_storage, mesh_render_storage, camera_storage, time) = data;

        let delta_time_in_seconds = time.get_delta_time_in_seconds();

        // yaw is initialized to -90.0 degrees since a yaw of 0.0 results in a direction vector
        // pointing to the right so we initially rotate a bit to the left.
        let mut _yaw: f32 = -90.0;
        let mut _pitch: f32 = 0.0;

        let camera_transform = get_camera_transform(&tranform_storage, &camera_storage);

        for (mesh_transform, mesh_render) in (&tranform_storage, &mesh_render_storage).join() {
            // println!("= transform, mesh_render");
            render_mesh(
                Rc::clone(&self.window),
                &mesh_transform,
                &mesh_render,
                &camera_transform,
                &mut self.imgui,
                &self.ui_renderer,
                delta_time_in_seconds,
            );
        }
    }
}

fn render_mesh(
    window: Rc<RefCell<Window>>,
    mesh_tranform: &Transform,
    mesh_render: &MeshRender,
    camera_tranform: &Transform,
    imgui: &mut ImGui,
    ui_renderer: &Renderer,
    delta_time_seconds: f32,
) {
    let camera_pos = camera_tranform.position;
    let camera_forward = camera_tranform.forward();
    let gl_window = &window.borrow().gl_window;

    // println!("camera_tranform.rotation: {:?}", camera_tranform.rotation);
    // println!("camera_tranform.rotation: {:#?}", Euler::from(camera_tranform.rotation));

    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        mesh_render.material.bind();

        // camera/view transformation
        let view: Matrix4<f32> = Matrix4::look_at(
            camera_pos,
            camera_pos + camera_forward,
            CAMERA_UP
        );
        mesh_render.material.set_matrix4("view", &view);

        let mesh_vector = point_to_vector(mesh_tranform.position);
        let model: Matrix4<f32> = Matrix4::from_translation(mesh_vector);
        mesh_render.material.set_matrix4("model", &model);

        mesh_render.mesh.Draw();
    }

    // TODO: move
    let size_pixels = gl_window.get_inner_size().unwrap();
    let hdipi = gl_window.hidpi_factor();
    let size_points = (
        (size_pixels.0 as f32 / hdipi) as u32,
        (size_pixels.1 as f32 / hdipi) as u32,
    );

    let ui = imgui.frame(size_points, size_pixels, delta_time_seconds);

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

    ui_renderer.render(ui);

    gl_window.swap_buffers().unwrap();
}

// Only one camera is available.
fn get_camera_transform(
    tranform_storage: &ReadStorage<Transform>,
    camera_storage: &ReadStorage<Camera>
) -> Transform {
    let mut camera_join = (tranform_storage, camera_storage).join();
    let camera_tuple = camera_join.nth(0);
    let camera_transform = camera_tuple.unwrap().0;

    (*camera_transform).clone()
}

fn point_to_vector(point: Point3<f32>) -> Vector3<f32> {
    vec3(
       point.x,
       point.y,
       point.z,
   )
}
