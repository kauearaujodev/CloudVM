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
            framebuffer: FrameBuffer::new(
                width,
                height,
            ),
            enabled: true,
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
        if !self.enabled {
            return Err(
                "Renderizador desativado.".to_string()
            );
        }

        self.framebuffer.clear(color);

        Ok(())
    }

    pub fn draw_pixel(
        &mut self,
        x: u32,
        y: u32,
        color: u32,
    ) -> Result<(), String> {
        if !self.enabled {
            return Err(
                "Renderizador desativado.".to_string()
            );
        }

        self.framebuffer.set_pixel(
            x,
            y,
            color,
        )
    }

    pub fn render_frame(
        &mut self,
    ) -> Result<(), String> {
        if !self.enabled {
            return Err(
                "Renderizador desativado.".to_string()
            );
        }

        self.frame_count += 1;

        Ok(())
    }

    pub fn resize(
        &mut self,
        width: u32,
        height: u32,
    ) {
        self.framebuffer.resize(
            width,
            height,
        );
    }

    pub fn width(&self) -> u32 {
        self.framebuffer.width
    }

    pub fn height(&self) -> u32 {
        self.framebuffer.height
    }
}
