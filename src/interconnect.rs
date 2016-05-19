use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::rc::Weak;

use super::sdl2::audio::{AudioCallback, AudioSpecDesired};
use super::sdl2::event::Event;
use super::sdl2::keyboard::Keycode;
use super::sdl2;

use super::graphics::Graphics;
use super::memory::Memory;
use super::input::Input;

pub struct Interconnect {
    // SDL objects for communication with the window system.
    audio_device: sdl2::audio::AudioDevice<BeepCallback>,

    // Memory handles allocation along with reading and writing memory.
    pub memory: Memory,

    // Graphics manages drawing with SDL.
    pub graphics: Graphics,

    // Input handles SDL input events and the input state.
    pub input: Input,

    // When true beeping audio will play.
    pub beeping: bool,
}

impl Interconnect {
    pub fn new(rom: Vec<u8>) -> Interconnect {
        // Setup SDL for graphics and audio.
        let sdl_context = sdl2::init().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();

        // Setup beep sound parameters.
        let desired_spec = AudioSpecDesired {
            freq: Some(44100), // I think this is healthy?
            channels: Some(1), // Mono.
            samples: None,     // Default sample size.
        };
        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            BeepCallback {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();

        // Initialize all the peripherals needed by the virtual machine.
        let memory = Memory::new(rom);
        let graphics = Graphics::new(&sdl_context);
        let input = Input::new(&sdl_context);

        let interconnect = Interconnect {
            audio_device: device,
            memory: memory,
            graphics: graphics,
            input: input,
            beeping: false,
        };

        interconnect
    }

    pub fn handle_sound(&self) {
        if self.beeping {
            self.audio_device.resume();
        } else {
            self.audio_device.pause();
        }
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "interconnect")
    }
}

struct BeepCallback {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for BeepCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave.
        for x in out.iter_mut() {
            *x = match self.phase {
                0.0...0.5 => self.volume,
                _ => -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
