use super::sdl2::audio::{AudioCallback, AudioSpecDesired};
use super::sdl2;

pub struct Sound {
    // SDL objects for communication with the window system.
    audio_device: sdl2::audio::AudioDevice<BeepCallback>,

    // When true beeping audio will play.
    pub beeping: bool,
}

impl Sound {
    pub fn new(sdl_context: &sdl2::Sdl) -> Sound {
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

        Sound {
            audio_device: device,
            beeping: false,
        }
    }

    pub fn handle_sound(&self) {
        if self.beeping {
            self.audio_device.resume();
        } else {
            self.audio_device.pause();
        }
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
