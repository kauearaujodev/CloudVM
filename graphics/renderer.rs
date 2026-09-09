use super::framebuffer::FrameBuffer;

pub struct Renderer {
    pub framebuffer: FrameBuffer,
    pub enabled: bool,
    pub frame_count: u64,
}

impl Renderer {
    pub fn new(
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            framebuffer: FrameBuffer::new(width, height),
            enabled: false,
            frame_count: 0,
        }
    }

    pub fn start(&mut self) {
        self.enabled = true;
    }

    pub fn stop(&mut self) {
        self.enabled = false;
    }

    pub fn clear(
        &mut self,
        color: u32,
    ) -> Result<(), String> {
        self.check_enabled()?;

        self.framebuffer.clear(color);

        Ok(())
    }

    pub fn draw_pixel(
        &mut self,
        x: u32,
        y: u32,
        color: u32,
    ) -> Result<(), String> {
        self.check_enabled()?;

        self.framebuffer.set_pixel(x, y, color)
    }

    pub fn render_frame(&mut self) -> Result<(), String> {
        self.check_enabled()?;

        self.frame_count += 1;

        Ok(())
    }

    pub fn resize(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        if width == 0 || height == 0 {
            return Err("Resolução inválida.".to_string());
        }

        self.framebuffer.resize(width, height);

        Ok(())
    }

    fn check_enabled(&self) -> Result<(), String> {
        if !self.enabled {
            return Err("Renderizador desativado.".to_string());
        }

        Ok(())
    }
}
