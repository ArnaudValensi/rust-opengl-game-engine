extern crate gl;
extern crate glutin;

use errors::*;
use specs::{World, DispatcherBuilder};
use cgmath::{Matrix4,  Deg, perspective, Point3};
use components::transform::Transform;
use components::mesh_render::MeshRender;
use components::camera::Camera;
use systems::render::Render;
use systems::window_event::WindowEvent;
use voxel::chunk::Chunk;
use mesh::Mesh;
use material::Material;
use voxel::voxel_mesh_builder::build_mesh;
use config::{SCR_WIDTH, SCR_HEIGHT};
use lifecycle::{Lifecycle, Event};
use input::input::Input;
use window::Window;
use std::rc::Rc;
use std::cell::RefCell;

// settings
const FOV: f32 = 45.0;

fn run() -> Result<()> {
    println!(" 🦄 Starting engine...");

    let window = Rc::new(RefCell::new(Window::new(SCR_WIDTH, SCR_HEIGHT)));
    let render_system = Render::new(Rc::clone(&window));
    let window_event_system = WindowEvent::new(Rc::clone(&window));
    let input = Input::new();
    let material = Material::new();
    let projection: Matrix4<f32> = perspective(Deg(FOV), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
    let mut event_loop = Lifecycle::new();

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

    world.add_resource(input);

    world.create_entity()
        .with(Transform { position: Point3::new(0.0, 0.0, 3.0) })
        .with(Camera)
        .build();

    world.create_entity()
        .with(Transform { position: Point3::new(0.0, 0.0, 0.0) })
        .with(MeshRender { material: material.clone(), mesh: chunk_mesh.clone() })
        .build();

    // world.create_entity()
    //     .with(Transform { position: Point3::new(0.0, 0.0, 0.0) })
    //     .with(MeshRender { material, mesh: chunk_mesh })
    //     .build();

    let mut dispatcher_builder = DispatcherBuilder::new();
        // .with(render_system, "render_system", &[])
    dispatcher_builder.add_thread_local(render_system);
    dispatcher_builder.add_thread_local(window_event_system);

    let mut dispatcher = dispatcher_builder.build();

    while let Some(event) = event_loop.next() {
        match event {
            Event::FixedUpdate => {}
            Event::OnInput => {}
            Event::Update => {
                dispatcher.dispatch(&mut world.res);
            }
            Event::Render => {}
        }
    }

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
