#[derive(Clone, Debug)]
pub struct PciDevice {
    pub vendor_id: u16,
    pub device_id: u16,
    pub name: String,
}

pub struct PciBus {
    devices: Vec<PciDevice>,
}

impl PciBus {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn add_device(
        &mut self,
        vendor_id: u16,
        device_id: u16,
        name: impl Into<String>,
    ) {
        self.devices.push(PciDevice {
            vendor_id,
            device_id,
            name: name.into(),
        });
    }

    pub fn devices(&self) -> &[PciDevice] {
        &self.devices
    }

    pub fn find_device(
        &self,
        vendor_id: u16,
        device_id: u16,
    ) -> Option<&PciDevice> {
        self.devices.iter().find(|device| {
            device.vendor_id == vendor_id
                && device.device_id == device_id
        })
    }
}
