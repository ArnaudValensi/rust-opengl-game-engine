use std::collections::HashSet;
use input::keyboard::KeyCode;

#[derive(Default, Debug)]
pub struct Input {
    keys_down: HashSet<KeyCode>,
    keys_down_this_tick: HashSet<KeyCode>,
    keys_up_this_tick: HashSet<KeyCode>,
    cursor_locked: bool,
    mouse_position: (f64, f64),
    // NOTE: Only set if cursor is locked.
    mouse_axis: (f64, f64),
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
            cursor_locked: true,
            mouse_position: (0.0, 0.0),
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

    pub fn set_mouse_position(&mut self, mouse_position: (f64, f64), screen_center: (f64, f64)) {
        self.mouse_position = mouse_position;

        if self.cursor_locked {
            self.mouse_axis = (
                mouse_position.0 - screen_center.0,
                mouse_position.1 - screen_center.1
            );
        }
    }

    pub fn get_mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }

    pub fn get_mouse_axis(&self) -> (f64, f64) {
        self.mouse_axis
    }

    pub fn toggle_cursor_lock(&mut self) {
        self.cursor_locked = !self.cursor_locked;
    }

    pub fn is_cursor_locked(&self) -> bool {
        self.cursor_locked
    }
}
