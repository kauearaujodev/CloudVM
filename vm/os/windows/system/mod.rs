pub struct WindowsSystem {
    pub installed: bool,
    pub version: String,
}

impl WindowsSystem {
    pub fn new(version: String) -> Self {
        Self {
            installed: false,
            version,
        }
    }

    pub fn install(&mut self) {
        self.installed = true;
    }

    pub fn is_installed(&self) -> bool {
        self.installed
    }
}
