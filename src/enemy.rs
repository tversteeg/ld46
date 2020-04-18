use crate::{
    color, effect::ScreenFlash, entity::Lifetime, lives::Lives, movement::*,
    particle::ParticleEmitter, phase::Phase, physics::*, player::Player, random, ship::Ships,
    sprite::Sprites,
};
use specs_blit::{specs::*, Sprite, SpriteRef};

const ENEMY_VELOCITY_RESOURCES_MIN_FACTOR_X: f64 = 0.1;
const ENEMY_VELOCITY_RESOURCES_MAX_FACTOR_X: f64 = 0.9;
const ENEMY_VELOCITY_RESOURCES_MIN_FACTOR_Y: f64 = 0.02;
const ENEMY_VELOCITY_RESOURCES_MAX_FACTOR_Y: f64 = 0.5;
const ENEMY_VELOCITY_RESOURCES_SPEED_X: f64 = 0.1;
const ENEMY_VELOCITY_RESOURCES_SPEED_Y: f64 = 0.05;
const ENEMY_VELOCITY_MIN_SPEED: f64 = 1.0;
const ENEMY_ZIGZAG_RESOURCES_MIN_FACTOR: f64 = 0.1;
const ENEMY_ZIGZAG_RESOURCES_MAX_FACTOR: f64 = 0.9;
const ENEMY_ZIGZAG_RESOURCES_SPEED: f64 = 0.01;

const ENEMY_ENGINE_PARTICLE_LIFETIME: f64 = 10.0;
const ENEMY_DEAD_EMITTER_LIFETIME: f64 = 5.0;
const ENEMY_DEAD_PARTICLE_LIFETIME: f64 = 50.0;

const MIN_RESOURCE_USAGE_FACTOR: f64 = 0.01;
const MAX_RESOURCE_USAGE_FACTOR: f64 = 0.3;
const TIME_RANDOM_FACTOR: f64 = 5.0;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EnemyType {
    Small,
    Medium,
    Big,
}

