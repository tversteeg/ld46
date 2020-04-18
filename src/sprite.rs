use crate::{color, physics::Position};
use anyhow::Result;
use specs_blit::{
    blit::{BlitBuffer, Color},
    specs::{Join, ReadStorage, System, WriteStorage},
    Sprite, SpriteRef,
};
use sprite_gen::{
    MaskValue::{self, *},
    Options,
};

/// A system that connects sprites to the physics position.
pub struct SpritePositionSystem;
impl<'a> System<'a> for SpritePositionSystem {
    type SystemData = (ReadStorage<'a, Position>, WriteStorage<'a, Sprite>);

    fn run(&mut self, (pos, mut sprite): Self::SystemData) {
        for (pos, sprite) in (&pos, &mut sprite).join() {
            sprite.set_pos(pos.x as i32, pos.y as i32);
        }
    }
}

pub fn buffer(width: usize, options: Options, mask: &[MaskValue]) -> BlitBuffer {
    let buffer_width = if options.mirror_x { width * 2 } else { width };

    BlitBuffer::from_buffer(
        &sprite_gen::gen_sprite(&mask, width, options)
            .into_iter()
            // Invert the colors
            .map(|p| p ^ 0xFF_FF_FF_FF)
            .collect::<Vec<_>>(),
        buffer_width as i32,
        Color::from_u32(0),
    )
}

/// Generate a random sprite from a mask and return it as a blit buffer.
pub fn generate(
    width: usize,
    options: Options,
    mask: &[MaskValue],
    rotations: u16,
) -> Result<SpriteRef> {
    specs_blit::load(buffer(width, options, mask), rotations)
}

/// Generate a single pixel sprite.
pub fn single_pixel(color: Color) -> Result<SpriteRef> {
    let buf = BlitBuffer::from_buffer(&[color.u32()], 1, Color::from_u32(0));

    specs_blit::load(buf, 1)
}

pub struct Sprites {
    pub red_particle: SpriteRef,
    pub white_particle: SpriteRef,
    pub big_projectile: SpriteRef,
    pub big_projectile_width: f64,
    pub big_projectile_height: f64,
    pub small_projectile: SpriteRef,
    pub small_projectile_width: f64,
    pub small_projectile_height: f64,
}

impl Sprites {
    pub fn generate() -> Result<Self> {
        let white_particle = single_pixel(Color::from_u32(color::FOREGROUND))?;
        let red_particle = single_pixel(Color::from_u32(color::RED))?;
        let (big_projectile, big_projectile_width, big_projectile_height) =
            Sprites::generate_big_projectile()?;
        let (small_projectile, small_projectile_width, small_projectile_height) =
            Sprites::generate_small_projectile()?;

        Ok(Self {
            red_particle,
            white_particle,
            big_projectile,
            big_projectile_width,
            big_projectile_height,
            small_projectile,
            small_projectile_width,
            small_projectile_height,
        })
    }

    pub fn big_projectile(&self) -> (SpriteRef, f64, f64) {
        (
            self.big_projectile.clone(),
            self.big_projectile_width,
            self.big_projectile_height,
        )
    }

    pub fn small_projectile(&self) -> (SpriteRef, f64, f64) {
        (
            self.small_projectile.clone(),
            self.small_projectile_width,
            self.small_projectile_height,
        )
    }

    fn generate_big_projectile() -> Result<(SpriteRef, f64, f64)> {
        let (width, height, options) = (
            4,
            4,
            Options {
                mirror_x: true,
                mirror_y: true,
                colored: true,
                edge_brightness: 0.0,
                color_variations: 0.8547504,
                brightness_noise: 0.9012264,
                saturation: 1.0,
                seed: unsafe { miniquad::rand() as u64 },
            },
        );
        let data = [
            Empty, Empty, Empty, Body1, Empty, Body1, Body2, Body2, Empty, Body2, Body2, Body2,
            Body1, Body2, Body2, Body2,
        ];

        Ok((
            generate(width, options, &data, 1)?,
            width as f64 * 2.0,
            height as f64 * 2.0,
        ))
    }

    fn generate_small_projectile() -> Result<(SpriteRef, f64, f64)> {
        let (width, height, options) = (
            3,
            3,
            Options {
                mirror_x: true,
                mirror_y: true,
                colored: true,
                edge_brightness: 0.0,
                color_variations: 0.8547504,
                brightness_noise: 0.9012264,
                saturation: 1.0,
                seed: unsafe { miniquad::rand() as u64 },
            },
        );
        let data = [
            Empty, Empty, Empty, Empty, Body1, Body2, Empty, Body2, Body2,
        ];

        Ok((
            generate(width, options, &data, 1)?,
            width as f64 * 2.0,
            height as f64 * 2.0,
        ))
    }
}
