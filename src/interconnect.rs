use std::fmt;
use std::thread::sleep;
use std::time::Duration;

use super::byteorder::{BigEndian, ByteOrder};
use super::sdl2;
use super::sdl2::pixels::Color;
use super::sdl2::rect::Point;
use super::sdl2::event::Event;
use super::sdl2::keyboard::Keycode;

// Size of the memory map of a CHIP-8 interpreter is 4kb.
const RAM_SIZE: usize = 4096;

// Where fonts are stored in interpreter memory.
const FONT_OFFSET: usize = 0;

// Font size constants.
const CHARACTER_SIZE: usize = 5;
const CHARACTER_COUNT: usize = 16;

// Display size parameters.
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

// Wait for the duration it takes for an instruction to execute.
const INPUT_WAIT_DELAY: u64 = 2;

// Memory map constraints.
pub const START_RESERVED: usize = 0x000;
pub const END_RESERVED: usize = 0x200;
pub const END_PROGRAM_SPACE: usize = 0xFFF;

pub struct Interconnect {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    renderer: sdl2::render::Renderer<'static>,
    event_pump: sdl2::EventPump,

    // The current keyboard input state.
    pub input_state: [bool; 0xF],

    // Used for input waiting.
    pub input_dirty: bool,

    // Last key pressed.
    pub last_input: u8,

    // The CPU reads this value before executing instructions, and when set to
    // true the CPU will stop executing.
    pub halt: bool,

    // RAM used by the application. 4k in size.
    pub ram: Vec<u8>,

    // 64x32 buffer for the application to write to.
    pub display: Vec<u8>,
}

impl Interconnect {
    pub fn new(rom: Vec<u8>) -> Interconnect {
        let mut ram = vec![0; RAM_SIZE];

        // Dump the rom into ram starting at the start of the program space.
        for i in 0..rom.len() {
            ram[i + END_RESERVED] = rom[i];
        }

        // Setup SDL for graphics and audio.
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Notch", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        // Create a renderer that is scaled up a bit. The CHIP-8 display is
        // very small for today's standards.
        let mut renderer = window.renderer().build().unwrap();
        renderer.set_scale(10.0, 10.0);

        // Clear the screen to black.
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut interconnect = Interconnect {
            sdl_context: sdl_context,
            video_subsystem: video_subsystem,
            renderer: renderer,
            event_pump: event_pump,
            input_state: [false; 0xF],
            input_dirty: false,
            last_input: 0,
            halt: false,
            ram: ram,
            display: vec![0; DISPLAY_SIZE],
        };
        interconnect.dump_fonts();
        interconnect
    }

    fn set_input(&mut self, key: u8, down: bool) {
        self.input_state[key as usize] = down;
        self.last_input = key;
        self.input_dirty = true;
    }

    /// Get input events from SDL and set the input state.
    pub fn handle_input(&mut self) {
        // Collect the events from the iterator ahead of time so we are not
        // borrowing when we need to set the input state.
        let events: Vec<Event> = self.event_pump.poll_iter().collect();

        for event in events {
            match event {
                Event::Quit {..} => {
                    // Detect close button or escape button events.
                    // The interpreter is then signaled to halt and stop
                    // executing code.
                    self.halt = true;
                },

                // Keyboard to CHIP-8 keycode mapping.
                Event::KeyDown { keycode: Some(Keycode::Num0), .. } => { self.set_input(0x0, true ); },
                Event::KeyUp   { keycode: Some(Keycode::Num0), .. } => { self.set_input(0x0, false); },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => { self.set_input(0x1, true ); },
                Event::KeyUp   { keycode: Some(Keycode::Num1), .. } => { self.set_input(0x1, false); },
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => { self.set_input(0x2, true ); },
                Event::KeyUp   { keycode: Some(Keycode::Num2), .. } => { self.set_input(0x2, false); },
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => { self.set_input(0x3, true ); },
                Event::KeyUp   { keycode: Some(Keycode::Num3), .. } => { self.set_input(0x3, false); },
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => { self.set_input(0x4, true ); },
                Event::KeyUp   { keycode: Some(Keycode::Num4), .. } => { self.set_input(0x4, false); },
                Event::KeyDown { keycode: Some(Keycode::Num5), .. } => { self.set_input(0x5, true ); },
                Event::KeyUp   { keycode: Some(Keycode::Num5), .. } => { self.set_input(0x5, false); },
                Event::KeyDown { keycode: Some(Keycode::Num6), .. } => { self.set_input(0x6, true ); },
                Event::KeyUp   { keycode: Some(Keycode::Num6), .. } => { self.set_input(0x6, false); },
                Event::KeyDown { keycode: Some(Keycode::Num7), .. } => { self.set_input(0x7, true ); },
                Event::KeyUp   { keycode: Some(Keycode::Num7), .. } => { self.set_input(0x7, false); },
                Event::KeyDown { keycode: Some(Keycode::Num8), .. } => { self.set_input(0x8, true ); },
                Event::KeyUp   { keycode: Some(Keycode::Num8), .. } => { self.set_input(0x8, false); },
                Event::KeyDown { keycode: Some(Keycode::Num9), .. } => { self.set_input(0x9, true ); },
                Event::KeyUp   { keycode: Some(Keycode::Num9), .. } => { self.set_input(0x9, false); },
                Event::KeyDown { keycode: Some(Keycode::A),    .. } => { self.set_input(0xa, true ); },
                Event::KeyUp   { keycode: Some(Keycode::A),    .. } => { self.set_input(0xa, false); },
                Event::KeyDown { keycode: Some(Keycode::B),    .. } => { self.set_input(0xb, true ); },
                Event::KeyUp   { keycode: Some(Keycode::B),    .. } => { self.set_input(0xb, false); },
                Event::KeyDown { keycode: Some(Keycode::C),    .. } => { self.set_input(0xc, true ); },
                Event::KeyUp   { keycode: Some(Keycode::C),    .. } => { self.set_input(0xc, false); },
                Event::KeyDown { keycode: Some(Keycode::D),    .. } => { self.set_input(0xd, true ); },
                Event::KeyUp   { keycode: Some(Keycode::D),    .. } => { self.set_input(0xd, false); },
                Event::KeyDown { keycode: Some(Keycode::E),    .. } => { self.set_input(0xe, true ); },
                Event::KeyUp   { keycode: Some(Keycode::E),    .. } => { self.set_input(0xe, false); },
                Event::KeyDown { keycode: Some(Keycode::F),    .. } => { self.set_input(0xf, true ); },
                Event::KeyUp   { keycode: Some(Keycode::F),    .. } => { self.set_input(0xf, false); },
                _ => {}
            }
        }
    }

