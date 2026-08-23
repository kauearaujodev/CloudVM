pub struct WindowsDesktop {
    pub visible: bool,
    pub width: u32,
    pub height: u32,
}

impl WindowsDesktop {
    pub fn new(
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            visible: false,
            width,
            height,
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn resize(
        &mut self,
        width: u32,
        height: u32,
    ) {
        self.width = width;
        self.height = height;
    }
}
