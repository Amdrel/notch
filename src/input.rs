use std::thread::sleep;
use std::time::Duration;

use super::sdl2::event::Event;
use super::sdl2::keyboard::Keycode;
use super::sdl2;

// Wait for the duration it takes for an instruction to execute.
const INPUT_WAIT_DELAY: u64 = 2;

pub struct Input {
    event_pump: sdl2::EventPump,

    // The current keyboard input state.
    pub input_state: [bool; 16],

    // Used for input waiting.
    pub input_dirty: bool,

    // Last key pressed.
    pub last_input: u8,

    // Set to true when sdl sends a close event.
    pub close_requested: bool,
}

impl Input {
    pub fn new(sdl_context: &sdl2::Sdl) -> Input {
        // SDL object used to collect input events.
        let event_pump = sdl_context.event_pump().unwrap();

        Input {
            event_pump: event_pump,
            input_state: [false; 16],
            input_dirty: false,
            last_input: 0,
            close_requested: false,
        }
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
                    // executing code when the cpu reads this value.
                    self.close_requested = true;
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
                break;
            }

            sleep(Duration::from_millis(INPUT_WAIT_DELAY));
        }

        self.last_input
    }

    fn set_input(&mut self, key: u8, down: bool) {
        self.input_state[key as usize] = down;
        self.last_input = key;
        self.input_dirty = true;
    }
}
