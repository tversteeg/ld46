use crate::{input::Input, physics::*, sprite};
use anyhow::Result;
use specs_blit::{
    specs::{
        Builder, Component, Join, NullStorage, Read, ReadStorage, System, World, WorldExt,
        WriteStorage,
    },
    Sprite,
};
use sprite_gen::{MaskValue::*, Options};

const PLAYER_SPEED: f64 = 0.5;
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
        WriteStorage<'a, Position>,
        ReadStorage<'a, BoundingBox>,
    );

    fn run(&mut self, (input, player, mut pos, bb): Self::SystemData) {
        for (pos, bb, _) in (&mut pos, &bb, &player).join() {
            let offset = bb.center_offset();
            pos.y = input.mouse_y() as f64 - offset.y;
        }
    }
}

/// Spawn a new player.
pub fn spawn_player(world: &mut World) -> Result<()> {
    let (width, height, options) = (
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
            seed: quad_rand::rand() as u64,
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
    let sprite = sprite::generate(width, options, &data)?;

    world
        .create_entity()
        .with(Sprite::new(sprite))
        .with(Player)
        .with(Position::new(10.0, 200.0))
        .with(Velocity::new(0.0, 0.0))
        .with(Drag(PLAYER_DRAG))
        .with(Speed(PLAYER_SPEED))
        .with(BoundingBox::new(width as f64, height as f64 * 2.0))
        .build();

    Ok(())
}
