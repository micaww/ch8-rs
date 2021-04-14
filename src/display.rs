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
        if x >= WIDTH || y >= HEIGHT {
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
        if x >= WIDTH || y >= HEIGHT {
            return false;
        }

        let mut collision = false;

        let mut current_y = y;

        for &byte in bytes.iter() {
            let mut current_x = x;

            for bit in (0..8).rev() {
                let bit = (byte >> bit) & 0x01;

                if bit > 0 && self.xor_pixel(current_x, current_y) {
                    collision = true;
                }

                current_x += 1;

                if current_x == WIDTH {
                    current_x = 0;
                }
            }

            current_y += 1;

            if current_y == HEIGHT {
                current_y = 0;
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
