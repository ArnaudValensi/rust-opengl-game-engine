use specs::{WriteStorage, ReadStorage, System, Join, Read};
use components::transform::Transform;
use components::player::Player;
use input::input::Input;
use input::keyboard::KeyCode;

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

        for (transform, _) in (&mut tranform_storage, &player_storage).join() {
            if input.get_key(KeyCode::D) {
                transform.position.x += 0.02;
            } else if input.get_key(KeyCode::A) {
                transform.position.x -= 0.02;
            }

            if input.get_key(KeyCode::W) {
                transform.position.z -= 0.02;
            } else if input.get_key(KeyCode::S) {
                transform.position.z += 0.02;
            }
        }
    }
}
