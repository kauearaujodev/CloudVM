pub struct Vram {
    capacity: usize,
    used: usize,
}

impl Vram {
    pub fn new(size_mb: u64) -> Self {
        let capacity = (size_mb as usize)
            .saturating_mul(1024 * 1024);

        Self {
            capacity,
            used: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn used(&self) -> usize {
        self.used
    }

    pub fn free(&self) -> usize {
        self.capacity.saturating_sub(self.used)
    }

    pub fn allocate(&mut self, size: usize) -> Result<(), String> {
        if size > self.free() {
            return Err("VRAM insuficiente.".to_string());
        }

        self.used += size;
        Ok(())
    }

    pub fn free_memory(&mut self, size: usize) {
        self.used = self.used.saturating_sub(size);
    }

    pub fn clear(&mut self) {
        self.used = 0;
    }
}
