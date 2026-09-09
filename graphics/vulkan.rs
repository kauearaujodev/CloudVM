pub struct VulkanBackend {
    pub available: bool,
    pub initialized: bool,
}

impl VulkanBackend {
    pub fn new() -> Self {
        Self {
            available: false,
            initialized: false,
        }
    }

    pub fn detect(&mut self) {
        self.available = true;
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        if !self.available {
            return Err(
                "Vulkan não está disponível.".to_string()
            );
        }

        self.initialized = true;
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.initialized = false;
    }

    pub fn is_ready(&self) -> bool {
        self.available && self.initialized
    }
      }
