// ============================================================
// CONTROLE VIRTUAL
// Estilo controle de console para uso na tela
// ============================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Button {
    Cross,
    Circle,
    Square,
    Triangle,

    L1,
    L2,
    R1,
    R2,

    L3,
    R3,

    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,

    Options,
    Share,

    PS,
    Touchpad,
}

// ============================================================
// ANALÓGICO
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Stick {
    pub x: f32,
    pub y: f32,

    pub pressed: bool,

    pub active: bool,
}

impl Stick {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            pressed: false,
            active: false,
        }
    }

    pub fn set(
        &mut self,
        x: f32,
        y: f32,
    ) {
        self.x = x.clamp(-1.0, 1.0);
        self.y = y.clamp(-1.0, 1.0);
        self.active = true;
    }

    pub fn press(&mut self) {
        self.pressed = true;
    }

    pub fn release(&mut self) {
        self.pressed = false;
    }

    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.pressed = false;
        self.active = false;
    }
}

// ============================================================
// GATILHO
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct Trigger {
    pub value: f32,
}

impl Trigger {
    pub fn new() -> Self {
        Self {
            value: 0.0,
        }
    }

    pub fn set(
        &mut self,
        value: f32,
    ) {
        self.value =
            value.clamp(0.0, 1.0);
    }

    pub fn reset(&mut self) {
        self.value = 0.0;
    }
}

// ============================================================
// TOUCHPAD
// ============================================================

#[derive(Clone, Copy, Debug)]
pub struct TouchPoint {
    pub x: f32,
    pub y: f32,
    pub active: bool,
}

impl TouchPoint {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            active: false,
        }
    }
}

// ============================================================
// EVENTO DO CONTROLE
// ============================================================

#[derive(Clone, Debug)]
pub enum ControllerEvent {

    ButtonPressed(Button),

    ButtonReleased(Button),

    LeftStick {
        x: f32,
        y: f32,
    },

    RightStick {
        x: f32,
        y: f32,
    },

    L2(f32),

    R2(f32),

    Touch {
        finger: u8,
        x: f32,
        y: f32,
    },

    TouchReleased {
        finger: u8,
    },
}

// ============================================================
// CONTROLE
// ============================================================

pub struct Controller {
    pub connected: bool,

    pub left_stick: Stick,

    pub right_stick: Stick,

    pub l2: Trigger,

    pub r2: Trigger,

    pub touch_1: TouchPoint,

    pub touch_2: TouchPoint,

    buttons: u32,

    pub vibration_low: f32,

    pub vibration_high: f32,

    pub lightbar_r: u8,

    pub lightbar_g: u8,

    pub lightbar_b: u8,

    events: Vec<ControllerEvent>,
}

impl Controller {

    // ========================================================
    // CRIAR CONTROLE
    // ========================================================

    pub fn new() -> Self {
        Self {
            connected: true,

            left_stick:
                Stick::new(),

            right_stick:
                Stick::new(),

            l2:
                Trigger::new(),

            r2:
                Trigger::new(),

            touch_1:
                TouchPoint::new(),

            touch_2:
                TouchPoint::new(),

            buttons: 0,

            vibration_low: 0.0,

            vibration_high: 0.0,

            lightbar_r: 0,

            lightbar_g: 100,

            lightbar_b: 255,

            events:
                Vec::new(),
        }
    }

    // ========================================================
    // BOTÕES
    // ========================================================

    fn button_bit(
        button: Button,
    ) -> u32 {

        match button {

            Button::Cross =>
                1 << 0,

            Button::Circle =>
                1 << 1,

            Button::Square =>
                1 << 2,

            Button::Triangle =>
                1 << 3,

            Button::L1 =>
                1 << 4,

            Button::R1 =>
                1 << 5,

            Button::L3 =>
                1 << 6,

            Button::R3 =>
                1 << 7,

            Button::DPadUp =>
                1 << 8,

            Button::DPadDown =>
                1 << 9,

            Button::DPadLeft =>
                1 << 10,

            Button::DPadRight =>
                1 << 11,

            Button::Options =>
                1 << 12,

            Button::Share =>
                1 << 13,

            Button::PS =>
                1 << 14,

            Button::Touchpad =>
                1 << 15,

            Button::L2 =>
                1 << 16,

            Button::R2 =>
                1 << 17,
        }
    }

