pub struct Cpu {
    pub registers: [u64; 16],
    pub pc: u64,
    pub sp: u64,
    pub flags: u64,
    pub halted: bool,
    pub instructions: u64,
}

impl Cpu {
    pub fn new(ram_size: u64) -> Self {
        Self {
            registers: [0; 16],
            pc: 0,
            sp: ram_size.saturating_sub(8),
            flags: 0,
            halted: false,
            instructions: 0,
        }
    }

    pub fn reset(&mut self, ram_size: u64) {
        self.registers = [0; 16];
        self.pc = 0;
        self.sp = ram_size.saturating_sub(8);
        self.flags = 0;
        self.halted = false;
        self.instructions = 0;
    }

    pub fn set_zero_flag(&mut self, value: u64) {
        if value == 0 {
            self.flags |= 1;
        } else {
            self.flags &= !1;
        }
    }

    pub fn is_zero(&self) -> bool {
        self.flags & 1 != 0
    }

    pub fn add(&mut self, a: u64, b: u64) -> u64 {
        let result = a.wrapping_add(b);
        self.set_zero_flag(result);
        result
    }

    pub fn sub(&mut self, a: u64, b: u64) -> u64 {
        let result = a.wrapping_sub(b);
        self.set_zero_flag(result);
        result
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }
}
