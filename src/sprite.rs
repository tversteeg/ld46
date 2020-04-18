use crate::{color, physics::Position};
use anyhow::Result;
use specs_blit::{
    blit::{BlitBuffer, Color},
    specs::{Join, ReadStorage, System, WriteStorage},
    Sprite, SpriteRef,
};
use sprite_gen::{MaskValue, Options};

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
}

impl Sprites {
    pub fn generate() -> Result<Self> {
        let white_particle = single_pixel(Color::from_u32(color::FOREGROUND))?;
        let red_particle = single_pixel(Color::from_u32(color::RED))?;

        Ok(Self {
            red_particle,
            white_particle,
        })
    }
}
