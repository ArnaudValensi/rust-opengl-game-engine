extern crate gl;
extern crate glutin;

use errors::*;
use specs::{World, DispatcherBuilder};
use cgmath::{Matrix4,  Deg, perspective, Point3};
use components::transform::Transform;
use components::mesh_render::MeshRender;
use components::camera::Camera;
use components::player::Player;
use components::parent::Parent;
use resources::active_camera::ActiveCamera;
use resources::rotating_entity::RotatingEntity;
use systems::render::Render;
use systems::gui_rendering::GuiRendering;
use systems::swap_frame_buffer::SwapFrameBuffer;
use systems::window_event::WindowEvent;
use systems::player_movement::PlayerMovement;
use systems::mouse_control::MouseControl;
use systems::transformation::Transformation;
// use systems::Rotator;
use voxel::chunk::Chunk;
use mesh::Mesh;
use material::Material;
use voxel::voxel_mesh_builder::build_mesh;
use config::{SCR_WIDTH, SCR_HEIGHT};
use lifecycle::{Lifecycle, Event};
use input::input::Input;
use time::Time;
use window::Window;
use std::rc::Rc;
use std::cell::RefCell;

// settings
const FOV: f32 = 45.0;

fn run() -> Result<()> {
    println!(" 🦄 Starting engine...");

    let window = Rc::new(RefCell::new(Window::new(SCR_WIDTH, SCR_HEIGHT)));
    let render_system = Render::new();
    let swap_frame_buffer_system = SwapFrameBuffer::new(Rc::clone(&window));
    let window_event_system = WindowEvent::new(Rc::clone(&window));
    let mouse_control_system = MouseControl::new(Rc::clone(&window));
    let gui_rendering_system = GuiRendering::new(Rc::clone(&window));
    let input = Input::new();
    let time = Time::new();
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
    let mut chunk2 = Chunk::new(2, 2, 2);

    chunk.set_voxel(0, 0, 0, 1)?;
    chunk.set_voxel(1, 0, 0, 1)?;
    chunk.set_voxel(1, 0, 1, 1)?;
    chunk2.set_voxel(0, 0, 0, 1)?;

    println!("chunk: {:#?}", chunk);

    let chunk_mesh_data = build_mesh(&chunk);
    let chunk_mesh = Mesh::new(chunk_mesh_data, Vec::default());
    let chunk_mesh_data2 = build_mesh(&chunk2);
    let chunk_mesh2 = Mesh::new(chunk_mesh_data2, Vec::default());

    let mut world = World::new();

    world.register::<Transform>();
    world.register::<MeshRender>();
    world.register::<Camera>();
    world.register::<Player>();
    world.register::<Parent>();

    world.add_resource(time);
    world.add_resource(input);

    let scene_root_entity = world.create_entity().build();
    let transformation_system = Transformation::new(scene_root_entity);

    let camera_entity = world.create_entity()
        .with(Transform::new(Point3::new(0.0, 0.0, 0.0), "Camera"))
        .with(Camera)
        .with(Player)
        .build();
    world.add_resource(ActiveCamera(camera_entity));

    let mut chunk_transform = Transform::new(Point3::new(0.0, 0.0, -1.0), "Chunk0");
    chunk_transform.set_rotation(0.0, 45.0, 0.0);
    let chunk0 = world.create_entity()
        .with(chunk_transform)
        .with(MeshRender { material: material.clone(), mesh: chunk_mesh.clone() })
        .build();
    world.add_resource(RotatingEntity(chunk0));

    world.create_entity()
        .with(Parent { entity: chunk0 })
        .with(Transform::new(Point3::new(0.0, 0.0, -2.0), "Chunk1"))
        .with(MeshRender { material: material.clone(), mesh: chunk_mesh2.clone() })
        .build();

    let mut dispatcher_builder = DispatcherBuilder::new();

    dispatcher_builder.add_thread_local(window_event_system);
    dispatcher_builder.add_thread_local(mouse_control_system);
    dispatcher_builder.add_thread_local(PlayerMovement::new());
    // dispatcher_builder.add_thread_local(Rotator::new());
    dispatcher_builder.add_thread_local(transformation_system);
    dispatcher_builder.add_thread_local(render_system);
    dispatcher_builder.add_thread_local(gui_rendering_system);
    dispatcher_builder.add_thread_local(swap_frame_buffer_system);

    let mut dispatcher = dispatcher_builder.build();

    while let Some(event) = event_loop.next() {
        match event {
            Event::FixedUpdate => {}
            Event::OnInput => {}
            Event::Update => {
                {
                    let mut time = world.write_resource::<Time>();
                    (*time).update();
                }

                dispatcher.dispatch(&mut world.res);

                if !window.borrow().running {
                    return Ok(());
                }
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
