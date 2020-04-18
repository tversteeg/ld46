use crate::{
    color, effect::ScreenFlash, entity::Lifetime, lives::Lives, physics::*, player::Player, random,
};
use specs_blit::{specs::*, Sprite, SpriteRef};

type Vec2 = vek::Vec2<f64>;

/// A projectile that moves around but doesn't collide.
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Projectile;

/// A component that emits projectiles while it lives.
#[derive(Component, Debug)]
pub struct ProjectileEmitter {
    interval: f64,
    current_interval: f64,
    speed: f64,
    spread: f64,
    /// The sprite to use.
    sprite: SpriteRef,
    /// Optional offset.
    offset: Vec2,
    size: BoundingBox,
}

impl ProjectileEmitter {
    pub fn new(sprite: SpriteRef, size: BoundingBox) -> Self {
        Self {
            speed: 1.0,
            spread: 1.0,
            sprite,
            interval: random::range(200.0, 400.0),
            current_interval: 0.0,
            offset: Vec2::new(0.0, 0.0),
            size,
        }
    }

    pub fn with_speed(mut self, speed: f64) -> Self {
        self.speed = speed;

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
        WriteStorage<'a, ProjectileEmitter>,
        ReadStorage<'a, Position>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut emitter, pos, updater): Self::SystemData) {
        for (emitter, pos) in (&mut emitter, &pos).join() {
            emitter.current_interval += 1.0;
            if emitter.current_interval > emitter.interval {
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
            }
        }
    }
}

pub struct ProjectileSystem;
impl<'a> System<'a> for ProjectileSystem {
    type SystemData = (
        Entities<'a>,
        Option<Write<'a, Lives>>,
        ReadStorage<'a, Projectile>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, BoundingBox>,
        WriteStorage<'a, Velocity>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (entities, lives, projectile, pos, player, bb, mut vel, updater): Self::SystemData,
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
            for (projectile_pos, projectile_bb, projectile_vel, _) in
                (&pos, &bb, &mut vel, &projectile).join()
            {
                let projectile_aabr = projectile_bb.to_aabr(projectile_pos);

                if projectile_aabr.collides_with_aabr(player_aabr) {
                    let speed = projectile_vel.magnitude();
                    let angle = (projectile_pos.0 - player_aabr.center() - Vec2::new(-20.0, 0.0))
                        .normalized();
                    projectile_vel.0 = angle * speed;
                }
            }
        }
    }
}
