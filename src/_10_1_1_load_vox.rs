#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate gl;
extern crate glutin;

use common::{Window, process_events};
use input::keyboard::{KeyCode};
use self::glutin::{
    GlContext,
};

use std::os::raw::c_void;
use std::path::Path;
use std::ffi::CStr;
use std::time::{Instant, Duration};
use floating_duration::TimeAsFloat;

use shader::Shader;
use voxel::voxel_mesh_builder::build_mesh;
use mesh::Mesh;

use image;
use image::GenericImage;

use cgmath::{Matrix4, Vector3, vec3,  Deg, perspective, Point3};
use cgmath::prelude::*;

// use vox_loader::VoxLoader;
use errors::*;
use voxel::chunk::Chunk;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;
const FOV: f32 = 45.0;

const cameraUp: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

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

fn run() -> Result<()> {
    let mut cameraPos = Point3::new(0.0, 0.0, 3.0);
    let mut cameraFront: Vector3<f32> = Vector3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    // yaw is initialized to -90.0 degrees since a yaw of 0.0 results in a direction vector
    // pointing to the right so we initially rotate a bit to the left.
    let mut yaw: f32 = -90.0;
    let mut pitch: f32 = 0.0;

    // timing
    let mut deltaTime: Duration; // time between current frame and last frame
    let mut lastFrame = Instant::now();

    // glfw: initialize and configure
    // ------------------------------
    let mut window = Window::new(SCR_WIDTH, SCR_HEIGHT);

    let (ourShader, texture1, texture2) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // build and compile our shader program
        // ------------------------------------
        let ourShader = Shader::new(
            "src/shaders/7.2.camera.vs",
            "src/shaders/7.2.camera.fs");

        // load and create a texture
        // -------------------------
        let (mut texture1, mut texture2) = (0, 0);
        // texture 1
        // ---------
        gl::GenTextures(1, &mut texture1);
        gl::BindTexture(gl::TEXTURE_2D, texture1);
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // load image, create texture and generate mipmaps
        let img = image::open(&Path::new("resources/textures/container.jpg")).expect("Failed to load texture");
        let data = img.raw_pixels();
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGB as i32,
                       img.width() as i32,
                       img.height() as i32,
                       0,
                       gl::RGB,
                       gl::UNSIGNED_BYTE,
                       &data[0] as *const u8 as *const c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);
        // texture 2
        // ---------
        gl::GenTextures(1, &mut texture2);
        gl::BindTexture(gl::TEXTURE_2D, texture2);
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // load image, create texture and generate mipmaps
        let img = image::open(&Path::new("resources/textures/awesomeface.png")).expect("Failed to load texture");
        let img = img.flipv(); // flip loaded texture on the y-axis.
        let data = img.raw_pixels();
        // note that the awesomeface.png has transparency and thus an alpha channel, so make sure to tell OpenGL the data type is of GL_RGBA
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGB as i32,
                       img.width() as i32,
                       img.height() as i32,
                       0,
                       gl::RGBA,
                       gl::UNSIGNED_BYTE,
                       &data[0] as *const u8 as *const c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
        // -------------------------------------------------------------------------------------------
        ourShader.useProgram();
        ourShader.setInt(c_str!("texture1"), 0);
        ourShader.setInt(c_str!("texture2"), 1);

        // pass projection matrix to shader (as projection matrix rarely changes there's no need to do this per frame)
        // -----------------------------------------------------------------------------------------------------------
        let projection: Matrix4<f32> = perspective(Deg(FOV), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
        ourShader.setMat4(c_str!("projection"), &projection);

        (ourShader, texture1, texture2)
    };

    // let chunk = VoxLoader::new();
    let mut chunk = Chunk::new(2, 3, 4);

    chunk.set_voxel(0, 0, 0, 1)?;
    chunk.set_voxel(1, 0, 0, 1)?;

    println!("chunk: {:#?}", chunk);

    let chunk_mesh_data = build_mesh(&chunk);
    let chunk_mesh = Mesh::new(chunk_mesh_data, Vec::default());

    // render loop
    // -----------
    while window.running {
        // per-frame time logic
        // --------------------
        // let currentFrame = glfw.get_time() as f32;
        let currentFrame = Instant::now();
        deltaTime = currentFrame - lastFrame;
        lastFrame = currentFrame;

        // events
        // -----
        process_events(&mut window);

        // input
        // -----
        // processInput(&mut window, deltaTime, &mut cameraPos);
        processInput(
            &mut window,
            deltaTime,
            &mut cameraPos,
            &mut yaw,
            &mut pitch,
            &mut cameraFront,
        );

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            ourShader.useProgram();

            // bind textures on corresponding texture units
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);
            //
            // // activate shader
            // ourShader.useProgram();
            //
            // camera/view transformation
            let view: Matrix4<f32> = Matrix4::look_at(cameraPos, cameraPos + cameraFront, cameraUp);
            ourShader.setMat4(c_str!("view"), &view);

            let model: Matrix4<f32> = Matrix4::from_translation(vec3(0.0, 0.0, 0.0));
            // model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(0.0));
            ourShader.setMat4(c_str!("model"), &model);

            chunk_mesh.Draw(&ourShader);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        // window.swap_buffers();
        // glfw.poll_events();
        window.gl_window.swap_buffers().unwrap();
    }

    // TODO: clean meshes VAO, VBO
    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    // unsafe {
    //     gl::DeleteVertexArrays(1, &VAO);
    //     gl::DeleteBuffers(1, &VBO);
    // }

    Ok(())
}

