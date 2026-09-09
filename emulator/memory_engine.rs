pub struct MemoryEngine {
    memory: Vec<u8>,
}

impl MemoryEngine {
    pub fn new(size_mb: usize) -> Self {
        let size = size_mb.saturating_mul(1024 * 1024);

        Self {
            memory: vec![0; size],
        }
    }

    pub fn size(&self) -> usize {
        self.memory.len()
    }

    pub fn read8(&self, address: usize) -> Result<u8, String> {
        self.memory
            .get(address)
            .copied()
            .ok_or_else(|| "Endereço de memória inválido.".into())
    }

    pub fn write8(
        &mut self,
        address: usize,
        value: u8,
    ) -> Result<(), String> {
        let cell = self
            .memory
            .get_mut(address)
            .ok_or_else(|| "Endereço de memória inválido.".to_string())?;

        *cell = value;
        Ok(())
    }

    pub fn read32(
        &self,
        address: usize,
    ) -> Result<u32, String> {
        if address + 4 > self.memory.len() {
            return Err("Leitura fora da memória.".into());
        }

        let bytes = [
            self.memory[address],
            self.memory[address + 1],
            self.memory[address + 2],
            self.memory[address + 3],
        ];

        Ok(u32::from_le_bytes(bytes))
    }

    pub fn write32(
        &mut self,
        address: usize,
        value: u32,
    ) -> Result<(), String> {
        if address + 4 > self.memory.len() {
            return Err("Escrita fora da memória.".into());
        }

        let bytes = value.to_le_bytes();

        self.memory[address..address + 4]
            .copy_from_slice(&bytes);

        Ok(())
    }

    pub fn clear(&mut self) {
        self.memory.fill(0);
    }
}
