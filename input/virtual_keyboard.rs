use std::collections::HashSet;

pub struct VirtualKeyboard {
    pressed_keys: HashSet<String>,
}

impl VirtualKeyboard {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
        }
    }

    pub fn press(
        &mut self,
        key: impl Into<String>,
    ) {
        self.pressed_keys.insert(key.into());
    }

    pub fn release(
        &mut self,
        key: &str,
    ) {
        self.pressed_keys.remove(key);
    }

    pub fn is_pressed(
        &self,
        key: &str,
    ) -> bool {
        self.pressed_keys.contains(key)
    }

    pub fn release_all(&mut self) {
        self.pressed_keys.clear();
    }
}
