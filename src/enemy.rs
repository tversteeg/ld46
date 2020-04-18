use crate::{lives::Lives, movement::*, physics::*, ship::Ships};
use specs_blit::{specs::*, Sprite, SpriteRef};

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
        .with(Velocity::new(-0.5, 0.0))
        .with(Zigzag::new(1.0, 0.01))
        .build();
}

pub struct EnemySystem;
impl<'a> System<'a> for EnemySystem {
    type SystemData = (
        Entities<'a>,
        Option<Write<'a, Lives>>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (entities, mut lives, enemy, pos): Self::SystemData) {
        if let Some(mut lives) = lives {
            for (entity, pos, _) in (&*entities, &pos, &enemy).join() {
                if pos.0.x <= 0.0 {
                    lives.reduce();

                    let _ = entities.delete(entity);
                }
            }
        }
    }
}
