use crate::sprite;
use specs_blit::SpriteRef;
use sprite_gen::{MaskValue::*, Options};

pub struct Ships {
    pub ally_small: SpriteRef,
    pub ally_medium: SpriteRef,
    pub ally_big: SpriteRef,

    pub enemy_small: SpriteRef,
    pub enemy_medium: SpriteRef,
    pub enemy_big: SpriteRef,
}

impl Ships {
    pub fn generate() -> Self {
        Self {
            ally_small: Ships::gen_enemy_small(),
            ally_medium: Ships::gen_enemy_medium(),
            ally_big: Ships::gen_enemy_medium(),

            enemy_small: Ships::gen_enemy_small(),
            enemy_medium: Ships::gen_enemy_medium(),
            enemy_big: Ships::gen_enemy_medium(),
        }
    }

    fn gen_enemy_small() -> SpriteRef {
        let (width, _height, options) = (
            10,
            8,
            Options {
                mirror_x: false,
                mirror_y: true,
                colored: true,
                edge_brightness: 0.018196218,
                color_variations: 0.23466119,
                brightness_noise: 0.83068764,
                saturation: 0.6434743,
                seed: unsafe { miniquad::rand() as u64 },
            },
        );
        let data = [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Body1, Body1, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Body1, Body1, Body1, Body1, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Body1, Body2, Body1, Empty, Empty, Empty, Empty, Body1, Body1, Body1, Body1,
            Body2, Body1, Empty, Empty, Body1, Body1, Body1, Body1, Body1, Body2, Body1, Body1,
            Empty, Body1, Body1, Body2, Body2, Body2, Body1, Body1, Body1, Body1, Body1, Body1,
            Body1, Body1, Body2, Body2, Body2, Body2, Body1, Body1,
        ];

        sprite::generate(width, options, &data, 1).expect("Could not generate ship")
    }

    fn gen_enemy_medium() -> SpriteRef {
        let (width, _height, options) = (
            13,
            10,
            Options {
                mirror_x: false,
                mirror_y: true,
                colored: true,
                edge_brightness: 0.018196218,
                color_variations: 0.23466119,
                brightness_noise: 0.83068764,
                saturation: 0.6434743,
                seed: unsafe { miniquad::rand() as u64 },
            },
        );
        let data = [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Body1, Body1, Body1, Body1, Body1, Body1, Body1,
            Body1, Empty, Empty, Empty, Empty, Empty, Body1, Body1, Body2, Body2, Body2, Body2,
            Body2, Body1, Empty, Empty, Empty, Empty, Empty, Empty, Body1, Body1, Body1, Body1,
            Body1, Body2, Body1, Empty, Empty, Empty, Empty, Empty, Body1, Body1, Body1, Body1,
            Body1, Body1, Body2, Body1, Empty, Empty, Empty, Empty, Body1, Body1, Body1, Body1,
            Body1, Body1, Body1, Body2, Body1, Empty, Empty, Empty, Body1, Body1, Body1, Body1,
            Body1, Body1, Body1, Body1, Body2, Body1, Empty, Empty, Empty, Body1, Body1, Body2,
            Body2, Body2, Body2, Body1, Body1, Body2, Body1, Empty, Empty, Empty, Body1, Body2,
            Body1, Body1, Body1, Body1, Body2, Body1, Body2, Body1, Empty, Empty, Body1, Body1,
            Body2, Body1, Body1, Body1, Body1, Body2, Body1, Body2, Body1, Empty,
        ];

        sprite::generate(width, options, &data, 1).expect("Could not generate ship")
    }
}
