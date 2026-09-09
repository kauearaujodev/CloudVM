pub struct SystemBus {
    pub enabled: bool,
}

impl SystemBus {
    pub fn new() -> Self {
        Self {
            enabled: false,
        }
    }

    pub fn start(&mut self) {
        self.enabled = true;
    }

    pub fn stop(&mut self) {
        self.enabled = false;
    }

    pub fn check(&self) -> Result<(), String> {
        if !self.enabled {
            return Err("System Bus está desligado.".into());
        }

        Ok(())
    }
}
