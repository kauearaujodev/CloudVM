#[derive(Clone, Copy, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Copy, Debug)]
pub enum MouseAction {
    Move,
    Press,
    Release,
}

pub struct VirtualMouse {
    pub x: f32,
    pub y: f32,
    pub button: Option<MouseButton>,
}

impl VirtualMouse {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            button: None,
        }
    }

    pub fn move_to(
        &mut self,
        x: f32,
        y: f32,
    ) {
        self.x = x;
        self.y = y;
    }

    pub fn press(
        &mut self,
        button: MouseButton,
    ) {
        self.button = Some(button);
    }

    pub fn release(&mut self) {
        self.button = None;
    }

    pub fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
