use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum GraphicsApi {
    Vulkan,
    OpenGL,
    Software,
}

#[derive(Clone, Copy, Debug)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn pixels(&self) -> usize {
        (self.width as usize)
            .saturating_mul(self.height as usize)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GpuCapabilities {
    pub supports_3d: bool,
    pub supports_compute: bool,
    pub supports_vulkan: bool,
    pub supports_opengl: bool,
}

impl Default for GpuCapabilities {
    fn default() -> Self {
        Self {
            supports_3d: true,
            supports_compute: true,
            supports_vulkan: true,
            supports_opengl: true,
        }
    }
}

pub struct VirtualGpu {
    pub name: String,
    pub vram_bytes: u64,
    pub resolution: Resolution,
    pub api: GraphicsApi,
    pub capabilities: GpuCapabilities,

    enabled: bool,
    frame_count: u64,
}

impl VirtualGpu {
    pub fn new(
        name: String,
        vram_gb: u64,
        width: u32,
        height: u32,
    ) -> Self {
        let vram_bytes = vram_gb
            .saturating_mul(1024)
            .saturating_mul(1024)
            .saturating_mul(1024);

        Self {
            name,
            vram_bytes,
            resolution: Resolution::new(
                width,
                height,
            ),
            api: GraphicsApi::Vulkan,
            capabilities: GpuCapabilities::default(),
            enabled: false,
            frame_count: 0,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.vram_bytes == 0 {
            return Err(
                "A GPU precisa de VRAM.".to_string()
            );
        }

        if self.resolution.width == 0
            || self.resolution.height == 0
        {
            return Err(
                "Resolução inválida.".to_string()
            );
        }

        self.enabled = true;

        Ok(())
    }

    pub fn stop(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_api(
        &mut self,
        api: GraphicsApi,
    ) -> Result<(), String> {
        match api {
            GraphicsApi::Vulkan
                if !self.capabilities.supports_vulkan =>
            {
                Err(
                    "Vulkan não é suportado."
                        .to_string()
                )
            }

            GraphicsApi::OpenGL
                if !self.capabilities.supports_opengl =>
            {
                Err(
                    "OpenGL não é suportado."
                        .to_string()
                )
            }

            _ => {
                self.api = api;
                Ok(())
            }
        }
    }

    pub fn set_resolution(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        if width == 0 || height == 0 {
            return Err(
                "Resolução inválida.".to_string()
            );
        }

        self.resolution =
            Resolution::new(width, height);

        Ok(())
    }

    pub fn render_frame(
        &mut self,
    ) -> Result<(), String> {
        if !self.enabled {
            return Err(
                "GPU desativada.".to_string()
            );
        }

        self.frame_count += 1;

        Ok(())
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn vram_mb(&self) -> u64 {
        self.vram_bytes / (1024 * 1024)
    }

    pub fn vram_gb(&self) -> u64 {
        self.vram_bytes / (1024 * 1024 * 1024)
    }

    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    pub fn supports_3d(&self) -> bool {
        self.capabilities.supports_3d
    }

    pub fn supports_compute(&self) -> bool {
        self.capabilities.supports_compute
    }
}

impl fmt::Debug for VirtualGpu {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("VirtualGpu")
            .field("name", &self.name)
            .field("vram_gb", &self.vram_gb())
            .field("resolution", &self.resolution)
            .field("api", &self.api)
            .field("enabled", &self.enabled)
            .field("frame_count", &self.frame_count)
            .finish()
    }
    }
