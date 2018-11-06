extern crate gl;
extern crate glutin;

use cgmath::{perspective, Deg, Matrix4, Point3, Vector3};
use components::camera::Camera;
use components::mesh_render::MeshRender;
use components::parent::Parent;
use components::player::Player;
use components::transform::Transform;
use errors::print_errors_and_exit;
use failure::Error;
use resources::active_camera::ActiveCamera;
use resources::rotating_entity::RotatingEntity;
use resources::Time;
use specs::{Builder, DispatcherBuilder, World};
use systems::gui_rendering::GuiRendering;
use systems::mouse_control::MouseControl;
use systems::player_movement::PlayerMovement;
use systems::render::Render;
use systems::transformation::Transformation;
use systems::window_event::WindowEvent;
use systems::AfterRender;
use systems::Rotator;
use config::{SCR_HEIGHT, SCR_WIDTH};
use input::Input;
use lifecycle::{Event, Lifecycle};
use material::Material;
use mesh::Mesh;
use palette::Palette;
use std::cell::RefCell;
use std::rc::Rc;
use vox_loader::VoxLoader;
use voxel::chunk::Chunk;
use voxel::voxel_mesh_builder::build_mesh;
use voxel::Terrain;
use window::Window;

// settings
const FOV: f32 = 45.0;

fn run() -> Result<(), Error> {
    info!(" 🦄 Starting BigSeed...");

    let window = Rc::new(RefCell::new(Window::new(SCR_WIDTH, SCR_HEIGHT)));
    let render_system = Render::new();
    let window_event_system = WindowEvent::new(Rc::clone(&window));
    let mouse_control_system = MouseControl::new(Rc::clone(&window));
    let player_movement_system = PlayerMovement::new();
    let gui_rendering_system = GuiRendering::new(Rc::clone(&window));
    let after_render_system = AfterRender::new(Rc::clone(&window));
    let input = Input::new();
    let time = Time::new();
    let mut terrain = Terrain::new();
    let material = Material::new();
    let projection: Matrix4<f32> =
        perspective(Deg(FOV), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
    let palette: Vec<f32> = Palette::get();
    let mut event_loop = Lifecycle::new();

    terrain.generate();

    unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);
        material.set_vector4_array("palette", &palette);
        material.set_matrix4("projection", &projection);
        material.set_vector3("lightPosition", &Vector3::<f32> { x: 5.0, y: 0.0, z: 0.0 });
        material.set_vector3("lightColor", &Vector3::<f32>::new(1.0, 1.0, 1.0 ));
        material.set_vector3("ambientLightColor", &Vector3::<f32>::new(1.0, 1.0, 1.0 ));
    }

    let mut chunk = Chunk::new(2, 3, 4);
    let mut chunk2 = Chunk::new(2, 2, 2);
    let chunk3 = VoxLoader::load("chr_old.vox")?;

    chunk.set_voxel(0, 0, 0, 2)?;
    chunk.set_voxel(1, 0, 0, 2)?;
    chunk.set_voxel(1, 0, 1, 2)?;
    chunk2.set_voxel(0, 0, 0, 1)?;

    let chunk_mesh_data = build_mesh(&chunk);
    let chunk_mesh = Mesh::new(chunk_mesh_data, Vec::default());
    let chunk_mesh_data2 = build_mesh(&chunk2);
    let chunk_mesh2 = Mesh::new(chunk_mesh_data2, Vec::default());
    let chunk_mesh_data3 = build_mesh(&chunk3);
    let chunk_mesh3 = Mesh::new(chunk_mesh_data3, Vec::default());

    let mut world = World::new();

    world.register::<Transform>();
    world.register::<MeshRender>();
    world.register::<Camera>();
    world.register::<Player>();
    world.register::<Parent>();

    world.add_resource(time);
    world.add_resource(input);
    // world.add_resource(terrain);

    let scene_root_entity = world.create_entity().build();
    let transformation_system = Transformation::new(scene_root_entity);

    let camera_entity = world
        .create_entity()
        .with(Transform::new(Point3::new(0.0, 0.0, 0.0), "Camera"))
        .with(Camera)
        .with(Player)
        .build();
    world.add_resource(ActiveCamera(camera_entity));

    let mut chunk_transform = Transform::new(Point3::new(0.0, 0.0, -1.0), "Chunk0");
    chunk_transform.set_rotation(0.0, 45.0, 0.0);
    let chunk0 = world
        .create_entity()
        .with(chunk_transform)
        .with(MeshRender {
            material: material.clone(),
            mesh: chunk_mesh,
        })
        .build();
    world.add_resource(RotatingEntity(chunk0));

    world
        .create_entity()
        .with(Parent { entity: chunk0 })
        .with(Transform::new(Point3::new(0.0, 0.0, -2.0), "Chunk1"))
        .with(MeshRender {
            material: material.clone(),
            mesh: chunk_mesh2,
        })
        .build();

    world
        .create_entity()
        .with(Transform::new(Point3::new(0.0, 0.0, 0.0), "Chunk2"))
        .with(MeshRender {
            material,
            mesh: chunk_mesh3,
        })
        .build();

    let mut dispatcher_builder = DispatcherBuilder::new();

    dispatcher_builder.add_thread_local(window_event_system);
    dispatcher_builder.add_thread_local(mouse_control_system);
    dispatcher_builder.add_thread_local(player_movement_system);
    dispatcher_builder.add_thread_local(Rotator::new());
    dispatcher_builder.add_thread_local(transformation_system);
    dispatcher_builder.add_thread_local(render_system);
    dispatcher_builder.add_thread_local(gui_rendering_system);
    dispatcher_builder.add_thread_local(after_render_system);

    let mut dispatcher = dispatcher_builder.build();

    // TODO: Simplify event loop.
    while let Some(event) = event_loop.next() {
        match event {
            Event::FixedUpdate => {}
            Event::OnInput => {}
            Event::Update => {
                {
                    let mut time = world.write_resource::<Time>();
                    (*time).update();
                }

                dispatcher.dispatch(&world.res);

                if !window.borrow().running {
                    return Ok(());
                }
            }
            Event::Render => {}
        }
    }

    Ok(())
}

pub fn game() {
    if let Err(ref e) = run() {
        print_errors_and_exit(e);
    }
}
