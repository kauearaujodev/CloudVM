pub struct WindowsBootloader {
    pub loaded: bool,
}

impl WindowsBootloader {
    pub fn new() -> Self {
        Self {
            loaded: false,
        }
    }

    pub fn load(
        &mut self,
        windows_installed: bool,
    ) -> Result<(), String> {
        if !windows_installed {
            return Err(
                "Não foi possível iniciar: Windows não instalado."
                    .to_string()
            );
        }

        self.loaded = true;

        Ok(())
    }

    pub fn reset(&mut self) {
        self.loaded = false;
    }
}
