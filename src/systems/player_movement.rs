use specs::{WriteStorage, ReadStorage, System, Join, Read};
use cgmath::Vector3;
use cgmath::prelude::*;
use components::transform::Transform;
use components::player::Player;
use input::input::Input;
use input::keyboard::KeyCode;

const CAMERA_SPEED: f32 = 0.02;

pub struct PlayerMovement {
    yaw: f32,
    pitch: f32,
}

impl PlayerMovement {
    pub fn new() -> Self {
        Self {
            // yaw is initialized to -90.0 degrees since a yaw of 0.0 results in a direction vector
            // pointing to the right so we initially rotate a bit to the left.
            yaw: -90.0,
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

        for (transform, _) in (&mut tranform_storage, &player_storage).join() {

            process_position(&input, transform);
            process_rotation(
                &input.mouse_axis,
                &mut self.yaw,
                &mut self.pitch,
                &mut transform.forward,
            );
        }
    }
}

fn process_position(input: &Input, transform: &mut Transform) {
    let camera_forward = transform.forward;
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
    yaw: &mut f32,
    pitch: &mut f32,
    camera_front: &mut Vector3<f32>,
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
    *camera_front = front.normalize();
}
