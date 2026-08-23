pub struct VirtualDisk {
    pub path: String,
    pub size_gb: u64,
}

impl VirtualDisk {
    pub fn new(
        path: String,
        size_gb: u64,
    ) -> Self {
        Self {
            path,
            size_gb,
        }
    }
}
