pub struct Storage {
    pub capacity_gb: u64,
    pub used_gb: u64,
    pub mounted: bool,
}

impl Storage {
    pub fn new(capacity_gb: u64) -> Self {
        Self {
            capacity_gb,
            used_gb: 0,
            mounted: false,
        }
    }

    pub fn mount(&mut self) {
        self.mounted = true;
    }

    pub fn unmount(&mut self) {
        self.mounted = false;
    }

    pub fn free_gb(&self) -> u64 {
        self.capacity_gb
            .saturating_sub(self.used_gb)
    }

    pub fn allocate(
        &mut self,
        gb: u64,
    ) -> Result<(), String> {
        if self.used_gb + gb > self.capacity_gb {
            return Err(
                "Espaço insuficiente no armazenamento virtual"
                    .to_string()
            );
        }

        self.used_gb += gb;

        Ok(())
    }
}
