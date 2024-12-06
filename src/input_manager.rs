use piston_window::input::{Key, PressEvent, ReleaseEvent};
use piston_window::{Button, Event};
use std::collections::HashSet;

pub struct InputManager {
    keys: std::collections::HashSet<Key>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keys: HashSet::new(),
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        self.keys.insert(key);
    }

    pub fn key_released(&mut self, key: Key) {
        self.keys.remove(&key);
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys.contains(&key)
    }

    pub fn handle_event(&mut self, event: &Event) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            self.key_pressed(key);
        }
        if let Some(Button::Keyboard(key)) = event.release_args() {
            self.key_released(key);
        }
    }
}
