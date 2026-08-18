pub struct Display {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub enabled: bool,
}

impl Display {
    pub fn new(
        width: u32,
        height: u32,
        refresh_rate: u32,
    ) -> Self {
        Self {
            width,
            height,
            refresh_rate,
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

    pub fn set_refresh_rate(
        &mut self,
        refresh_rate: u32,
    ) {
        self.refresh_rate = refresh_rate;
    }
}
