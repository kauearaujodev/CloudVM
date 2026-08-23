use super::config::VmConfig;

#[derive(Clone, Debug, PartialEq)]
pub enum VmState {
    Created,
    Starting,
    Running,
    Paused,
    Stopped,
}

pub struct VirtualMachine {
    pub config: VmConfig,
    pub state: VmState,
}

impl VirtualMachine {
    pub fn new(config: VmConfig) -> Self {
        Self {
            config,
            state: VmState::Created,
        }
    }

    pub fn start(&mut self) {
        if self.state == VmState::Stopped
            || self.state == VmState::Created
        {
            self.state = VmState::Starting;
        }
    }

    pub fn running(&mut self) {
        self.state = VmState::Running;
    }

    pub fn pause(&mut self) {
        if self.state == VmState::Running {
            self.state = VmState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.state == VmState::Paused {
            self.state = VmState::Running;
        }
    }

    pub fn stop(&mut self) {
        self.state = VmState::Stopped;
    }

    pub fn is_running(&self) -> bool {
        self.state == VmState::Running
    }
}