    pub fn press(
        &mut self,
        button: Button,
    ) {

        self.buttons |=
            Self::button_bit(button);

        self.events.push(
            ControllerEvent::
                ButtonPressed(button)
        );
    }

    pub fn release(
        &mut self,
        button: Button,
    ) {

        self.buttons &=
            !Self::button_bit(button);

        self.events.push(
            ControllerEvent::
                ButtonReleased(button)
        );
    }

    pub fn is_pressed(
        &self,
        button: Button,
    ) -> bool {

        self.buttons &
            Self::button_bit(button)
            != 0
    }

    // ========================================================
    // ANALÓGICO ESQUERDO
    // ========================================================

    pub fn left_stick(
        &mut self,
        x: f32,
        y: f32,
    ) {

        self.left_stick.set(
            x,
            y,
        );

        self.events.push(
            ControllerEvent::LeftStick {
                x: self.left_stick.x,
                y: self.left_stick.y,
            }
        );
    }

    // ========================================================
    // ANALÓGICO DIREITO
    // ========================================================

    pub fn right_stick(
        &mut self,
        x: f32,
        y: f32,
    ) {

        self.right_stick.set(
            x,
            y,
        );

        self.events.push(
            ControllerEvent::RightStick {
                x: self.right_stick.x,
                y: self.right_stick.y,
            }
        );
    }

    // ========================================================
    // L2
    // ========================================================

    pub fn set_l2(
        &mut self,
        value: f32,
    ) {

        self.l2.set(value);

        self.events.push(
            ControllerEvent::L2(
                self.l2.value
            )
        );
    }

    // ========================================================
    // R2
    // ========================================================

    pub fn set_r2(
        &mut self,
        value: f32,
    ) {

        self.r2.set(value);

        self.events.push(
            ControllerEvent::R2(
                self.r2.value
            )
        );
    }

    // ========================================================
    // TOUCHPAD
    // ========================================================

    pub fn touch(
        &mut self,
        finger: u8,
        x: f32,
        y: f32,
    ) {

        let point =
            TouchPoint {
                x: x.clamp(
                    0.0,
                    1.0
                ),

                y: y.clamp(
                    0.0,
                    1.0
                ),

                active: true,
            };

        match finger {

            0 => {
                self.touch_1 =
                    point;
            }

            1 => {
                self.touch_2 =
                    point;
            }

            _ => return,
        }

        self.events.push(
            ControllerEvent::Touch {
                finger,
                x: point.x,
                y: point.y,
            }
        );
    }

    pub fn release_touch(
        &mut self,
        finger: u8,
    ) {

        match finger {

            0 => {
                self.touch_1.active =
                    false;
            }

            1 => {
                self.touch_2.active =
                    false;
            }

            _ => return,
        }

        self.events.push(
            ControllerEvent::
                TouchReleased {
                    finger
                }
        );
    }

    // ========================================================
    // VIBRAÇÃO
    // ========================================================

    pub fn vibration(
        &mut self,
        low: f32,
        high: f32,
    ) {

        self.vibration_low =
            low.clamp(
                0.0,
                1.0
            );

        self.vibration_high =
            high.clamp(
                0.0,
                1.0
            );
    }

    pub fn stop_vibration(
        &mut self,
    ) {

        self.vibration_low =
            0.0;

        self.vibration_high =
            0.0;
    }

    // ========================================================
    // LIGHTBAR
    // ========================================================

    pub fn lightbar(
        &mut self,
        r: u8,
        g: u8,
        b: u8,
    ) {

        self.lightbar_r = r;

        self.lightbar_g = g;

        self.lightbar_b = b;
    }

    // ========================================================
    // LER EVENTOS
    // ========================================================

    pub fn events(
        &mut self,
    ) -> Vec<ControllerEvent> {

        std::mem::take(
            &mut self.events
        )
    }

    // ========================================================
    // RESET
    // ========================================================

    pub fn reset(
        &mut self,
    ) {

        self.left_stick.reset();

        self.right_stick.reset();

        self.l2.reset();

        self.r2.reset();

        self.touch_1.active =
            false;

        self.touch_2.active =
            false;

        self.buttons = 0;

        self.stop_vibration();

        self.events.clear();
    }
  }
