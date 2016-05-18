// Size of the memory map of a CHIP-8 interpreter is 4kB.
const RAM_SIZE: usize = 4096;

pub struct Memory {
    // Memory allocated for the running CHIP-8 application.
    ram: Vec<u8>,
}

impl Memory {
    /// Allocate some memory for ram and load a rom into it.
    pub fn new(rom: Vec<u8>) -> Memory {
        // Allocate 4kB of memory as defined in the many specifications.
        let mut ram = vec![0; RAM_SIZE];

        // Dump the rom containing the executable code of the program into ram
        // starting at the start of the program space.
        for i in 0..rom.len() {
            ram[i + END_RESERVED] = rom[i];
        }
    }

    /// Write an 8-bit byte at a specific address. There is no concern over
    /// endianess since this function operates on single bytes.
    #[inline(always)]
    pub fn write(&mut self, addr: u16, byte: u8) {

    }

    /// Reads a 16-bit word from ram. This function is used mainly to read and
    /// execute instructions as their word size is 16 bits.
    #[inline(always)]
    pub fn read_word(&self, addr: u16) -> u16 {
        BigEndian::read_u16(&self.ram[addr as usize..])
    }

    /// Dumps the standard CHIP-8 fonts to ram. The fonts are stored at the
    /// start of reserved memory and this is fine since the fonts are the only
    /// thing being stored in reserved memory.
    fn dump_fonts(&mut self) {
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
                self.ram[start + j] = fonts[i][j];
            }
        }
    }
}
