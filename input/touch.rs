#[derive(Clone, Copy, Debug)]
pub enum TouchAction {
    Down,
    Move,
    Up,
}

#[derive(Clone, Copy, Debug)]
pub struct TouchEvent {
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub action: TouchAction,
}

impl TouchEvent {
    pub fn new(
        id: u64,
        x: f32,
        y: f32,
        action: TouchAction,
    ) -> Self {
        Self {
            id,
            x,
            y,
            action,
        }
    }
}
