#[derive(Clone, Debug)]
pub struct VmConfig {
    pub name: String,
    pub ram_gb: u64,
    pub storage_gb: u64,
    pub cpu_cores: u32,
    pub gpu_memory_mb: u64,
}

impl VmConfig {
    pub fn new(
        name: String,
        ram_gb: u64,
        storage_gb: u64,
        cpu_cores: u32,
        gpu_memory_mb: u64,
    ) -> Self {
        Self {
            name,
            ram_gb,
            storage_gb,
            cpu_cores,
            gpu_memory_mb,
        }
    }
}
