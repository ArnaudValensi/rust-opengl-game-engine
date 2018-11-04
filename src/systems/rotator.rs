use specs::{WriteStorage, WriteExpect, System, Read};
use resources::rotating_entity::RotatingEntity;
use components::transform::Transform;
use resources::Time;

const SENSITIVITY: f32 = 5.0;

pub struct Rotator {
    yaw: f32,
}

impl Default for Rotator {
    fn default() -> Self {
        Self {
            yaw: 0.0,
        }
    }
}

impl Rotator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> System<'a> for Rotator {
    type SystemData = (
        WriteExpect<'a, RotatingEntity>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (active_camera, mut tranform_storage, time) = data;
        let transform = tranform_storage.get_mut(active_camera.0).unwrap();
        let delta_time_in_seconds = time.get_delta_time_in_seconds();

        self.yaw = (self.yaw + SENSITIVITY * delta_time_in_seconds) % 360.0;

        if self.yaw < 0.0 {
            self.yaw += 360.0;
        }

        transform.set_rotation(0.0, self.yaw, 0.0);
    }
}
