use super::sdl2::pixels::Color;
use super::sdl2::rect::Point;
use super::sdl2;

// Display size parameters.
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

pub struct Graphics {
    renderer: sdl2::render::Renderer<'static>,

    // 64x32 buffer for the application to write to. The contents of this buffer
    // is rendered to the SDL surface.
    pub display: Vec<u8>,
}

impl Graphics {
    pub fn new(sdl_context: &sdl2::Sdl) -> Graphics {
        let video_subsystem = sdl_context.video().unwrap();

        // Create a window 10x the scale of CHIP-8's display.
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

        Graphics {
            renderer: renderer,
            display: vec![0; DISPLAY_SIZE],
        }
    }

    /// Draws a sprite to the display.
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
                let mut index: usize;

                // Draw a pixel in the sprite onto the display. If the pixel x
                // position is greater than the width of the display, the sprite
                // wraps around the display.
                if pos >= DISPLAY_WIDTH {
                    // Wrap around to the left side to draw.
                    index = offset + pos - DISPLAY_WIDTH;
                } else {
                    // Draw at the current offset.
                    index = offset + pos;
                }

                if index >= DISPLAY_SIZE {
                    index -= DISPLAY_SIZE;
                }

                if index < DISPLAY_SIZE {
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

    /// Draw the display in it's current state to the SDL surface.
    /// All pixels are white but this may be subject to change.
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
}
