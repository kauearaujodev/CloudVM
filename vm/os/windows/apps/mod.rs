pub struct WindowsApps {
    pub chrome_installed: bool,
}

impl WindowsApps {
    pub fn new() -> Self {
        Self {
            chrome_installed: false,
        }
    }

    pub fn install_chrome(&mut self) {
        self.chrome_installed = true;
    }

    pub fn chrome_available(&self) -> bool {
        self.chrome_installed
    }
}
