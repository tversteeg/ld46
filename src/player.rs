use crate::{
    input::Input,
    physics::{Position, Speed, Velocity},
    sprite,
};
use anyhow::Result;
use specs_blit::{
    specs::{
        Builder, Component, DenseVecStorage, Join, Read, ReadStorage, System, World, WorldExt,
        WriteStorage,
    },
    Sprite,
};
use sprite_gen::{MaskValue::*, Options};

/// Component to set something as controllable.
#[derive(Component, Debug, Default)]
pub struct Player;

/// System processes the player input.
pub struct PlayerSystem;
impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        Read<'a, Input>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Speed>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (input, player, speed, mut vel): Self::SystemData) {
        for (vel, speed, _) in (&mut vel, &speed, &player).join() {
            if input.up_pressed() {
                vel.0.y += speed.0;
            }
            if input.down_pressed() {
                vel.0.y -= speed.0;
            }
            if input.left_pressed() {
                vel.0.x -= speed.0;
            }
            if input.right_pressed() {
                vel.0.x += speed.0;
            }
        }
    }
}

/// Spawn a new player.
pub fn spawn_player(world: &mut World) -> Result<()> {
    let (width, _height, options) = (
        12,
        20,
        Options {
            mirror_x: true,
            mirror_y: false,
            colored: true,
            edge_brightness: 0.3,
            color_variations: 0.21,
            brightness_noise: 0.31,
            saturation: 0.5,
            seed: unsafe { miniquad::rand() as u64 },
        },
    );
    let data = [
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body1, Empty, Empty,
        Empty, Empty, Empty, Empty, Body1, Body1, Body1, Body1, Body1, Body1, Empty, Empty, Empty,
        Empty, Empty, Body1, Body2, Body2, Body2, Body2, Body1, Body1, Empty, Empty, Empty, Empty,
        Empty, Empty, Body1, Body1, Body1, Body1, Body1, Body1, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Body1, Body1, Body1, Body2, Body1, Empty, Empty, Body1, Empty, Empty, Empty,
        Empty, Body1, Body1, Body1, Body1, Body2, Empty, Empty, Body1, Empty, Empty, Empty, Body1,
        Body1, Body1, Body1, Body1, Body2, Empty, Empty, Body1, Empty, Empty, Body1, Body1, Body1,
        Body1, Body1, Body2, Body2, Empty, Body1, Body1, Empty, Empty, Body1, Body1, Body1, Body1,
        Body1, Body2, Body2, Empty, Body1, Empty, Empty, Empty, Body1, Body1, Body2, Body2, Body1,
        Body2, Body2, Empty, Body1, Empty, Empty, Body2, Body2, Body2, Body2, Body2, Body2, Body2,
        Body2, Empty, Body1, Empty, Empty, Body2, Body2, Body2, Body1, Body2, Body2, Body1, Body2,
        Empty, Body1, Empty, Body2, Body2, Body1, Body2, Body1, Body2, Body2, Body1, Body2, Empty,
        Body1, Body2, Body2, Body1, Body1, Body2, Body1, Body2, Body2, Body1, Body2, Empty, Body1,
        Body2, Empty, Body1, Body1, Body2, Body1, Body2, Body2, Body1, Body2, Empty, Body2, Body2,
        Empty, Body1, Body1, Body2, Body2, Body2, Body1, Body1, Body2, Empty, Body2, Body2, Body2,
        Body2, Body2, Body2, Body2, Body1, Body1, Body1, Body1, Empty, Body1, Body1, Body1, Body1,
        Body1, Body1, Body1, Body1, Body1, Body1, Body1, Empty, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Empty, Empty, Empty, Empty,
    ];

    let sprite = sprite::generate(width, options, &data, 4)?;
    world
        .create_entity()
        .with(Sprite::new(sprite))
        .with(Player)
        .with(Position::new(0.0, 0.0))
        .with(Velocity::new(0.0, 0.0))
        .with(Speed(0.1))
        .build();

    Ok(())
}
