pub mod bootloader;
pub mod desktop;
pub mod system;
pub mod apps;

pub struct Windows {
    pub installed: bool,
    pub version: String,
    pub booted: bool,
}

impl Windows {
    pub fn new() -> Self {
        Self {
            installed: false,
            version: String::from("Windows"),
            booted: false,
        }
    }

    pub fn install(&mut self, version: String) {
        self.version = version;
        self.installed = true;
        self.booted = false;
    }

    pub fn boot(&mut self) -> Result<(), String> {
        if !self.installed {
            return Err(
                "O Windows não está instalado.".to_string()
            );
        }

        self.booted = true;

        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.booted = false;
    }

    pub fn is_running(&self) -> bool {
        self.booted
    }
}
