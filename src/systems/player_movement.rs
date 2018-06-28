// TODO: use quaternion in Transform
// TODO: use delta time
use specs::{WriteStorage, ReadStorage, System, Join, Read};
use cgmath::Vector3;
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
            process_rotation(&input.mouse_axis, &mut transform);
        }
    }
}

fn process_position(input: &Input, transform: &mut Transform) {
    let camera_forward = transform.forward();
    let camera_left = transform.right();

    if input.get_key(KeyCode::W) {
        transform.position += CAMERA_SPEED * camera_forward;
    }
    if input.get_key(KeyCode::S) {
        transform.position += -(CAMERA_SPEED * camera_forward);
    }
    if input.get_key(KeyCode::A) {
        transform.position += -(CAMERA_SPEED * camera_left);
    }
    if input.get_key(KeyCode::D) {
        transform.position += CAMERA_SPEED * camera_left;
    }
}

fn process_rotation(
    mouse_axis: &(f64, f64),
    transform: &mut Transform,
) {
    let (xpos, ypos) = (mouse_axis.0 as f32, mouse_axis.1 as f32);
    let sensitivity: f32 = 0.1;
    let yaw_offset = -xpos * sensitivity;
    let pitch_offset = -ypos * sensitivity;

    let rotation = transform.right() * pitch_offset + Vector3::unit_y() * yaw_offset;
    transform.rotate(rotation);
}
