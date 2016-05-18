use super::cpu::Cpu;
use super::interconnect::Interconnect;

pub struct VirtualMachine {
    cpu: Cpu,
}

impl VirtualMachine {
    pub fn new(rom: Vec<u8>) -> VirtualMachine {
        // Create a clean cpu state and interconnect (manages memory/input/etc).
        let interconnect = Interconnect::new(rom);
        let cpu = Cpu::new(interconnect);

        VirtualMachine {
            cpu: cpu,
        }
    }

    /// Wrapper for the cpu's run function. Simply starts code execution at the
    /// end of reserved program memory.
    pub fn run(&mut self) {
        self.cpu.run();
    }
}
