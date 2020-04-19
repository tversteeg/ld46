use crate::{
    color,
    effect::ScreenFlash,
    entity::Lifetime,
    lives::Lives,
    money::{Money, Wallet},
    movement::*,
    particle::ParticleEmitter,
    phase::Phase,
    physics::*,
    player::Player,
    projectile::{Projectile, ProjectileEmitter},
    random,
    ship::Ships,
    sprite::{RotationFollowsVelocity, Sprites},
};
use derive_deref::{Deref, DerefMut};
use specs_blit::{specs::*, Sprite, SpriteRef};

const ENEMY_ENGINE_PARTICLE_LIFETIME: f64 = 10.0;
const ENEMY_DEAD_EMITTER_LIFETIME: f64 = 5.0;
const ENEMY_DEAD_PARTICLE_LIFETIME: f64 = 10.0;

const TIME_RANDOM_FACTOR: f64 = 10.0;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EnemyType {
    Small,
    Medium,
    Big,
}

impl EnemyType {
    pub fn random() -> Self {
        if random::range(0.0, 1.0) < 0.1 {
            EnemyType::Big
        } else if random::range(0.0, 1.0) < 0.2 {
            EnemyType::Medium
        } else {
            EnemyType::Small
        }
    }

    pub fn bb(self) -> BoundingBox {
        match self {
            EnemyType::Small => BoundingBox::new(10.0, 16.0),
            EnemyType::Medium => BoundingBox::new(13.0, 20.0),
            EnemyType::Big => BoundingBox::new(22.0, 24.0),
        }
    }

    pub fn sprite(self, ships: &Ships) -> SpriteRef {
        ships.enemy(self)
    }

    pub fn projectile_sprite(self, sprites: &Sprites) -> (SpriteRef, f64, f64) {
        match self {
            EnemyType::Small => sprites.small_projectile(),
            _ => sprites.big_projectile(),
        }
    }

    pub fn speed_x(self) -> f64 {
        match self {
            EnemyType::Small => random::range(1.0, 1.5),
            EnemyType::Medium => random::range(0.3, 0.5),
            EnemyType::Big => random::range(0.3, 0.4),
        }
    }

    pub fn speed_y(self) -> f64 {
        match self {
            EnemyType::Small => random::range(-0.8, 0.8),
            EnemyType::Medium => random::range(-0.3, 0.3),
            EnemyType::Big => random::range(-0.4, 0.4),
        }
    }

    pub fn shoot_interval(self) -> f64 {
        (match self {
            EnemyType::Small => random::range(2.0, 4.0),
            EnemyType::Medium => random::range(0.8, 1.0),
            EnemyType::Big => random::range(3.0, 4.0),
        }) * 60.0
    }

    pub fn shoot_spread(self) -> f64 {
        match self {
            EnemyType::Small => random::range(0.0, 0.2),
            EnemyType::Medium => random::range(0.6, 0.8),
            EnemyType::Big => random::range(1.0, 2.0),
        }
    }

    pub fn shoot_split_into(self, sprites: &Sprites) -> Option<SpriteRef> {
        match self {
            EnemyType::Small => None,
            _ => Some(sprites.small_projectile().0),
        }
    }

    pub fn money(self) -> usize {
        (match self {
            EnemyType::Small => random::range(20.0, 30.0),
            EnemyType::Medium => random::range(50.0, 80.0),
            EnemyType::Big => random::range(100.0, 150.0),
        }) as usize
    }

    pub fn particle_amount(self) -> u8 {
        match self {
            EnemyType::Small => 1,
            EnemyType::Medium => 4,
            EnemyType::Big => 12,
        }
    }

    pub fn spawn_rest_before(self) -> f64 {
        match self {
            EnemyType::Small => 0.0,
            EnemyType::Medium => 0.5 * 60.0,
            EnemyType::Big => 1.0 * 60.0,
        }
    }

