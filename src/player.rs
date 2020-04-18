use crate::{
    input::Input,
    particle::ParticleEmitter,
    physics::{Drag, Position, Speed, Velocity},
    sprite,
};
use anyhow::Result;
use specs_blit::{
    blit::Color,
    specs::{
        Builder, Component, Join, NullStorage, Read, ReadStorage, System, World, WorldExt,
        WriteStorage,
    },
    Sprite,
};
use sprite_gen::{MaskValue::*, Options};

const PLAYER_SPEED: f64 = 1.0;
const PLAYER_DRAG: f64 = 0.85;

/// Component to set something as controllable.
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
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
                vel.0.y -= speed.0;
            }
            if input.down_pressed() {
                vel.0.y += speed.0;
            }
        }
    }
}

/// Spawn a new player.
pub fn spawn_player(world: &mut World) -> Result<()> {
    let (width, _height, options) = (
        11,
        22,
        Options {
            mirror_x: false,
            mirror_y: true,
            colored: true,
            edge_brightness: 0.33511457,
            color_variations: 0.01,
            brightness_noise: 0.50169325,
            saturation: 0.4671184,
            seed: unsafe { miniquad::rand() as u64 },
        },
    );
    let data = [
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body1, Body1,
        Body1, Body1, Body1, Body1, Body1, Body1, Body1, Solid, Empty, Body2, Body2, Body2, Body2,
        Body2, Empty, Empty, Empty, Body1, Solid, Empty, Body2, Body2, Body2, Empty, Body2, Body2,
        Body2, Body2, Body1, Empty, Empty, Empty, Body1, Body1, Empty, Empty, Empty, Body2, Empty,
        Body1, Empty, Empty, Empty, Body1, Empty, Empty, Solid, Solid, Body2, Empty, Body1, Solid,
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body2, Empty, Body1, Solid, Empty, Empty,
        Solid, Solid, Empty, Empty, Solid, Body2, Body2, Body1, Empty, Empty, Empty, Body1, Body1,
        Empty, Empty, Body1, Body2, Body2, Body1, Empty, Empty, Empty, Body1, Empty, Body1, Empty,
        Body1, Empty, Body2, Body1, Solid, Empty, Empty, Empty, Body1, Empty, Empty, Body1, Body1,
        Body2, Body1, Solid, Empty, Empty, Empty, Body1, Empty, Empty, Body1, Body1, Body2, Body1,
        Empty, Empty, Empty, Empty, Body1, Empty, Empty, Empty, Empty, Empty, Body1, Empty, Empty,
        Empty, Body2, Body2, Body2, Empty, Body1, Body2, Empty, Body1, Solid, Empty, Empty, Empty,
        Body2, Empty, Empty, Body1, Empty, Empty, Body1, Solid, Empty, Empty, Empty, Body2, Empty,
        Empty, Empty, Body1, Empty, Body1, Empty, Empty, Empty, Empty, Body2, Empty, Body1, Empty,
        Empty, Empty, Body1, Empty, Empty, Empty, Empty, Body2, Empty, Body1, Empty, Empty, Empty,
        Body1, Solid, Empty, Empty, Empty, Body2, Empty, Body1, Empty, Empty, Body2, Body1, Solid,
        Empty, Empty, Empty, Body2, Empty, Body1, Body1, Empty, Body2, Body1, Empty, Empty, Empty,
        Empty, Body2, Empty, Body1, Body1, Empty, Body2, Body1, Empty, Empty, Empty, Body2, Body2,
        Body2, Body1, Body1, Empty, Empty, Body1, Solid, Empty,
    ];
    let sprite = sprite::generate(width, options, &data, 1)?;

    // TODO don't generate this every time
    let particle_sprite = sprite::single_pixel(Color::from_u32(0xFF))?;

    world
        .create_entity()
        .with(Sprite::new(sprite))
        .with(Player)
        .with(Position::new(5.0, 200.0))
        .with(Velocity::new(0.0, 0.0))
        .with(Drag(PLAYER_DRAG))
        .with(Speed(PLAYER_SPEED))
        .with(ParticleEmitter::new(10.0, particle_sprite))
        .build();

    Ok(())
}
