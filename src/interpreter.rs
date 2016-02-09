use super::interconnect::Interconnect;
use super::cpu;

pub struct Interpreter {
    cpu: cpu::Cpu,
}

impl Interpreter {
    pub fn new(rom: Vec<u8>) -> Interpreter {
        let interconnect = Interconnect::new(rom);

        Interpreter {
            cpu: cpu::Cpu::new(interconnect),
        }
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }
}
