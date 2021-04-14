pub const WIDTH: u8 = 64;
pub const HEIGHT: u8 = 32;

const SIZE: usize = WIDTH as usize * HEIGHT as usize;

pub struct DisplayBuffer {
    pixels: [bool; SIZE]
}

impl DisplayBuffer {
    pub fn new() -> Self {
        DisplayBuffer {
            pixels: [false; SIZE]
        }
    }

    /// Checks if a pixel is set by index.
    pub fn is_set(&self, index: usize) -> bool {
        self.pixels[index]
    }

    /// XORs a pixel and returns if there was a collision.
    fn xor_pixel(&mut self, x: u8, y: u8) -> bool {
        if x > WIDTH || y > HEIGHT {
            return false;
        }

        let index = x as usize + (y as usize * WIDTH as usize);

        let old = self.pixels[index];
        let new = old ^ true;

        self.pixels[index] = new;

        // there is a collision if the pixel was on, but is now off
        old && !new
    }

    /// Draws a sprite and returns if there was a collision.
    pub fn draw_sprite(&mut self, x: u8, y: u8, bytes: &[u8]) -> bool {
        let mut collision = false;

        for (y_offset, &byte) in bytes.iter().enumerate() {
            for (x_offset, bit) in (0..8).rev().enumerate() {
                let bit = (byte >> bit) & 0x01;

                if bit > 0 && self.xor_pixel(x + x_offset as u8, y + y_offset as u8) {
                    collision = true;
                }
            }
        }

        collision
    }

    /// Clears all pixels
    pub fn clear(&mut self) {
        self.pixels.iter_mut()
            .for_each(|x| *x = false);
    }
}
