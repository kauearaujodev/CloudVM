pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    pixels: Vec<u32>,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width as usize)
            .saturating_mul(height as usize);

        Self {
            width,
            height,
            pixels: vec![0; size],
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.pixels.fill(color);
    }

    pub fn set_pixel(
        &mut self,
        x: u32,
        y: u32,
        color: u32,
    ) -> Result<(), String> {
        if x >= self.width || y >= self.height {
            return Err(format!(
                "Pixel ({}, {}) fora da tela.",
                x, y
            ));
        }

        let index =
            (y as usize * self.width as usize)
            + x as usize;

        self.pixels[index] = color;

        Ok(())
    }

    pub fn get_pixel(
        &self,
        x: u32,
        y: u32,
    ) -> Result<u32, String> {
        if x >= self.width || y >= self.height {
            return Err(format!(
                "Pixel ({}, {}) fora da tela.",
                x, y
            ));
        }

        let index =
            (y as usize * self.width as usize)
            + x as usize;

        Ok(self.pixels[index])
    }

    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }

    pub fn pixels_mut(&mut self) -> &mut [u32] {
        &mut self.pixels
    }

    pub fn resize(
        &mut self,
        width: u32,
        height: u32,
    ) {
        self.width = width;
        self.height = height;

        let size = (width as usize)
            .saturating_mul(height as usize);

        self.pixels.resize(size, 0);
    }

    pub fn size(&self) -> usize {
        self.pixels.len()
    }
}
