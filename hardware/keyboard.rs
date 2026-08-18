use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct KeyEvent {
    pub key: String,
    pub pressed: bool,
}

pub struct Keyboard {
    pub connected: bool,
    events: VecDeque<KeyEvent>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            connected: true,
            events: VecDeque::new(),
        }
    }

    pub fn press(&mut self, key: String) {
        self.events.push_back(KeyEvent {
            key,
            pressed: true,
        });
    }

    pub fn release(&mut self, key: String) {
        self.events.push_back(KeyEvent {
            key,
            pressed: false,
        });
    }

    pub fn next_event(&mut self) -> Option<KeyEvent> {
        self.events.pop_front()
    }
}
