use super::instance::{VirtualMachine, VmState};

pub struct VmBoot;

impl VmBoot {
    pub fn boot(
        vm: &mut VirtualMachine,
    ) -> Result<(), String> {
        match vm.state {
            VmState::Created
            | VmState::Stopped => {
                vm.start();
                vm.running();

                Ok(())
            }

            VmState::Running => {
                Err(
                    "A VM já está em execução."
                        .to_string()
                )
            }

            VmState::Starting => {
                Err(
                    "A VM ainda está iniciando."
                        .to_string()
                )
            }

            VmState::Paused => {
                Err(
                    "A VM está pausada."
                        .to_string()
                )
            }
        }
    }
}
