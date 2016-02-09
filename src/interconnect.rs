use std::fmt;
use std::default;
use super::byteorder::{BigEndian, ByteOrder};

// Size of the memory map of a CHIP-8 interpreter is 4kb.
const RAM_SIZE: usize = 4096;

// Memory map constants.
//const START_RESERVED: u16 = 0x000;
//const END_RESERVED: u16 = 0x200;
//const END_PROGRAM_SPACE: u16 = 0xFFF;

#[derive(Default)]
pub struct Interconnect {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Interconnect {
    pub fn new(rom: Vec<u8>) -> Interconnect {
        Interconnect {
            rom: rom,
            ram: vec![0; RAM_SIZE],
        }
    }

    #[inline(always)]
    pub fn read_word(&self, addr: u16) -> u16 {
        BigEndian::read_u16(&self.rom[addr as usize..])
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "interconnect")
    }
}
