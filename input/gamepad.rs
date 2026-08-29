#[derive(Clone, Copy, Debug)]
pub enum GamepadButton {
    Cross,
    Circle,
    Square,
    Triangle,

    L1,
    R1,
    L2,
    R2,

    L3,
    R3,

    Share,
    Options,

    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,

    Home,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Stick {
    pub x: f32,
    pub y: f32,
}

impl Stick {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn set(
        &mut self,
        x: f32,
        y: f32,
    ) {
        self.x = x.clamp(-1.0, 1.0);
        self.y = y.clamp(-1.0, 1.0);
    }

    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
}

pub struct Gamepad {
    pub left_stick: Stick,
    pub right_stick: Stick,

    pub left_trigger: f32,
    pub right_trigger: f32,

    buttons: Vec<GamepadButton>,
}

impl Gamepad {
    pub fn new() -> Self {
        Self {
            left_stick: Stick::new(),
            right_stick: Stick::new(),

            left_trigger: 0.0,
            right_trigger: 0.0,

            buttons: Vec::new(),
        }
    }

    pub fn press(
        &mut self,
        button: GamepadButton,
    ) {
        if !self.buttons.contains(&button) {
            self.buttons.push(button);
        }
    }

    pub fn release(
        &mut self,
        button: GamepadButton,
    ) {
        self.buttons.retain(|b| *b != button);
    }

    pub fn is_pressed(
        &self,
        button: GamepadButton,
    ) -> bool {
        self.buttons.contains(&button)
    }

    pub fn set_left_stick(
        &mut self,
        x: f32,
        y: f32,
    ) {
        self.left_stick.set(x, y);
    }

    pub fn set_right_stick(
        &mut self,
        x: f32,
        y: f32,
    ) {
        self.right_stick.set(x, y);
    }

    pub fn set_triggers(
        &mut self,
        left: f32,
        right: f32,
    ) {
        self.left_trigger = left.clamp(0.0, 1.0);
        self.right_trigger = right.clamp(0.0, 1.0);
    }

    pub fn reset(&mut self) {
        self.left_stick.reset();
        self.right_stick.reset();

        self.left_trigger = 0.0;
        self.right_trigger = 0.0;

        self.buttons.clear();
    }
}
