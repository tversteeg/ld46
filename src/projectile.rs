use crate::{
    color, effect::ScreenFlash, entity::Lifetime, lives::Lives, particle::ParticleEmitter,
    physics::*, player::Player, random, sprite::Sprites, upgrade::Upgrades,
};
use specs_blit::{specs::*, Sprite, SpriteRef};

type Vec2 = vek::Vec2<f64>;

/// A projectile that moves around but doesn't collide.
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Projectile;

#[derive(Component, Debug)]
pub struct SplitInto(SpriteRef);

/// A component that emits projectiles while it lives.
#[derive(Component, Debug)]
pub struct ProjectileEmitter {
    interval: f64,
    current_interval: f64,
    speed: f64,
    spread: f64,
    /// The sprite to use.
    sprite: SpriteRef,

    split_into: Option<SpriteRef>,
    /// Optional offset.
    offset: Vec2,
    size: BoundingBox,
}

impl ProjectileEmitter {
    pub fn new(sprite: SpriteRef, size: BoundingBox) -> Self {
        let interval = random::range(200.0, 400.0);
        Self {
            speed: 1.0,
            spread: 1.0,
            sprite,
            interval,
            split_into: None,
            current_interval: random::range(0.0, interval),
            offset: Vec2::new(0.0, 0.0),
            size,
        }
    }

    pub fn with_interval(mut self, interval: f64) -> Self {
        self.interval = interval;
        self.current_interval = random::range(0.0, interval);

        self
    }

    pub fn with_speed(mut self, speed: f64) -> Self {
        self.speed = speed;

        self
    }

    pub fn with_spread(mut self, spread: f64) -> Self {
        self.spread = spread;

        self
    }

    pub fn split_into(mut self, split: Option<SpriteRef>) -> Self {
        self.split_into = split;

        self
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;

        self
    }
}

/// System that will spawn projectiles.
pub struct ProjectileEmitterSystem;
impl<'a> System<'a> for ProjectileEmitterSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Sprites>,
        WriteStorage<'a, ProjectileEmitter>,
        ReadStorage<'a, Position>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, sprites, mut emitter, pos, updater): Self::SystemData) {
        for (emitter, pos) in (&mut emitter, &pos).join() {
            emitter.current_interval += 1.0;
            if emitter.current_interval > emitter.interval && pos.x > 200.0 {
                emitter.current_interval = 0.0;

                // Spawn a new projectile
                let projectile = entities.create();
                updater.insert(projectile, Projectile);
                // Clone the position of the emitter
                updater.insert(projectile, pos.add_offset(emitter.offset));
                // Add a new random velocity
                updater.insert(
                    projectile,
                    Velocity::new(
                        -emitter.speed,
                        random::range(-emitter.spread, emitter.spread),
                    ),
                );
                updater.insert(projectile, emitter.size.clone());
                // Use the sprite reference of the emitter
                updater.insert(projectile, Sprite::new(emitter.sprite.clone()));

                if let Some(ref sprite) = emitter.split_into {
                    updater.insert(projectile, SplitInto(sprite.clone()));
                }

                updater.insert(
                    projectile,
                    ParticleEmitter::new(3.0, sprites.white_particle.clone())
                        .with_dispersion(1.0)
                        .with_offset(emitter.size.center_offset()),
                );
            }
        }
    }
}

pub struct ProjectileSystem;
impl<'a> System<'a> for ProjectileSystem {
    type SystemData = (
        Entities<'a>,
        Option<Write<'a, Lives>>,
        ReadExpect<'a, Sprites>,
        Read<'a, Upgrades>,
        ReadStorage<'a, Projectile>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, BoundingBox>,
        ReadStorage<'a, SplitInto>,
        WriteStorage<'a, Velocity>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            entities,
            lives,
            sprites,
            upgrades,
            projectile,
            pos,
            player,
            bb,
            split_into,
            mut vel,
            updater,
        ): Self::SystemData,
    ) {
        if let Some(mut lives) = lives {
            for (entity, pos, _) in (&*entities, &pos, &projectile).join() {
                if pos.0.x <= 0.0 {
                    lives.reduce();

                    let flash = entities.create();
                    updater.insert(flash, ScreenFlash::new(color::RED));
                    updater.insert(flash, Lifetime::new(5.0));

                    let _ = entities.delete(entity);
                }
            }
        }

        for (player_pos, player_bb, _) in (&pos, &bb, &player).join() {
            let player_aabr = player_bb.to_aabr(player_pos);
            for (entity, projectile_pos, projectile_bb, projectile_vel, projectile_split_into, _) in
                (
                    &*entities,
                    &pos,
                    &bb,
                    &mut vel,
                    (&split_into).maybe(),
                    &projectile,
                )
                    .join()
            {
                // Don't collide with projectiles already moving in the proper direction
                if projectile_vel.x > 0.0 {
                    continue;
                }

                let projectile_aabr = projectile_bb.to_aabr(projectile_pos);

                if projectile_aabr.collides_with_aabr(player_aabr) {
                    let speed = projectile_vel.magnitude();
                    let angle = (projectile_pos.0 - player_aabr.center() - Vec2::new(-20.0, 0.0))
                        .normalized();
                    let angle_rad = angle.y.atan2(angle.x);

                    projectile_vel.0 = angle * speed;
                    if upgrades.split {
                        if let Some(ref sprite) = projectile_split_into {
                            // Delete the source
                            let _ = entities.delete(entity);

                            for i in -1..2 {
                                let angle = angle_rad + i as f64 * 0.1;

                                let new_projectile = entities.create();
                                updater.insert(new_projectile, Projectile);
                                updater.insert(new_projectile, Sprite::new(sprite.0.clone()));
                                updater.insert(new_projectile, projectile_pos.clone());
                                updater.insert(new_projectile, projectile_bb.clone());

                                updater.insert(
                                    new_projectile,
                                    Velocity::new(angle.cos() * speed, angle.sin() * speed),
                                );

                                updater.insert(
                                    new_projectile,
                                    ParticleEmitter::new(3.0, sprites.white_particle.clone())
                                        .with_dispersion(1.0)
                                        .with_offset(projectile_bb.center_offset()),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
