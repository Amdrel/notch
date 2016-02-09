use std::fmt;
use super::byteorder::{BigEndian, ByteOrder};

// Size of the memory map of a CHIP-8 interpreter is 4kb.
const RAM_SIZE: usize = 4096;

// Display size parameters.
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

// Memory map constants.
//const START_RESERVED: u16 = 0x000;
//const END_RESERVED: u16 = 0x200;
//const END_PROGRAM_SPACE: u16 = 0xFFF;

#[derive(Default)]
pub struct Interconnect {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub display: Vec<u8>,
}

impl Interconnect {
    pub fn new(rom: Vec<u8>) -> Interconnect {
        Interconnect {
            rom: rom,
            ram: vec![0; RAM_SIZE],
            display: vec![0; DISPLAY_SIZE],
        }
    }

    #[inline(always)]
    pub fn read_word(&self, addr: u16) -> u16 {
        BigEndian::read_u16(&self.rom[addr as usize..])
    }

    #[inline(always)]
    pub fn draw(&mut self, x: usize, y: usize, sprite: Vec<u8>) -> u8 {
        let start = y * DISPLAY_WIDTH + x;
        let mut collision: u8 = 0;

        for i in 0..sprite.len() {
            // Each byte in a sprite draws on one line.
            let offset = start + DISPLAY_WIDTH * i;

            // Loop through the bits in the current byte and set the display
            // values based on them.
            for j in 0..8 {
                let bit = (0x01 << j) & sprite[i];
                let pos = x + j;
                let index: usize;

                // Determine which pixel shall be drawn. If the sprite is
                // overflowing off the right side of the screen, if wraps back
                // to the left side.
                if pos >= DISPLAY_WIDTH {
                    let diff = pos - DISPLAY_WIDTH;
                    index = y * (DISPLAY_WIDTH * i) + j - diff;
                } else {
                    index = offset + j;
                }

                // If the sprite is drawing off the bottom of the display, do
                // not draw pixels off the display.
                if index >= 2048 {
                    continue;
                }

                // Save the previous state of the pixel before setting it
                // for collision detection.
                let prev = self.display[index];

                // Draw the bit to the display.
                self.display[index] = bit ^ prev;

                // Check the previous state of the pixel and check if it
                // was erased, if so then there was a sprite collision.
                if prev == 1 && self.display[index] == 0 {
                    collision = 1;
                }
            }
        }

        // TODO: Get rid of when a real framebuffer is aquired. Just a way to
        // visually see what is being drawn in the terminal.
        for i in 0..DISPLAY_HEIGHT {
            for j in 0..DISPLAY_WIDTH {
                if self.display[j + DISPLAY_HEIGHT * i] == 1 {
                    print!("¶");
                } else {
                    print!(" ");
                }
            }

            println!("");
        }

        collision
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "interconnect")
    }
}
