use super::config::VmConfig;
use super::instance::VirtualMachine;

pub struct VmManager {
    pub machines: Vec<VirtualMachine>,
}

impl VmManager {
    pub fn new() -> Self {
        Self {
            machines: Vec::new(),
        }
    }

    pub fn create(
        &mut self,
        config: VmConfig,
    ) -> Result<usize, String> {
        if config.ram_gb == 0 {
            return Err(
                "A memória RAM deve ser maior que 0 GB."
                    .to_string()
            );
        }

        if config.storage_gb == 0 {
            return Err(
                "O armazenamento deve ser maior que 0 GB."
                    .to_string()
            );
        }

        let vm = VirtualMachine::new(config);

        self.machines.push(vm);

        Ok(self.machines.len() - 1)
    }

    pub fn get(
        &self,
        id: usize,
    ) -> Option<&VirtualMachine> {
        self.machines.get(id)
    }

    pub fn get_mut(
        &mut self,
        id: usize,
    ) -> Option<&mut VirtualMachine> {
        self.machines.get_mut(id)
    }

    pub fn delete(
        &mut self,
        id: usize,
    ) -> Result<(), String> {
        if id >= self.machines.len() {
            return Err(
                "VM não encontrada.".to_string()
            );
        }

        self.machines.remove(id);

        Ok(())
    }

    pub fn count(&self) -> usize {
        self.machines.len()
    }
}
