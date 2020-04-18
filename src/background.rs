use crate::{color, random};
use std::ptr;

const STARS_BRIGHT: usize = 200;
const STARS_DIM: usize = 200;
const RED_COLORS: usize = 500;
const GREEN_COLORS: usize = 100;
const BLUE_COLORS: usize = 200;

const SIZE: usize = crate::WIDTH * crate::HEIGHT;

pub struct Background {
    buffer: Vec<u32>,
}

impl Background {
    pub fn new() -> Self {
        let mut buffer: Vec<[u8; 3]> = vec![[0x05; 3]; SIZE];
        (0..RED_COLORS).for_each(|_| buffer[random::usize(SIZE)][0] = 0xFF);
        (0..GREEN_COLORS).for_each(|_| buffer[random::usize(SIZE)][1] = 0xFF);
        (0..BLUE_COLORS).for_each(|_| buffer[random::usize(SIZE)][2] = 0xFF);

        fastblur::gaussian_blur(&mut buffer, crate::WIDTH, crate::HEIGHT, 10.0);

        (0..RED_COLORS).for_each(|_| buffer[random::usize(SIZE)][0] = 0xFF);
        (0..GREEN_COLORS).for_each(|_| buffer[random::usize(SIZE)][1] = 0xFF);
        (0..BLUE_COLORS).for_each(|_| buffer[random::usize(SIZE)][2] = 0xFF);

        fastblur::gaussian_blur(&mut buffer, crate::WIDTH, crate::HEIGHT, 7.0);

        // Convert [R, G, B] to [u32]
        let mut buffer = buffer
            .into_iter()
            .map(|rgb| {
                u32::from(rgb[0])
                    | (u32::from(rgb[1]) << 8)
                    | (u32::from(rgb[2]) << 16)
                    | 0xFF000000
            })
            .collect::<Vec<_>>();

        // Add some nice colors pretty stars
        (0..STARS_BRIGHT).for_each(|_| buffer[random::usize(SIZE)] = color::STAR_BRIGHT);
        (0..STARS_DIM).for_each(|_| buffer[random::usize(SIZE)] = color::STAR_DIM);

        Self { buffer }
    }

    pub fn copy(&self, buffer: &mut Vec<u32>) {
        unsafe {
            ptr::copy(self.buffer.as_ptr(), buffer.as_mut_ptr(), SIZE);
        }
    }
}
