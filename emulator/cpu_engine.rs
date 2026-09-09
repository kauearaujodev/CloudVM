pub struct CpuEngine {
    pub registers: [u64; 16],
    pub instruction_pointer: u64,
    pub running: bool,
    pub cycles: u64,
}

impl CpuEngine {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            instruction_pointer: 0,
            running: false,
            cycles: 0,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn reset(&mut self) {
        self.registers = [0; 16];
        self.instruction_pointer = 0;
        self.cycles = 0;
        self.running = false;
    }

    pub fn step(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("CPU virtual está desligada.".into());
        }

        self.instruction_pointer =
            self.instruction_pointer.wrapping_add(1);

        self.cycles =
            self.cycles.wrapping_add(1);

        Ok(())
    }

    pub fn set_register(
        &mut self,
        index: usize,
        value: u64,
    ) -> Result<(), String> {
        if index >= self.registers.len() {
            return Err("Registrador inválido.".into());
        }

        self.registers[index] = value;
        Ok(())
    }

    pub fn register(&self, index: usize) -> Result<u64, String> {
        self.registers
            .get(index)
            .copied()
            .ok_or_else(|| "Registrador inválido.".into())
    }
}