impl EnemyType {
    pub fn sprite(self, ships: &Ships) -> SpriteRef {
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
pub struct EnemyEmitter {
    /// Time, resources for enemy.
    spawner: Vec<(f64, f64)>,
    current_time: f64,
    total_time: f64,
}

impl EnemyEmitter {
    pub fn new(resources: f64, time: f64) -> Self {
        // Fill the resource list with random values until it's full
        let mut cached_resources = vec![];
        let mut current_resources = 0.0;
        while current_resources < resources {
            let enemy_resources = random::range(
                resources * MIN_RESOURCE_USAGE_FACTOR,
                resources * MAX_RESOURCE_USAGE_FACTOR + (time / 3600.0),
            );
            cached_resources.push(enemy_resources);
            current_resources += enemy_resources;
        }

        // Spread it out over time
        let time_dist = time / cached_resources.len() as f64;
        let mut spawner = cached_resources
            .into_iter()
            .enumerate()
            .map(|(index, resources)| {
                (
                    index as f64 * time_dist
                        + random::range(-TIME_RANDOM_FACTOR, TIME_RANDOM_FACTOR),
                    resources,
                )
            })
            .collect::<Vec<_>>();

        // Always spawn the first one immediately
        spawner[0].0 = 5.0;

        Self {
            spawner,
            current_time: 0.0,
            total_time: time,
        }
    }

    pub fn spawn_enemy_with_resource_usage(
        entities: &Entities,
        updater: &LazyUpdate,
        sprites: &Sprites,
        ships: &Ships,
        mut resources: f64,
    ) {
        let enemy = entities.create();
        updater.insert(enemy, Enemy);

        let type_ = if resources < 50.0 {
            EnemyType::Small
        } else if resources < 100.0 {
            resources -= 50.0;
            EnemyType::Medium
        } else {
            resources -= 100.0;
            EnemyType::Big
        };

        updater.insert(enemy, Sprite::new(type_.sprite(ships)));

        updater.insert(
            enemy,
            Position::new(
                crate::WIDTH as f64 + 10.0,
                random::range(0.0, crate::HEIGHT as f64),
            ),
        );

        let bb = type_.bb();
        updater.insert(
            enemy,
            ParticleEmitter::new(
                ENEMY_ENGINE_PARTICLE_LIFETIME,
                sprites.white_particle.clone(),
            )
            .with_offset(bb.to_aabr(&Position::new(0.0, 0.0)).center()),
        );

        updater.insert(enemy, bb);

        let x_velocity_resources = random::range(
            ENEMY_VELOCITY_RESOURCES_MIN_FACTOR_X,
            ENEMY_VELOCITY_RESOURCES_MAX_FACTOR_X,
        ) * resources;
        resources -= x_velocity_resources;

        if random::bool() {
            // Straight pattern
            let y_velocity_resources = random::range(
                ENEMY_VELOCITY_RESOURCES_MIN_FACTOR_Y,
                ENEMY_VELOCITY_RESOURCES_MAX_FACTOR_Y,
            ) * resources;
            resources -= y_velocity_resources;

            updater.insert(
                enemy,
                Velocity::new(
                    -x_velocity_resources * ENEMY_VELOCITY_RESOURCES_SPEED_X
                        - ENEMY_VELOCITY_MIN_SPEED,
                    y_velocity_resources * ENEMY_VELOCITY_RESOURCES_SPEED_Y,
                ),
            );
        } else {
            // Zigzag pattern
            updater.insert(
                enemy,
                Velocity::new(
                    -x_velocity_resources * ENEMY_VELOCITY_RESOURCES_SPEED_X
                        - ENEMY_VELOCITY_MIN_SPEED,
                    0.0,
                ),
            );

            let zigzag_amount_resources = random::range(
                ENEMY_ZIGZAG_RESOURCES_MIN_FACTOR,
                ENEMY_ZIGZAG_RESOURCES_MAX_FACTOR,
            ) * resources;
            resources -= zigzag_amount_resources;

            updater.insert(
                enemy,
                Zigzag::new(
                    zigzag_amount_resources * ENEMY_ZIGZAG_RESOURCES_SPEED,
                    random::range(0.001, 0.2),
                ),
            );
        }
    }
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Enemy;

pub struct EnemySystem;
impl<'a> System<'a> for EnemySystem {
    type SystemData = (
        Entities<'a>,
        Option<Write<'a, Lives>>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, Position>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, lives, enemy, pos, updater): Self::SystemData) {
        if let Some(mut lives) = lives {
            for (entity, pos, _) in (&*entities, &pos, &enemy).join() {
                if pos.0.x <= 0.0 {
                    lives.reduce();

                    let flash = entities.create();
                    updater.insert(flash, ScreenFlash::new(color::RED));
                    updater.insert(flash, Lifetime::new(5.0));

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

pub struct EnemyEmitterSystem;
impl<'a> System<'a> for EnemyEmitterSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Sprites>,
        Option<Read<'a, Ships>>,
        Write<'a, Phase>,
        WriteStorage<'a, EnemyEmitter>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (entities, sprites, ships, mut phase, mut emitter, updater): Self::SystemData,
    ) {
        if let Some(ships) = ships {
            for emitter in (&mut emitter).join() {
                if emitter.current_time >= emitter.total_time {
                    // Time ran out, change to another phase
                    *phase = Phase::SwitchTo(Box::new(Phase::Setup));
                    break;
                }
                emitter.current_time += 1.0;

                if let Some((time, resources)) = emitter.spawner.first() {
                    if *time < emitter.current_time {
                        EnemyEmitter::spawn_enemy_with_resource_usage(
                            &entities, &updater, &sprites, &ships, *resources,
                        );

                        emitter.spawner.remove(0);
                    }
                }
            }
        }
    }
}
