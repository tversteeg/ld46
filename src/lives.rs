use crate::sprite;
use specs_blit::{blit::*, PixelBuffer};
use sprite_gen::{MaskValue::*, Options};

pub struct Lives {
    amount: u8,
    sprite: BlitBuffer,
}

impl Lives {
    pub fn new(amount: u8) -> Self {
        let (width, _height, options) = (
            5,
            10,
            Options {
                mirror_x: true,
                mirror_y: false,
                colored: true,
                edge_brightness: 0.24585259,
                color_variations: 0.47232494,
                brightness_noise: 0.81954944,
                saturation: 1.0,
                seed: unsafe { miniquad::rand() as u64 },
            },
        );
        let data = [
            Empty, Empty, Solid, Solid, Empty, Empty, Solid, Body1, Body1, Solid, Solid, Body2,
            Body2, Body1, Solid, Solid, Body2, Body1, Body1, Body1, Solid, Body2, Body2, Body1,
            Body1, Empty, Solid, Body2, Body2, Body1, Empty, Empty, Solid, Body2, Body2, Empty,
            Empty, Empty, Solid, Body2, Empty, Empty, Empty, Empty, Solid, Empty, Empty, Empty,
            Empty, Empty,
        ];

        let sprite = sprite::buffer(width, options, &data);

        Self { amount, sprite }
    }

    pub fn render(&self, buffer: &mut PixelBuffer) {
        let width = buffer.width();
        for i in 0..self.amount {
            self.sprite
                .blit(buffer.pixels_mut(), width, (20 + i as i32 * 12, 5));
        }
    }

    pub fn reduce(&mut self) {
        self.amount -= 1;
    }

    pub fn is_dead(&self) -> bool {
        self.amount == 0
    }
}
