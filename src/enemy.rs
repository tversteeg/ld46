use crate::{
    entity::Lifetime, lives::Lives, movement::*, particle::ParticleEmitter, physics::*,
    player::Player, ship::Ships, sprite::Sprites,
};
use specs_blit::{specs::*, Sprite, SpriteRef};

const ENEMY_VELOCITY: f64 = -0.6;
const ENEMY_DEAD_EMITTER_LIFETIME: f64 = 5.0;
const ENEMY_DEAD_PARTICLE_LIFETIME: f64 = 20.0;

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

    pub fn bb(self) -> BoundingBox {
        match self {
            EnemyType::Small => BoundingBox::new(10.0, 16.0),
            EnemyType::Medium => BoundingBox::new(13.0, 20.0),
            _ => unimplemented!(),
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
        .with(type_.bb())
        .with(Position::new(crate::WIDTH as f64 - 10.0, 200.0))
        .with(Velocity::new(ENEMY_VELOCITY, 0.0))
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

pub struct EnemyCollisionSystem;
impl<'a> System<'a> for EnemyCollisionSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Sprites>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BoundingBox>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, sprites, enemy, player, pos, bb, updater): Self::SystemData) {
        // Check for collision with the player
        for (player_pos, player_bb, _) in (&pos, &bb, &player).join() {
            let player_aabr = player_bb.to_aabr(player_pos);
            for (entity, enemy_pos, enemy_bb, _) in (&*entities, &pos, &bb, &enemy).join() {
                let enemy_aabr = enemy_bb.to_aabr(enemy_pos);

                if enemy_aabr.collides_with_aabr(player_aabr) {
                    // Remove the enemy
                    let _ = entities.delete(entity);

                    let emitter = entities.create();
                    updater.insert(
                        emitter,
                        ParticleEmitter::new(
                            ENEMY_DEAD_PARTICLE_LIFETIME,
                            sprites.white_particle.clone(),
                        ),
                    );
                    updater.insert(emitter, Position::from_vec2(enemy_aabr.center()));
                    updater.insert(emitter, Position::from_vec2(enemy_aabr.center()));
                    updater.insert(emitter, Lifetime::new(ENEMY_DEAD_EMITTER_LIFETIME));
                }
            }
        }
    }
}