    pub fn spawn_rest_after(self) -> f64 {
        match self {
            EnemyType::Small => 0.0,
            EnemyType::Medium => 3.0 * 60.0,
            EnemyType::Big => 4.0 * 60.0,
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct EnemyEmitter {
    /// Time, resources for enemy.
    spawner: Vec<(f64, EnemyType)>,
    current_time: f64,
    total_time: f64,
}

impl EnemyEmitter {
    pub fn new(level: Option<usize>) -> Self {
        if level.is_none() {
            // Big ship is spawning these
            return Self {
                spawner: vec![
                    (2.0 * 60.0, EnemyType::Small),
                    (3.5 * 60.0, EnemyType::Small),
                    (5.0 * 60.0, EnemyType::Small),
                    (7.0 * 60.0, EnemyType::Small),
                ],
                current_time: 0.0,
                total_time: 10.0 * 60.0,
            };
        }

        let level = level.unwrap();
        match level {
            1 => {
                return Self {
                    spawner: vec![(30.0, EnemyType::Small)],
                    current_time: 0.0,
                    total_time: 5.0 * 60.0,
                };
            }
            2 => {
                return Self {
                    spawner: vec![(30.0, EnemyType::Small), (120.0, EnemyType::Small)],
                    current_time: 0.0,
                    total_time: 5.0 * 60.0,
                };
            }
            3 => {
                return Self {
                    spawner: vec![(30.0, EnemyType::Medium)],
                    current_time: 0.0,
                    total_time: 5.0 * 60.0,
                };
            }
            4 => {
                return Self {
                    spawner: vec![(30.0, EnemyType::Big)],
                    current_time: 0.0,
                    total_time: 5.0 * 60.0,
                };
            }
            _ => (),
        }

        let total_time = (level as f64 * 10.0 - 5.0) * 60.0;

        // Spread it out over time
        let amount_of_enemies = (level * level) / 2;
        let time_dist = total_time / amount_of_enemies as f64;
        let mut rest = 0.0;
        let mut spawner = (0..amount_of_enemies)
            .map(|index| {
                let type_ = EnemyType::random();
                rest += type_.spawn_rest_before();

                let result = (
                    index as f64
                        * (time_dist + random::range(-TIME_RANDOM_FACTOR, TIME_RANDOM_FACTOR))
                        + rest,
                    type_,
                );

                rest += type_.spawn_rest_after();

                result
            })
            .collect::<Vec<_>>();

        // Always spawn the first one immediately
        if let Some(mut first) = spawner.first_mut() {
            first.0 = 30.0;
        }

        Self {
            spawner,
            current_time: 0.0,
            total_time,
        }
    }

    pub fn spawn_enemy_with_resource_usage(
        entities: &Entities,
        updater: &LazyUpdate,
        sprites: &Sprites,
        ships: &Ships,
        type_: EnemyType,
        pos: &Option<&Position>,
    ) {
        let enemy = entities.create();
        updater.insert(enemy, Enemy);

        if type_ == EnemyType::Big {
            updater.insert(enemy, EnemyEmitter::new(None));
        }

        let bb = type_.bb();

        updater.insert(
            enemy,
            match pos {
                Some(pos) => (*pos).clone(),
                None => Position::new(
                    crate::WIDTH as f64 - 10.0,
                    random::range(0.0, crate::HEIGHT as f64 - bb.y),
                ),
            },
        );

        updater.insert(enemy, Sprite::new(type_.sprite(&ships)));
        updater.insert(enemy, RotationFollowsVelocity);

        let speed_x = type_.speed_x();
        let speed_y = type_.speed_y();

        if random::bool() {
            // Straight pattern
            updater.insert(enemy, Velocity::new(-speed_x, speed_y));
        } else {
            // Zigzag pattern
            updater.insert(enemy, Velocity::new(-speed_x, 0.0));
            updater.insert(enemy, Zigzag::new(speed_y, random::range(0.001, 0.2)));
        }
        if speed_x > 0.7 {
            updater.insert(
                enemy,
                ParticleEmitter::new(
                    ENEMY_ENGINE_PARTICLE_LIFETIME,
                    sprites.white_particle.clone(),
                )
                .with_amount(type_.particle_amount())
                .with_dispersion(speed_x * 0.2)
                .with_offset(bb.center_offset()),
            );
        }

        let (proj_sprite, proj_width, proj_height) = type_.projectile_sprite(&sprites);

        // Shoot bullets
        updater.insert(
            enemy,
            ProjectileEmitter::new(proj_sprite, BoundingBox::new(proj_width, proj_height))
                .with_speed(speed_x + 2.0)
                .with_spread(type_.shoot_spread())
                .with_interval(type_.shoot_interval())
                .with_offset(bb.center_offset())
                .split_into(type_.shoot_split_into(&sprites)),
        );

        updater.insert(enemy, bb);

        // The rest of the resources is the leftover money
        updater.insert(enemy, Money::new(type_.money()));
    }

    pub fn enemies_left(&self) -> usize {
        self.spawner.len()
    }
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Enemy;

#[derive(Debug, Default, Deref, DerefMut)]
pub struct EnemiesLeft(pub usize);

pub struct EnemySystem;
impl<'a> System<'a> for EnemySystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, Phase>,
        Option<Write<'a, Lives>>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, Position>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut phase, lives, enemy, pos, updater): Self::SystemData) {
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

        if *phase == Phase::WaitingForLastEnemy && enemy.is_empty() {
            *phase = Phase::SwitchTo(Box::new(Phase::Setup));
        }
    }
}

pub struct EnemyCollisionSystem;
impl<'a> System<'a> for EnemyCollisionSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, Wallet>,
        ReadExpect<'a, Sprites>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Projectile>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, BoundingBox>,
        ReadStorage<'a, Money>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (entities, mut wallet, sprites, enemy, player, projectile, pos, vel, bb, money, updater): Self::SystemData,
    ) {
        // Check for collision with the player
        for (player_pos, player_bb, _) in (&pos, &bb, &player).join() {
            let player_aabr = player_bb.to_aabr(player_pos);
            for (entity, enemy_pos, enemy_bb, money, _) in
                (&*entities, &pos, &bb, (&money).maybe(), &enemy).join()
            {
                let enemy_aabr = enemy_bb.to_aabr(enemy_pos);

                if enemy_aabr.collides_with_aabr(player_aabr) {
                    // Remove the enemy
                    let _ = entities.delete(entity);

                    if let Some(money) = money {
                        wallet.add(money);
                    }

                    let emitter = entities.create();
                    updater.insert(
                        emitter,
                        ParticleEmitter::new(
                            ENEMY_DEAD_PARTICLE_LIFETIME,
                            sprites.white_particle.clone(),
                        )
                        .with_dispersion(3.0)
                        .with_amount(8),
                    );
                    updater.insert(emitter, Position::from_vec2(enemy_aabr.center()));
                    updater.insert(emitter, Position::from_vec2(enemy_aabr.center()));
                    updater.insert(emitter, Lifetime::new(ENEMY_DEAD_EMITTER_LIFETIME));
                }
            }
        }
        // Check for collision with the projectile
        for (projectile_entity, projectile_pos, projectile_bb, projectile_vel, _) in
            (&*entities, &pos, &bb, &vel, &projectile).join()
        {
            if projectile_vel.x > 0.0 {
                let projectile_aabr = projectile_bb.to_aabr(projectile_pos);
                for (enemy_entity, enemy_pos, enemy_bb, money, _) in
                    (&*entities, &pos, &bb, (&money).maybe(), &enemy).join()
                {
                    let enemy_aabr = enemy_bb.to_aabr(enemy_pos);

                    if enemy_aabr.collides_with_aabr(projectile_aabr) {
                        let _ = entities.delete(projectile_entity);
                        let _ = entities.delete(enemy_entity);

                        if let Some(money) = money {
                            wallet.add(money);
                        }

                        let emitter = entities.create();
                        updater.insert(
                            emitter,
                            ParticleEmitter::new(
                                ENEMY_DEAD_PARTICLE_LIFETIME,
                                sprites.white_particle.clone(),
                            )
                            .with_dispersion(3.0)
                            .with_amount(4),
                        );
                        updater.insert(emitter, Position::from_vec2(enemy_aabr.center()));
                        updater.insert(emitter, Position::from_vec2(enemy_aabr.center()));
                        updater.insert(emitter, Lifetime::new(ENEMY_DEAD_EMITTER_LIFETIME));
                    }
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
        Write<'a, EnemiesLeft>,
        WriteStorage<'a, EnemyEmitter>,
        ReadStorage<'a, Position>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (entities, sprites, ships, mut phase, mut enemies_left, mut emitter, pos, updater): Self::SystemData,
    ) {
        if let Some(ships) = ships {
            enemies_left.0 = 0;

            for (entity, emitter, pos) in (&*entities, &mut emitter, (&pos).maybe()).join() {
                enemies_left.0 += emitter.enemies_left();
                if emitter.current_time >= emitter.total_time {
                    // Time ran out, change to another phase
                    if pos.is_none() {
                        // Only delete the emitters that are not attached, the other ones will be
                        // deleted automatically
                        let _ = entities.delete(entity);
                    }
                    continue;
                }
                emitter.current_time += 1.0;

                if let Some((time, type_)) = emitter.spawner.first() {
                    if *time < emitter.current_time {
                        EnemyEmitter::spawn_enemy_with_resource_usage(
                            &entities, &updater, &sprites, &ships, *type_, &pos,
                        );

                        emitter.spawner.remove(0);
                    }
                }
            }

            if *phase == Phase::Play && emitter.is_empty() {
                *phase = Phase::WaitingForLastEnemy;
            }
        }
    }
}