    /// Wait until an input event comes through and return the key for that
    /// input event.
    pub fn wait_input(&mut self) -> u8 {
        // Input dirtiness is used to determine if a key has been pressed,
        // regardless if the input state changed at all.
        self.input_dirty = false;

        loop {
            // Poll for input from SDL.
            self.handle_input();

            // Return the key that was pressed to make the input state dirty.
            if self.input_dirty {
                self.last_input;
            }

            sleep(Duration::from_millis(INPUT_WAIT_DELAY));
        }
    }

    /// Reads a 16 bit word from ram. This function is used mainly to read and
    /// execute instructions.
    #[inline(always)]
    pub fn read_word(&self, addr: u16) -> u16 {
        BigEndian::read_u16(&self.ram[addr as usize..])
    }

    /// Find the memory address of the requested character.
    #[inline(always)]
    pub fn get_font(&self, font: u8) -> u16 {
        FONT_OFFSET as u16 + font as u16 * CHARACTER_SIZE as u16
    }

    /// Draws a sprite to the display.
    #[inline(always)]
    pub fn draw(&mut self, x: usize, y: usize, sprite: Vec<u8>) -> u8 {
        let line = y * DISPLAY_WIDTH;
        let mut collision: u8 = 0;
        let mut values = vec![0 as u8; 8];

        for i in 0..sprite.len() {
            // Each byte in a sprite draws on one line.
            let offset = line + DISPLAY_WIDTH * i;

            // Organize the bits from the current sprite byte into a slice.
            for j in 0..values.len() {
                let bit = (sprite[i] >> j) & 0x01;
                values[8 - 1 - j] = bit;
            }

            // Loop through the bits in the current byte and set the display
            // values based on them.
            for j in 0..values.len() {
                let value = values[j];
                let pos: usize = x + j;
                let index: usize;

                // Draw a pixel in the sprite onto the display. If the pixel x
                // position is greater than the width of the display, the sprite
                // wraps around the display.
                if pos > DISPLAY_WIDTH {
                    // Wrap around to the left side to draw.
                    index = offset + pos - DISPLAY_WIDTH;
                } else {
                    // Draw at the current offset.
                    index = offset + pos;
                }

                if index >= 0 && index < DISPLAY_SIZE {
                    // Save the previous state of the pixel before setting it
                    // for collision detection.
                    let prev = self.display[index];

                    // Draw the bit to the display.
                    self.display[index] = value ^ prev;

                    // Check the previous state of the pixel and check if it
                    // was erased, if so then there was a sprite collision.
                    if prev == 1 && self.display[index] == 0 {
                        collision = 1;
                    }
                }
            }
        }

        // Draw to the SDL surface. Humans have these things called "eyes" and
        // they get upset when they cannot see things.
        self.draw_display();

        collision
    }

    /// Clears all pixels on the display by setting them all to an off state.
    pub fn clear_display(&mut self) {
        for i in 0..DISPLAY_SIZE {
            self.display[i] = 0;
        }
        self.draw_display();
    }

    /// Draw the display to the SDL surface. All pixels are white.
    fn draw_display(&mut self) {
        // Clear the screen to black.
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();

        // Draw the display to the SDL surface.
        self.renderer.set_draw_color(Color::RGB(255, 255, 255));
        for i in 0..DISPLAY_HEIGHT {
            let offset = DISPLAY_WIDTH * i;
            for j in 0..DISPLAY_WIDTH {
                if self.display[offset + j] == 1 {
                    self.renderer.draw_point(Point::new(j as i32, i as i32));
                }
            }
        }
        self.renderer.present();
    }

    /// Dumps the standard CHIP-8 fonts to ram.
    fn dump_fonts(&mut self) {
        // The characters 0-F to be stored in ram as a font for chip 8 programs.
        // Vectorception for ease of reading.
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

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "interconnect")
    }
}
