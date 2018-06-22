extern crate gl;
extern crate glutin;

use specs::{ReadStorage, System, Join};
use std::time::{Instant, Duration};
use cgmath::{Vector3, Matrix4, vec3, Point3};
use components::transform::Transform;
use components::mesh_render::MeshRender;
use components::camera::Camera;
use window::Window;
use self::glutin::GlContext;
use std::rc::Rc;

const CAMERA_UP: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub struct Render {
    delta_time: Duration,
    last_frame: Instant,
    window: Rc<Window>,
}

impl Render {
    pub fn new(window: Rc<Window>) -> Self {
        Self {
            delta_time: Duration::default(),
            last_frame: Instant::now(),
            window,
        }
    }
}

impl<'a> System<'a> for Render {
    type SystemData = (
        ReadStorage<'a, Transform>,
        ReadStorage<'a, MeshRender>,
        ReadStorage<'a, Camera>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (tranform_storage, mesh_render_storage, camera_storage) = data;

        // yaw is initialized to -90.0 degrees since a yaw of 0.0 results in a direction vector
        // pointing to the right so we initially rotate a bit to the left.
        let mut _yaw: f32 = -90.0;
        let mut _pitch: f32 = 0.0;

        let current_frame = Instant::now();
        self.delta_time = current_frame - self.last_frame;
        self.last_frame = current_frame;

        let camera_transform = get_camera_transform(&tranform_storage, &camera_storage);

        for (mesh_transform, mesh_render) in (&tranform_storage, &mesh_render_storage).join() {
            // println!("= transform, mesh_render");
            render_mesh(Rc::clone(&self.window), &mesh_transform, &mesh_render, &camera_transform);
        }
    }
}

fn render_mesh(
    window: Rc<Window>,
    mesh_tranform: &Transform,
    mesh_render: &MeshRender,
    camera_tranform: &Transform
) {
    let camera_front: Vector3<f32> = Vector3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    let camera_pos = camera_tranform.position;

    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        mesh_render.material.bind();

        // camera/view transformation
        let view: Matrix4<f32> = Matrix4::look_at(
            camera_pos,
            camera_pos + camera_front,
            CAMERA_UP
        );
        mesh_render.material.set_matrix4("view", &view);

        let mesh_vector = point_to_vector(mesh_tranform.position);
        let model: Matrix4<f32> = Matrix4::from_translation(mesh_vector);
        mesh_render.material.set_matrix4("model", &model);

        mesh_render.mesh.Draw();
    }

    window.gl_window.swap_buffers().unwrap();
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
