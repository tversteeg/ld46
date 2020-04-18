use crate::{enemy::EnemyType, random, sprite};
use specs_blit::SpriteRef;
use sprite_gen::{MaskValue::*, Options};

pub struct Ships {
    enemy_small: Vec<SpriteRef>,
    enemy_medium: Vec<SpriteRef>,
    enemy_big: Vec<SpriteRef>,
}

impl Ships {
    pub fn generate() -> Self {
        Self {
            enemy_small: Ships::gen_enemy_small(),
            enemy_medium: Ships::gen_enemy_medium(),
            enemy_big: Ships::gen_enemy_big(),
        }
    }

    pub fn enemy(&self, type_: EnemyType) -> SpriteRef {
        match type_ {
            EnemyType::Small => random::index(&self.enemy_small),
            EnemyType::Medium => random::index(&self.enemy_medium),
            EnemyType::Big => random::index(&self.enemy_big),
        }
        .clone()
    }

    fn gen_enemy_small() -> Vec<SpriteRef> {
        let (width, _height, mut options) = (
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

        (1..4)
            .map(|_| {
                options.seed = unsafe { miniquad::rand() as u64 };
                sprite::generate(width, options, &data, 1).expect("Could not generate ship")
            })
            .collect()
    }

    fn gen_enemy_medium() -> Vec<SpriteRef> {
        let (width, _height, mut options) = (
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

        (1..4)
            .map(|_| {
                options.seed = unsafe { miniquad::rand() as u64 };
                sprite::generate(width, options, &data, 1).expect("Could not generate ship")
            })
            .collect()
    }

    fn gen_enemy_big() -> Vec<SpriteRef> {
        let (width, _height, mut options) = (
            22,
            12,
            Options {
                mirror_x: false,
                mirror_y: true,
                colored: true,
                edge_brightness: 0.0,
                color_variations: 0.35297588,
                brightness_noise: 0.57201767,
                saturation: 0.7861507,
                seed: unsafe { miniquad::rand() as u64 },
            },
        );
        let data = [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body1, Body1, Body2,
            Body2, Body2, Body2, Body2, Body2, Body1, Body1, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Body1, Body1, Body1, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body1,
            Body1, Body1, Body1, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body2, Body1, Body1, Body1,
            Body2, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Body1, Body1, Body1, Body1, Body1, Body1, Body1, Body2, Body2, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body1, Body1, Body1, Body2,
            Body2, Body1, Body1, Body1, Body1, Body1, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body1, Body2, Body1, Body1,
            Body1, Body1, Body1, Body1, Body1, Body1, Body1, Empty, Empty, Empty, Empty, Empty,
            Empty, Body1, Body1, Body1, Body1, Body1, Body1, Body2, Body1, Body1, Body1, Body1,
            Body1, Body1, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body1, Body1,
            Body1, Body1, Body2, Body2, Body2, Body2, Body2, Body2, Body1, Body1, Body1, Body2,
            Body1, Body1, Empty, Empty, Empty, Empty, Body1, Body1, Body1, Body1, Body1, Body2,
            Body2, Body1, Body1, Body1, Body1, Body2, Body2, Body2, Body2, Body2, Empty, Empty,
            Empty, Empty, Empty, Body1, Body1, Body1, Body2, Body1, Body1, Body1, Body1, Body1,
            Body2, Body1, Body1, Body1, Body1, Body1, Body1, Body1, Body2, Body2, Empty, Empty,
        ];

        (1..4)
            .map(|_| {
                options.seed = unsafe { miniquad::rand() as u64 };
                sprite::generate(width, options, &data, 1).expect("Could not generate ship")
            })
            .collect()
    }
}
