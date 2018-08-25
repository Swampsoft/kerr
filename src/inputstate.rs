use std::fmt;
use std::mem::transmute;
use std::collections::HashMap;

use sdl2::{
    event::Event, keyboard::{Keycode, Scancode},
};

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,

    Fire,
}

pub struct InputState {
    inputs: [bool; 256],
    mapping: HashMap<Scancode, Input>,
}

impl Default for InputState {
    fn default() -> Self {
        let mut mapping = HashMap::new();
        mapping.insert(Scancode::Up, Input::Up);
        mapping.insert(Scancode::Down, Input::Down);
        mapping.insert(Scancode::Left, Input::Left);
        mapping.insert(Scancode::Right, Input::Right);

        mapping.insert(Scancode::W, Input::Up);
        mapping.insert(Scancode::S, Input::Down);
        mapping.insert(Scancode::A, Input::Left);
        mapping.insert(Scancode::D, Input::Right);

        mapping.insert(Scancode::Space, Input::Fire);

        InputState {
            inputs: [false; 256],
            mapping,
        }
    }
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown { scancode, .. } => {
                scancode.map(|c| self.set(c));
            }
            Event::KeyUp { scancode, .. } => {
                scancode.map(|c| self.unset(c));
            }
            _ => {}
        }
    }

    pub fn is_set(&self, i: Input) -> bool {
        self.inputs[i as usize]
    }

    pub fn set(&mut self, code: Scancode) {
        if let Some(i) = self.mapping.get(&code) {
            self.inputs[*i as usize] = true;
        }
    }

    pub fn unset(&mut self, code: Scancode) {
        if let Some(i) = self.mapping.get(&code) {
            self.inputs[*i as usize] = false;
        }
    }
}
