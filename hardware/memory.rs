use std::collections::HashMap;

pub const PAGE_SIZE: u64 = 4096;

pub struct Memory {
    pub size: u64,
    pages: HashMap<u64, Box<[u8; PAGE_SIZE as usize]>>,
}

impl Memory {
    pub fn new(size: u64) -> Self {
        Self {
            size,
            pages: HashMap::new(),
        }
    }

    fn check(&self, address: u64) -> Result<(), String> {
        if address >= self.size {
            Err(format!(
                "Endereço 0x{:X} fora da memória virtual",
                address
            ))
        } else {
            Ok(())
        }
    }

    pub fn read8(&mut self, address: u64) -> Result<u8, String> {
        self.check(address)?;

        let page = address / PAGE_SIZE;
        let offset = (address % PAGE_SIZE) as usize;

        let data = self
            .pages
            .entry(page)
            .or_insert_with(|| Box::new([0; PAGE_SIZE as usize]));

        Ok(data[offset])
    }

    pub fn write8(
        &mut self,
        address: u64,
        value: u8,
    ) -> Result<(), String> {
        self.check(address)?;

        let page = address / PAGE_SIZE;
        let offset = (address % PAGE_SIZE) as usize;

        let data = self
            .pages
            .entry(page)
            .or_insert_with(|| Box::new([0; PAGE_SIZE as usize]));

        data[offset] = value;

        Ok(())
    }

    pub fn read64(&mut self, address: u64) -> Result<u64, String> {
        let mut value = 0u64;

        for i in 0..8 {
            value |=
                (self.read8(address + i)? as u64) << (i * 8);
        }

        Ok(value)
    }

    pub fn write64(
        &mut self,
        address: u64,
        value: u64,
    ) -> Result<(), String> {
        for i in 0..8 {
            self.write8(
                address + i,
                ((value >> (i * 8)) & 0xFF) as u8,
            )?;
        }

        Ok(())
    }
              }
