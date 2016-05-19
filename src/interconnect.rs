use std::fmt;

use super::sdl2;

use super::graphics::Graphics;
use super::memory::Memory;
use super::input::Input;
use super::sound::Sound;

pub struct Interconnect {
    // Memory handles allocation along with reading and writing memory.
    pub memory: Memory,

    // Graphics manages drawing with SDL.
    pub graphics: Graphics,

    // Input handles SDL input events and the input state.
    pub input: Input,

    // Sounds handles sound output through SDL.
    pub sound: Sound,
}

impl Interconnect {
    pub fn new(rom: Vec<u8>) -> Interconnect {
        // Setup SDL for graphics and audio.
        let sdl_context = sdl2::init().unwrap();

        // Initialize all the peripherals needed by the virtual machine.
        let memory = Memory::new(rom);
        let graphics = Graphics::new(&sdl_context);
        let input = Input::new(&sdl_context);
        let sound = Sound::new(&sdl_context);

        Interconnect {
            memory: memory,
            graphics: graphics,
            input: input,
            sound: sound,
        }
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "interconnect")
    }
}
