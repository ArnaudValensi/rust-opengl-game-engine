extern crate gl;
extern crate glutin;
extern crate imgui;
extern crate imgui_opengl_renderer;

use specs::{ReadStorage, System, Join};
use cgmath::{Vector3, Matrix4, vec3, Point3};
use components::transform::Transform;
use components::mesh_render::MeshRender;
use components::camera::Camera;

const CAMERA_UP: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub struct Render { }

impl Render {
    pub fn new() -> Self {
        Self { }
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
        let camera_transform = get_camera_transform(&tranform_storage, &camera_storage);

        for (mesh_transform, mesh_render) in (&tranform_storage, &mesh_render_storage).join() {
            render_mesh(
                &mesh_transform,
                &mesh_render,
                &camera_transform,
            );
        }
    }
}

fn render_mesh(
    mesh_tranform: &Transform,
    mesh_render: &MeshRender,
    camera_tranform: &Transform,
) {
    let camera_pos = camera_tranform.position;
    let camera_forward = camera_tranform.forward();

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
