// TODO: use delta time
use specs::{WriteStorage, ReadStorage, System, Join, Read};
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
    yaw: f32,
}

impl PlayerMovement {
    pub fn new() -> Self {
        Self {
            pitch: 0.0,
            yaw: 180.0,
        }
    }

    fn process_position(&self, input: &Input, transform: &mut Transform) {
        let camera_forward = transform.forward();
        let camera_left = transform.left();

        if input.get_key(KeyCode::W) {
            transform.add_local_position(CAMERA_SPEED * camera_forward);
        }
        if input.get_key(KeyCode::S) {
            transform.add_local_position(-(CAMERA_SPEED * camera_forward));
        }
        if input.get_key(KeyCode::A) {
            transform.add_local_position(CAMERA_SPEED * camera_left);
        }
        if input.get_key(KeyCode::D) {
            transform.add_local_position(-(CAMERA_SPEED * camera_left));
        }
    }

    fn process_rotation(
        &mut self,
        mouse_axis: (f64, f64),
        transform: &mut Transform,
    ) {
        let (xpos, ypos) = (mouse_axis.0 as f32, mouse_axis.1 as f32);

        self.yaw = (self.yaw - xpos * SENSITIVITY) % 360.0;

        if self.yaw < 0.0 {
            self.yaw += 360.0;
        }

        self.pitch += ypos * SENSITIVITY;
        self.pitch = clamp(self.pitch, MIN_Y, MAX_Y);

        transform.set_rotation(self.pitch, self.yaw, 0.0);
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
            self.process_position(&input, transform);
            self.process_rotation(input.get_mouse_axis(), &mut transform);
        }
    }
}
