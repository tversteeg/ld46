use crate::physics::Position;
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

/// Generate a random sprite from a mask and return it as a blit buffer.
pub fn generate(
    width: usize,
    options: Options,
    mask: &[MaskValue],
    rotations: u16,
) -> Result<SpriteRef> {
    let buffer_width = if options.mirror_x { width * 2 } else { width };

    let buf = BlitBuffer::from_buffer(
        &sprite_gen::gen_sprite(&mask, width, options)
            .into_iter()
            // Invert the colors
            .map(|p| p ^ 0xFF_FF_FF_FF)
            .collect::<Vec<_>>(),
        buffer_width as i32,
        Color::from_u32(0),
    );

    specs_blit::load(buf, rotations)
}
