use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Debug)]
pub enum MouseEvent {
    Move {
        x: f32,
        y: f32,
    },

    Button {
        button: MouseButton,
        pressed: bool,
    },

    Scroll {
        x: f32,
        y: f32,
    },
}

pub struct Mouse {
    pub x: f32,
    pub y: f32,
    pub visible: bool,
    events: VecDeque<MouseEvent>,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            x: 0.5,
            y: 0.5,
            visible: true,
            events: VecDeque::new(),
        }
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.x = x.clamp(0.0, 1.0);
        self.y = y.clamp(0.0, 1.0);

        self.events.push_back(
            MouseEvent::Move {
                x: self.x,
                y: self.y,
            },
        );
    }

    pub fn click(&mut self, button: MouseButton) {
        self.events.push_back(
            MouseEvent::Button {
                button,
                pressed: true,
            },
        );
    }

    pub fn release(&mut self, button: MouseButton) {
        self.events.push_back(
            MouseEvent::Button {
                button,
                pressed: false,
            },
        );
    }

    pub fn next_event(&mut self) -> Option<MouseEvent> {
        self.events.pop_front()
    }
}
