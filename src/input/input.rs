use std::collections::HashSet;
use input::keyboard::KeyCode;

#[derive(Default, Debug)]
pub struct Input {
    keys_down: HashSet<KeyCode>,
    keys_down_this_tick: HashSet<KeyCode>,
    keys_up_this_tick: HashSet<KeyCode>,
    pub mouse_axis: (f64, f64),
}

impl Input {
    pub fn new() -> Self {
        let keys_down: HashSet<KeyCode> = HashSet::with_capacity(10);
        let keys_down_this_tick: HashSet<KeyCode> = HashSet::with_capacity(10);
        let keys_up_this_tick: HashSet<KeyCode> = HashSet::with_capacity(10);

        Input {
            keys_down,
            keys_down_this_tick,
            keys_up_this_tick,
            mouse_axis: (0.0, 0.0),
        }
    }

    pub fn get_key(&self, key: KeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn get_key_down(&self, key: KeyCode) -> bool {
        self.keys_down_this_tick.contains(&key)
    }

    pub fn get_key_up(&self, key: KeyCode) -> bool {
        self.keys_up_this_tick.contains(&key)
    }

    pub fn set_key_down(&mut self, key: KeyCode) {
        if !self.keys_down.contains(&key) {
            self.keys_down.insert(key);
            self.keys_down_this_tick.insert(key);
        }
    }

    pub fn set_key_up(&mut self, key: KeyCode) {
        self.keys_down.remove(&key);
        self.keys_up_this_tick.insert(key);
    }

    pub fn new_tick(&mut self) {
        self.keys_down_this_tick.clear();
        self.keys_up_this_tick.clear();
        self.mouse_axis = (0.0, 0.0);
    }
}