pub fn main_10_1_1() {
    if let Err(ref e) = run() {
        print_errors_and_exit(e);
    }
}

fn processInput(
    window: &mut Window,
    deltaTime: Duration,
    cameraPos: &mut Point3<f32>,
    yaw: &mut f32,
    pitch: &mut f32,
    cameraFront: &mut Vector3<f32>,
) {
    let input = &window.input;

    if input.get_key(KeyCode::Escape) {
        window.running = false;
    }

    let cameraSpeed: f32 = 2.5 * deltaTime.as_fractional_secs() as f32;

    if input.get_key(KeyCode::W) {
        *cameraPos += cameraSpeed * *cameraFront;
    }
    if input.get_key(KeyCode::S) {
        *cameraPos += -(cameraSpeed * *cameraFront);
    }
    if input.get_key(KeyCode::A) {
        *cameraPos += -(cameraFront.cross(cameraUp).normalize() * cameraSpeed);
    }
    if input.get_key(KeyCode::D) {
        *cameraPos += cameraFront.cross(cameraUp).normalize() * cameraSpeed;
    }

    processMouse(
        &input.mouse_axis,
        yaw,
        pitch,
        cameraFront,
    );
}

fn processMouse(
    mouse_axis: &(f64, f64),
    yaw: &mut f32,
    pitch: &mut f32,
    cameraFront: &mut Vector3<f32>,
) {
    let (xpos, ypos) = (mouse_axis.0 as f32, mouse_axis.1 as f32);

    let sensitivity: f32 = 0.1; // change this value to your liking
    let xoffset = xpos * sensitivity;
    let yoffset = -ypos * sensitivity;

    *yaw += xoffset;
    *pitch += yoffset;

    // make sure that when pitch is out of bounds, screen doesn't get flipped
    if *pitch > 89.0 {
        *pitch = 89.0;
    }
    if *pitch < -89.0 {
        *pitch = -89.0;
    }

    let front = Vector3 {
        x: yaw.to_radians().cos() * pitch.to_radians().cos(),
        y: pitch.to_radians().sin(),
        z: yaw.to_radians().sin() * pitch.to_radians().cos(),
    };
    *cameraFront = front.normalize();
}
