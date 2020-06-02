use rand::Rng;

pub const FIRE_WIDTH: usize = 320;
pub const FIRE_HEIGHT: usize = 168;
pub const TARGET_FPS: u64 = 60;
pub const TIME_PER_FRAME: u64 = 1000 / TARGET_FPS; // in milliseconds

/// RGBA color pallet
const PALLET: [[u8; 4]; 37] = [
    [0x07, 0x07, 0x07, 0xFF],
    [0x1F, 0x07, 0x07, 0xFF],
    [0x2F, 0x0F, 0x07, 0xFF],
    [0x47, 0x0F, 0x07, 0xFF],
    [0x57, 0x17, 0x07, 0xFF],
    [0x67, 0x1F, 0x07, 0xFF],
    [0x77, 0x1F, 0x07, 0xFF],
    [0x8F, 0x27, 0x07, 0xFF],
    [0x9F, 0x2F, 0x07, 0xFF],
    [0xAF, 0x3F, 0x07, 0xFF],
    [0xBF, 0x47, 0x07, 0xFF],
    [0xC7, 0x47, 0x07, 0xFF],
    [0xDF, 0x4F, 0x07, 0xFF],
    [0xDF, 0x57, 0x07, 0xFF],
    [0xDF, 0x57, 0x07, 0xFF],
    [0xD7, 0x5F, 0x07, 0xFF],
    [0xD7, 0x5F, 0x07, 0xFF],
    [0xD7, 0x67, 0x0F, 0xFF],
    [0xCF, 0x6F, 0x0F, 0xFF],
    [0xCF, 0x77, 0x0F, 0xFF],
    [0xCF, 0x7F, 0x0F, 0xFF],
    [0xCF, 0x87, 0x17, 0xFF],
    [0xC7, 0x87, 0x17, 0xFF],
    [0xC7, 0x8F, 0x17, 0xFF],
    [0xC7, 0x97, 0x1F, 0xFF],
    [0xBF, 0x9F, 0x1F, 0xFF],
    [0xBF, 0x9F, 0x1F, 0xFF],
    [0xBF, 0xA7, 0x27, 0xFF],
    [0xBF, 0xA7, 0x27, 0xFF],
    [0xBF, 0xAF, 0x2F, 0xFF],
    [0xB7, 0xAF, 0x2F, 0xFF],
    [0xB7, 0xB7, 0x2F, 0xFF],
    [0xB7, 0xB7, 0x37, 0xFF],
    [0xCF, 0xCF, 0x6F, 0xFF],
    [0xDF, 0xDF, 0x9F, 0xFF],
    [0xEF, 0xEF, 0xC7, 0xFF],
    [0xFF, 0xFF, 0xFF, 0xFF],
];

pub struct DoomFire {
    fire_pixels: Vec<usize>,
}

impl DoomFire {
    pub fn new() -> Self {
        // set the whole screen to color 0
        let mut fire_pixels = vec![0; FIRE_WIDTH * FIRE_HEIGHT];
        // set the bottom line to color 36
        for i in 0..FIRE_WIDTH {
            fire_pixels[(FIRE_HEIGHT - 1) * FIRE_WIDTH + i] = 36;
        }

        DoomFire { fire_pixels }
    }

    /// Update the internal state of each pixel for the Doom Fire effect.
    /// This is main code for the Doom Fire effect.
    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();

        for x in 0..FIRE_WIDTH {
            for y in 1..FIRE_HEIGHT {
                let src = y * FIRE_WIDTH + x;
                let pixel = self.fire_pixels[src];

                if pixel == 0 {
                    self.fire_pixels[src - FIRE_WIDTH] = 0;
                } else {
                    let rand_idx = (rng.gen_range(0.0, 3.0) + 0.5) as usize & 3;
                    let dst = src - rand_idx + 1;
                    self.fire_pixels[dst - FIRE_WIDTH] = pixel - (rand_idx & 1);
                }
            }
        }
    }

    /// Draw the next frame to a generic byte slice.
    /// frame will usually be some reference to a pixel buffer provided by some rendering library.
    /// This function will fill the frame with RGBA pixels
    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&PALLET[self.fire_pixels[i]])
        }
    }
}

impl Default for DoomFire {
    fn default() -> Self {
        DoomFire::new()
    }
}
