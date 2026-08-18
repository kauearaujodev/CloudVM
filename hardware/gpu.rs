pub struct Gpu {
    pub name: String,
    pub memory_mb: u64,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub enabled: bool,
}

impl Gpu {
    pub fn new(
        name: String,
        memory_mb: u64,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            name,
            memory_mb,
            width,
            height,
            fps: 60,
            enabled: true,
        }
    }

    pub fn set_resolution(
        &mut self,
        width: u32,
        height: u32,
    ) {
        self.width = width;
        self.height = height;
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.fps = fps;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}
