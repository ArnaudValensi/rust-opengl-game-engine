// TODO: use quaternion in Transform
// TODO: use delta time
use specs::{WriteStorage, ReadStorage, System, Join, Read};
use cgmath::{Vector3, Quaternion, Euler, Deg};
use cgmath::prelude::*;
use components::transform::Transform;
use components::player::Player;
use input::input::Input;
use input::keyboard::KeyCode;
use math::clamp;

const CAMERA_SPEED: f32 = 0.02;
const SENSITIVITY: f32 = 0.1;
const MIN_Y: f32 = -60.0;
const MAX_Y: f32 = 60.0;

pub struct PlayerMovement {
    pitch: f32,
    // rotation_y: f32,
}

impl PlayerMovement {
    pub fn new() -> Self {
        Self {
            pitch: 0.0,
            // rotation_y: 0.0,
        }
    }

    fn process_position(&self, input: &Input, transform: &mut Transform) {
        let camera_forward = transform.forward();
        let camera_left = transform.left();

        if input.get_key(KeyCode::W) {
            transform.position += CAMERA_SPEED * camera_forward;
        }
        if input.get_key(KeyCode::S) {
            transform.position += -(CAMERA_SPEED * camera_forward);
        }
        if input.get_key(KeyCode::A) {
            transform.position += CAMERA_SPEED * camera_left;
        }
        if input.get_key(KeyCode::D) {
            transform.position += -(CAMERA_SPEED * camera_left);
        }
    }

    fn process_rotation(
        &mut self,
        mouse_axis: &(f64, f64),
        transform: &mut Transform,
    ) {
        let (xpos, ypos) = (mouse_axis.0 as f32, mouse_axis.1 as f32);
        // let sensitivity: f32 = 0.1;
        // let yaw_offset = -xpos * sensitivity;
        // let pitch_offset = ypos * sensitivity;
        //
        // let rotation = transform.left() * pitch_offset + Vector3::unit_y() * yaw_offset;
        // transform.rotate(rotation);

        // let euler_angles = transform.get_local_euler_angles();
        // println!("euler_angles: {:#?}", euler_angles);

        let euler_angles = transform.get_local_euler_angles();

        println!("euler_angles: {:#?}", euler_angles);

        let yaw = euler_angles.y + xpos * SENSITIVITY;

        self.pitch -= ypos * SENSITIVITY;
        self.pitch = clamp(self.pitch, MIN_Y, MAX_Y);

        transform.rotation = Quaternion::from(Euler {
            x: Deg(self.pitch),
            y: Deg(yaw),
            z: Deg(0.0),
        });
    }
}

// if (axes == RotationAxes.MouseXAndY) {
//     float rotationX = transform.localEulerAngles.y + Input.GetAxis("Mouse X") * sensitivityX;
//
//     rotationY += Input.GetAxis("Mouse Y") * sensitivityY;
//     rotationY = Mathf.Clamp(rotationY, minimumY, maximumY);
//
//     transform.localEulerAngles = new Vector3(-rotationY, rotationX, 0);
// } else if (axes == RotationAxes.MouseX) {
//     transform.Rotate(0, Input.GetAxis("Mouse X") * sensitivityX, 0);
// } else {
//     rotationY += Input.GetAxis("Mouse Y") * sensitivityY;
//     rotationY = Mathf.Clamp(rotationY, minimumY, maximumY);
//
//     transform.localEulerAngles = new Vector3(-rotationY, transform.localEulerAngles.y, 0);
// }

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
            // self.process_position(&input, transform);
            self.process_rotation(&input.mouse_axis, &mut transform);
        }
    }
}
