use super::byteorder::{BigEndian, ByteOrder};

// Size of the memory map of a CHIP-8 interpreter is 4kB.
pub const RAM_SIZE: usize = 4096;

// Memory map constraints for CHIP-8 virtual machines.
pub const START_RESERVED: usize = 0x000;
pub const END_RESERVED: usize = 0x200;
pub const END_PROGRAM_SPACE: usize = 0xFFF;

// Font size constants.
const CHARACTER_SIZE: usize = 5;
const CHARACTER_COUNT: usize = 16;

// Where fonts are stored in interpreter memory.
const FONT_OFFSET: usize = 0;

pub struct Memory {
    // Memory allocated for the running CHIP-8 application.
    ram: Vec<u8>,
}

impl Memory {
    /// Allocate some memory for ram and load a rom into it.
    pub fn new(rom: Vec<u8>) -> Memory {
        // Allocate 4kB of memory as defined in the many specifications.
        let mut ram = vec![0; RAM_SIZE];
        Memory::dump_rom(&mut ram, &rom);
        Memory::dump_fonts(&mut ram);

        Memory {
            ram: ram,
        }
    }

    /// Simply returns an 8-bit word at the specified address.
    #[inline(always)]
    pub fn read(&self, addr: usize) -> u8 {
        self.ram[addr]
    }

    /// Write an 8-bit byte at a specific address. There is no concern over
    /// endianess since this function operates on single bytes.
    #[inline(always)]
    pub fn write(&mut self, addr: usize, byte: u8) {
        self.ram[addr] = byte;
    }

    /// Reads a 16-bit word from ram. This function is used mainly to read and
    /// execute instructions as their word size is 16 bits.
    #[inline(always)]
    pub fn read_word(&self, addr: u16) -> u16 {
        BigEndian::read_u16(&self.ram[addr as usize..])
    }

    /// Find the memory address of the requested character.
    #[inline(always)]
    pub fn get_font(&self, font: u8) -> u16 {
        FONT_OFFSET as u16 + font as u16 * CHARACTER_SIZE as u16
    }

    /// Dumps a passed rom containing executable code into ram starting at
    /// program space (right after reserved space ends).
    fn dump_rom(ram: &mut Vec<u8>, rom: &Vec<u8>) {
        // Dump the rom containing the executable code of the program into ram
        // starting at the start of the program space.
        for i in 0..rom.len() {
            ram[i + END_RESERVED] = rom[i];
        }
    }

    /// Dumps the standard CHIP-8 fonts to ram. The fonts are stored at the
    /// start of reserved memory and this is fine since the fonts are the only
    /// thing being stored in reserved memory.
    fn dump_fonts(ram: &mut Vec<u8>) {
        // The characters 0-F to be stored in ram as a font for chip 8 programs.
        // Nested vectors are used for ease of reading.
        //
        // Hopefully this is optimized by the compiler, but it's no problem
        // since this is run only once.
        let fonts = vec![
            vec![0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            vec![0x20, 0x60, 0x20, 0x20, 0x70], // 1
            vec![0xF0, 0x10, 0xf0, 0x80, 0xF0], // 2
            vec![0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            vec![0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            vec![0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            vec![0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
            vec![0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            vec![0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            vec![0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            vec![0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            vec![0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            vec![0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            vec![0xE0, 0x90, 0x90, 0x90, 0xE0], // D
            vec![0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            vec![0xF0, 0x80, 0xF0, 0x80, 0x80], // F
        ];

        for i in 0..CHARACTER_COUNT {
            // Find where the current character should be stored in memory.
            let start: usize = FONT_OFFSET + i * CHARACTER_SIZE;

            // Copy the current character into the calculated spot in memory.
            for j in 0..CHARACTER_SIZE {
                ram[start + j] = fonts[i][j];
            }
        }
    }
}
