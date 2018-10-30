use std::time::{Duration, Instant};

const SMOOTHING: f32 = 0.98;

pub struct Time {
    last_frame: Instant,
    delta_time: Duration,
    delta_time_in_seconds: f32,
    average_delta_time_in_seconds: f32,
}

impl Time {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self) {
        let now = Instant::now();

        self.delta_time = now - self.last_frame;
        self.delta_time_in_seconds = self.delta_time.as_secs() as f32
            + self.delta_time.subsec_nanos() as f32 / 1_000_000_000.0;
        self.average_delta_time_in_seconds = self.average_delta_time_in_seconds * SMOOTHING
            + self.delta_time_in_seconds * (1.0 - SMOOTHING);
        self.last_frame = now;
    }

    pub fn get_delta_time(&self) -> Duration {
        self.delta_time
    }

    pub fn get_delta_time_in_seconds(&self) -> f32 {
        self.delta_time_in_seconds
    }

    pub fn get_average_delta_time_in_seconds(&self) -> f32 {
        self.average_delta_time_in_seconds
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            last_frame: Instant::now(),
            delta_time: Duration::default(),
            delta_time_in_seconds: 0.0,
            average_delta_time_in_seconds: 0.0,
        }
    }
}
