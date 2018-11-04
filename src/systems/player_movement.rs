use components::player::Player;
use components::transform::Transform;
use input::keyboard::KeyCode;
use input::Input;
use math::clamp;
use specs::{Join, Read, ReadStorage, System, WriteStorage};
use resources::Time;

const CAMERA_SPEED: f32 = 20.0;
const SENSITIVITY: f32 = 10.0;
const MIN_Y: f32 = -60.0;
const MAX_Y: f32 = 60.0;

pub struct PlayerMovement {
    pitch: f32,
    yaw: f32,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 180.0,
        }
    }
}

impl PlayerMovement {
    pub fn new() -> Self {
        Self::default()
    }

    fn process_position(&self, input: &Input, transform: &mut Transform, delta_time: f32) {
        let camera_forward = transform.forward();
        let camera_left = transform.left();

        if input.get_key(KeyCode::W) {
            transform.add_to_local_position(CAMERA_SPEED * delta_time * camera_forward);
        }
        if input.get_key(KeyCode::S) {
            transform.add_to_local_position(-(CAMERA_SPEED * delta_time * camera_forward));
        }
        if input.get_key(KeyCode::A) {
            transform.add_to_local_position(CAMERA_SPEED * delta_time * camera_left);
        }
        if input.get_key(KeyCode::D) {
            transform.add_to_local_position(-(CAMERA_SPEED * delta_time * camera_left));
        }
    }

    fn process_rotation(
        &mut self,
        mouse_axis: (f64, f64),
        transform: &mut Transform,
        delta_time: f32,
    ) {
        let (xpos, ypos) = (mouse_axis.0 as f32, mouse_axis.1 as f32);

        self.yaw = (self.yaw - xpos * SENSITIVITY * delta_time) % 360.0;

        if self.yaw < 0.0 {
            self.yaw += 360.0;
        }

        self.pitch += ypos * SENSITIVITY * delta_time;
        self.pitch = clamp(self.pitch, MIN_Y, MAX_Y);

        transform.set_rotation(self.pitch, self.yaw, 0.0);
    }
}

impl<'a> System<'a> for PlayerMovement {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Player>,
        Read<'a, Input>,
        Read<'a, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut tranform_storage, player_storage, input, time) = data;
        let delta_time = time.get_delta_time_in_seconds();

        for (mut transform, _) in (&mut tranform_storage, &player_storage).join() {
            self.process_position(&input, transform, delta_time);
            self.process_rotation(input.get_mouse_axis(), &mut transform, delta_time);
        }
    }
}
