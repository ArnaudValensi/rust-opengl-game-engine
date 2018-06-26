use specs::{WriteStorage, ReadStorage, System, Join, Read};
use cgmath::Vector3;
use cgmath::prelude::*;
use components::transform::Transform;
use components::player::Player;
use input::input::Input;
use input::keyboard::KeyCode;

const CAMERA_SPEED: f32 = 0.02;

pub struct PlayerMovement {

}

impl<'a> System<'a> for PlayerMovement {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Player>,
        Read<'a, Input>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut tranform_storage, player_storage, input) = data;
        let camera_up = Vector3::unit_y();
        // let camera_speed: f32 = 2.5 * deltaTime.as_fractional_secs() as f32;

        for (transform, _) in (&mut tranform_storage, &player_storage).join() {
            if input.get_key(KeyCode::W) {
                transform.position += CAMERA_SPEED * transform.forward;
            }
            if input.get_key(KeyCode::S) {
                transform.position += -(CAMERA_SPEED * transform.forward);
            }
            if input.get_key(KeyCode::A) {
                transform.position += -(transform.forward.cross(camera_up).normalize() * CAMERA_SPEED);
            }
            if input.get_key(KeyCode::D) {
                transform.position += transform.forward.cross(camera_up).normalize() * CAMERA_SPEED;
            }
        }
    }
}
