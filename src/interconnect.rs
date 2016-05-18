use std::fmt;
use std::thread::sleep;
use std::time::Duration;

use super::sdl2::audio::{AudioCallback, AudioSpecDesired};
use super::sdl2::event::Event;
use super::sdl2::keyboard::Keycode;
use super::sdl2;

use super::graphics::Graphics;
use super::memory::Memory;

// Wait for the duration it takes for an instruction to execute.
const INPUT_WAIT_DELAY: u64 = 2;

pub struct Interconnect {
    // SDL objects for communication with the window system.
    audio_device: sdl2::audio::AudioDevice<BeepCallback>,
    event_pump: sdl2::EventPump,

    // Memory handles allocation along with reading and writing memory.
    pub memory: Memory,

    // Graphics manages drawing with SDL.
    pub graphics: Graphics,

    // The current keyboard input state.
    pub input_state: [bool; 16],

    // Used for input waiting.
    pub input_dirty: bool,

    // Last key pressed.
    pub last_input: u8,

    // When true beeping audio will play.
    pub beeping: bool,

    // The CPU reads this value before executing instructions, and when set to
    // true the CPU will stop executing.
    pub halt: bool,
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

        // SDL object used to collect input events.
        let event_pump = sdl_context.event_pump().unwrap();

        // Initialize the memory for the virtual machine and load the rom.
        let memory = Memory::new(rom);

        // Graphics requires an SDL context to create a window with.
        let graphics = Graphics::new(&sdl_context);

        // Create and return the finished virtual machine struct now that
        // everything is initialized and in a good state.
        let interconnect = Interconnect {
            audio_device: device,
            event_pump: event_pump,
            memory: memory,
            graphics: graphics,
            input_state: [false; 16],
            input_dirty: false,
            last_input: 0,
            beeping: false,
            halt: false,
        };
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

        self.handle_sound();
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
                break;
            }

            sleep(Duration::from_millis(INPUT_WAIT_DELAY));
        }

        self.last_input
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
