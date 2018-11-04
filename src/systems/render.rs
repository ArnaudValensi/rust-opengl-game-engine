extern crate gl;
extern crate glutin;
extern crate imgui;
extern crate imgui_opengl_renderer;

use cgmath::{Matrix4, Vector3};
use components::mesh_render::MeshRender;
use components::transform::Transform;
use resources::active_camera::ActiveCamera;
use specs::{Join, ReadExpect, ReadStorage, System};

const CAMERA_UP: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

#[derive(Default, Debug)]
pub struct Render;

impl Render {
    pub fn new() -> Self {
        Self::default()
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
            render_mesh(&mesh_transform, &mesh_render, &camera_transform);
        }
    }
}

fn clear_screen() {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

fn render_mesh(mesh_transform: &Transform, mesh_render: &MeshRender, camera_tranform: &Transform) {
    let camera_pos = camera_tranform.local_position;
    let camera_forward = camera_tranform.forward();

    unsafe {
        // TODO: Batch entities with the same material.
        mesh_render.material.bind();

        let view: Matrix4<f32> =
            Matrix4::look_at(camera_pos, camera_pos + camera_forward, CAMERA_UP);
        mesh_render.material.set_matrix4("view", &view);

        mesh_render
            .material
            .set_matrix4("model", &mesh_transform.world_matrix);

        mesh_render
            .material
            .set_matrix3("normalMatrix", &mesh_transform.get_normal_matrix());

        mesh_render.mesh.Draw();
    }
}
