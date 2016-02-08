use super::byteorder::{BigEndian, ByteOrder};
use super::cpu;

// Size of the memory map of a CHIP-8 interpreter is 4kb.
const RAM_SIZE: usize = 4096;

// Memory map constants.
const START_RESERVED: u16 = 0x000;
const END_RESERVED: u16 = 0x200;
const END_PROGRAM_SPACE: u16 = 0xFFF;

pub struct Interpreter {
    rom: Vec<u8>,
    ram: Vec<u8>,
    cpu: cpu::Cpu,
}

impl Interpreter {
    pub fn new(rom: Vec<u8>) -> Interpreter {
        Interpreter {
            rom: rom,
            ram: vec![0; RAM_SIZE],
            cpu: cpu::Cpu::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            println!("Loop!");
        }
    }

    fn read(&mut self) {
    }
}
