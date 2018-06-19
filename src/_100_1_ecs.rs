extern crate gl;
extern crate glutin;

use errors::*;
use specs::World;
use cgmath::{Matrix4, vec3,  Deg, perspective};
use components::transform::Transform;
use components::mesh_render::MeshRender;
use components::camera::Camera;
use voxel::chunk::Chunk;
use mesh::Mesh;
use material::Material;
use voxel::voxel_mesh_builder::build_mesh;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;
const FOV: f32 = 45.0;

fn run() -> Result<()> {
    println!("Hi!");

    let material = Material::new();
    let projection: Matrix4<f32> = perspective(Deg(FOV), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);

    unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);
        material.set_matrix4("projection", &projection);
    }

    let mut chunk = Chunk::new(2, 3, 4);

    chunk.set_voxel(0, 0, 0, 1)?;
    chunk.set_voxel(1, 0, 0, 1)?;

    println!("chunk: {:#?}", chunk);

    let chunk_mesh_data = build_mesh(&chunk);
    let chunk_mesh = Mesh::new(chunk_mesh_data, Vec::default());

    let mut world = World::new();

    world.register::<Transform>();
    world.register::<MeshRender>();
    world.register::<Camera>();

    let camera = world.create_entity()
        .with(Transform { position: vec3(0.0, 0.0, 3.0) })
        .with(Camera)
        .build();

    let terrain = world.create_entity()
        .with(Transform { position: vec3(0.0, 0.0, 0.0) })
        .with(MeshRender { material, mesh: chunk_mesh })
        .build();

    // println!("camera: {:?}", camera);

    Ok(())
}

fn print_errors_and_exit(e: &Error) {
    println!("error: {}", e);

    for e in e.iter().skip(1) {
        println!("caused by: {}", e);
    }

    // The backtrace is not always generated. Try to run this example
    // with `RUST_BACKTRACE=1`.
    if let Some(backtrace) = e.backtrace() {
        println!("backtrace: {:?}", backtrace);
    }

    ::std::process::exit(1);
}

pub fn main_100_1() {
    if let Err(ref e) = run() {
        print_errors_and_exit(e);
    }
}
