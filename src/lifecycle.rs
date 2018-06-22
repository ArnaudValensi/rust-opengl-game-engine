use std::time::{Instant, Duration};
use std::cmp;
use std::thread::sleep;

static BILLION: u64 = 1_000_000_000;
const DEFAULT_FIXED_UPDATE_PER_SECOND: u64 = 120;
const DEFAULT_UPDATE_PER_SECOND: u64 = 60;

fn ns_to_duration(ns: u64) -> Duration {
    let secs = ns / BILLION;
    let nanos = (ns % BILLION) as u32;
    Duration::new(secs, nanos)
}

#[derive(Debug)]
pub enum Event {
    FixedUpdate,
    OnInput,
    Update,
    Render,
}

enum State {
    Schedule,
    FixedUpdate,
    OnInput,
    Update,
    Render,
}

pub struct Lifecycle {
    state: State,
    last_fixed_update_time: Instant,
    last_update_time: Instant,
    fixed_delta_time: Duration,
    delta_time: Duration,
}

// TODO:
//   - pass return real delta time
impl Lifecycle {
    pub fn new() -> Lifecycle {
        println!("new Lifecycle");

        let start = Instant::now();

        Lifecycle {
            state: State::FixedUpdate,
            last_fixed_update_time: start,
            last_update_time: start,
            fixed_delta_time: ns_to_duration(BILLION / DEFAULT_FIXED_UPDATE_PER_SECOND),
            delta_time: ns_to_duration(BILLION / DEFAULT_UPDATE_PER_SECOND),
        }
    }

    pub fn next(&mut self) -> Option<Event> {
        loop {
            self.state = match self.state {
                State::Schedule => {
                    let current_time = Instant::now();
                    let next_frame = self.last_update_time + self.delta_time;
                    let next_update = self.last_fixed_update_time + self.fixed_delta_time;
                    let next_event = cmp::min(next_frame, next_update);

                    if next_event > current_time {
                        sleep(next_event - current_time);
                        State::Schedule
                    } else if next_event == next_frame {
                        State::OnInput
                    } else {
                        State::FixedUpdate
                    }
                }
                State::FixedUpdate => {
                    let current_time = Instant::now();
                    // let delta_time = current_time - self.last_fixed_update_time;

                    self.last_fixed_update_time = current_time;
                    self.state = State::Schedule;
                    return Some(Event::FixedUpdate);
                }
                State::OnInput => {
                    self.state = State::Update;
                    return Some(Event::OnInput);
                }
                State::Update => {
                    let current_time = Instant::now();
                    // let delta_time = current_time - self.last_update_time;

                    self.last_update_time = current_time;
                    self.state = State::Render;
                    return Some(Event::Update);
                }
                State::Render => {
                    self.state = State::Schedule;
                    return Some(Event::Render);
                }
            };
        }
    }
}
