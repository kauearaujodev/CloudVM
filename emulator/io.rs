use std::collections::HashMap;

pub struct IoBus {
    ports: HashMap<u16, u32>,
}

impl IoBus {
    pub fn new() -> Self {
        Self {
            ports: HashMap::new(),
        }
    }

    pub fn write(
        &mut self,
        port: u16,
        value: u32,
    ) {
        self.ports.insert(port, value);
    }

    pub fn read(
        &self,
        port: u16,
    ) -> u32 {
        self.ports
            .get(&port)
            .copied()
            .unwrap_or(0)
    }

    pub fn clear(&mut self) {
        self.ports.clear();
    }
}
