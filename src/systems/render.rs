extern crate gl;
extern crate glutin;
extern crate imgui;
extern crate imgui_opengl_renderer;

use specs::{ReadExpect, ReadStorage, System, Join};
use cgmath::{Vector3, Matrix4, vec3, Point3};
use resources::active_camera::ActiveCamera;
use components::transform::Transform;
use components::mesh_render::MeshRender;

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
        ReadExpect<'a, ActiveCamera>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, MeshRender>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (active_camera, tranform_storage, mesh_render_storage) = data;
        let camera_transform = tranform_storage.get(active_camera.0).unwrap();

        clear_screen();

        for (mesh_transform, mesh_render) in (&tranform_storage, &mesh_render_storage).join() {
            render_mesh(
                &mesh_transform,
                &mesh_render,
                &camera_transform,
            );
        }
    }
}

fn clear_screen() {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
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
        // TODO: batch entities with the same material
        mesh_render.material.bind();

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

fn point_to_vector(point: Point3<f32>) -> Vector3<f32> {
    vec3(
       point.x,
       point.y,
       point.z,
   )
}
