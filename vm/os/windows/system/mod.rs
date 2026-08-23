pub struct WindowsSystem {
    pub installed: bool,
    pub services_running: bool,
    pub processes_running: bool,
}

impl WindowsSystem {
    pub fn new() -> Self {
        Self {
            installed: false,
            services_running: false,
            processes_running: false,
        }
    }

    pub fn install(&mut self) {
        self.installed = true;
    }

    pub fn start(&mut self) {
        if self.installed {
            self.services_running = true;
            self.processes_running = true;
        }
    }

    pub fn shutdown(&mut self) {
        self.services_running = false;
        self.processes_running = false;
    }
}
