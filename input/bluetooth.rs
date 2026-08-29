use super::gamepad::Gamepad;

#[derive(Clone, Debug)]
pub enum ControllerType {
    Unknown,
    DualShock4,
    DualSense,
    XboxController,
    Generic,
}

pub struct BluetoothGamepad {
    pub name: String,
    pub controller_type: ControllerType,
    pub connected: bool,
    pub gamepad: Gamepad,
}

impl BluetoothGamepad {
    pub fn new(
        name: String,
        controller_type: ControllerType,
    ) -> Self {
        Self {
            name,
            controller_type,
            connected: false,
            gamepad: Gamepad::new(),
        }
    }

    pub fn connect(&mut self) {
        self.connected = true;
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
        self.gamepad.reset();
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}
