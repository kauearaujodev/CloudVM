use super::vram::Vram;

pub struct GpuContext {
    pub vram: Vram,
    pub initialized: bool,
    pub width: u32,
    pub height: u32,
}

impl GpuContext {
    pub fn new(
        vram_mb: u64,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            vram: Vram::new(vram_mb),
            initialized: false,
            width,
            height,
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        if self.width == 0 || self.height == 0 {
            return Err("Resolução inválida.".to_string());
        }

        self.initialized = true;
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.initialized = false;
        self.vram.clear();
    }

    pub fn resize(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        if width == 0 || height == 0 {
            return Err("Resolução inválida.".to_string());
        }

        self.width = width;
        self.height = height;

        Ok(())
    }
}
