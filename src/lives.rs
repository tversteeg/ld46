use crate::sprite;
use specs_blit::{blit::*, PixelBuffer};
use sprite_gen::{MaskValue::*, Options};

pub struct Lives {
    amount: u8,
    sprite: BlitBuffer,
}

impl Lives {
    pub fn new(amount: u8) -> Self {
        Self {
            amount,
            sprite: sprite(),
        }
    }

    pub fn render(&self, buffer: &mut PixelBuffer, x: i32, y: i32) {
        let width = buffer.width();
        for i in 0..self.amount {
            self.sprite
                .blit(buffer.pixels_mut(), width, (x + i as i32 * 12, y));
        }
    }

    pub fn increase(&mut self) {
        self.amount += 1;
    }

    pub fn reduce(&mut self) {
        if self.amount > 0 {
            self.amount -= 1;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.amount == 0
    }
}

pub fn sprite() -> BlitBuffer {
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
        Empty, Empty, Solid, Solid, Empty, Empty, Solid, Body1, Body1, Solid, Solid, Body2, Body2,
        Body1, Solid, Solid, Body2, Body1, Body1, Body1, Solid, Body2, Body2, Body1, Body1, Empty,
        Solid, Body2, Body2, Body1, Empty, Empty, Solid, Body2, Body2, Empty, Empty, Empty, Solid,
        Body2, Empty, Empty, Empty, Empty, Solid, Empty, Empty, Empty, Empty, Empty,
    ];

    sprite::buffer(width, options, &data)
}
