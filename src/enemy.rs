use crate::{physics::*, ship::Ships};
use specs_blit::{
    blit::Color,
    specs::{
        Builder, Component, Join, NullStorage, Read, ReadStorage, System, World, WorldExt,
        WriteStorage,
    },
    Sprite, SpriteRef,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EnemyType {
    Small,
    Medium,
    Big,
}

impl EnemyType {
    pub fn sprite(self, world: &mut World) -> SpriteRef {
        let ships = world.read_resource::<Ships>();

        match self {
            EnemyType::Small => ships.enemy_small.clone(),
            EnemyType::Medium => ships.enemy_medium.clone(),
            EnemyType::Big => ships.enemy_big.clone(),
        }
    }
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Enemy;

pub fn spawn_enemy(world: &mut World, type_: EnemyType) {
    let sprite = type_.sprite(world);

    world
        .create_entity()
        .with(Sprite::new(sprite))
        .with(Enemy)
        .with(Position::new(crate::WIDTH as f64 - 10.0, 200.0))
        .with(Velocity::new(0.0, 0.0))
        .build();
}
