// TODO: use quaternion in Transform
// TODO: use delta time
use specs::{WriteStorage, ReadStorage, System, Join, Read};
use cgmath::{Vector3};
use cgmath::prelude::*;
use components::transform::Transform;
use components::player::Player;
use input::input::Input;
use input::keyboard::KeyCode;

const CAMERA_SPEED: f32 = 0.02;

pub struct PlayerMovement {
    pitch: f32,
}

impl PlayerMovement {
    pub fn new() -> Self {
        Self {
            pitch: 0.0,
        }
    }
}

impl<'a> System<'a> for PlayerMovement {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Player>,
        Read<'a, Input>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut tranform_storage, player_storage, input) = data;
        // let camera_speed: f32 = 2.5 * deltaTime.as_fractional_secs() as f32;

        for (mut transform, _) in (&mut tranform_storage, &player_storage).join() {

            process_position(&input, transform);
            process_rotation(
                &input.mouse_axis,
                &mut self.pitch,
                &mut transform,
            );
        }
    }
}

fn process_position(input: &Input, transform: &mut Transform) {
    let camera_forward = transform.forward();
    let camera_right = transform.right();

    if input.get_key(KeyCode::W) {
        transform.position += CAMERA_SPEED * camera_forward;
    }
    if input.get_key(KeyCode::S) {
        transform.position += -(CAMERA_SPEED * camera_forward);
    }
    if input.get_key(KeyCode::A) {
        transform.position += -(CAMERA_SPEED * camera_right);
    }
    if input.get_key(KeyCode::D) {
        transform.position += CAMERA_SPEED * camera_right;
    }
}

fn process_rotation(
    mouse_axis: &(f64, f64),
    pitch: &mut f32,
    transform: &mut Transform,
) {
    let (xpos, ypos) = (mouse_axis.0 as f32, mouse_axis.1 as f32);

    let sensitivity: f32 = 0.1;

    let xoffset = -xpos * sensitivity;
    let yoffset = -ypos * sensitivity;

    let rotation = Vector3::unit_y() * xoffset;
    transform.rotate(rotation);

    // *pitch += yoffset;
    //
    // // make sure that when pitch is out of bounds, screen doesn't get flipped
    // if *pitch > 89.0 {
    //     *pitch = 89.0;
    // }
    // if *pitch < -89.0 {
    //     *pitch = -89.0;
    // }
    //
    // transform.rotation = Quaternion::from(Euler {
    //     x: Deg(*pitch),
    //     y: Deg(*yaw),
    //     z: Deg(0.0),
    // });
}
