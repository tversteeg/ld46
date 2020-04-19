use crate::{
    color,
    effect::ScreenFlash,
    entity::Lifetime,
    lives,
    lives::Lives,
    physics::{BoundingBox, Position, Velocity},
    projectile::Projectile,
    random,
};
use specs_blit::{specs::*, Sprite, SpriteRef};

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Pickup {
    Health,
}

#[derive(Component, Debug)]
pub struct PickupEmitter {
    current_time: f64,
    interval: f64,
    health_sprite: SpriteRef,
}

impl PickupEmitter {
    pub fn new() -> Self {
        Self {
            interval: random::range(15.0 * 60.0, 40.0 * 60.0),
            current_time: 0.0,
            health_sprite: specs_blit::load(lives::sprite()).expect("Could not create sprite"),
        }
    }

    pub fn sprite(&self, type_: Pickup) -> SpriteRef {
        match type_ {
            Pickup::Health => self.health_sprite.clone(),
        }
    }
}

/// System that will spawn pickups.
pub struct PickupEmitterSystem;
impl<'a> System<'a> for PickupEmitterSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, PickupEmitter>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut emitter, updater): Self::SystemData) {
        for mut emitter in (&mut emitter).join() {
            emitter.current_time += 1.0;
            if emitter.current_time >= emitter.interval {
                emitter.current_time = 0.0;

                let type_ = Pickup::Health;

                let pickup = entities.create();
                updater.insert(pickup, type_);
                updater.insert(
                    pickup,
                    Position::new(
                        crate::WIDTH as f64,
                        random::range(10.0, crate::HEIGHT as f64 - 25.0),
                    ),
                );
                updater.insert(pickup, Velocity::new(-0.5, 0.0));
                updater.insert(pickup, Sprite::new(emitter.sprite(type_)));
                updater.insert(pickup, BoundingBox::new(10.0, 10.0));
            }
        }
    }
}

pub struct PickupSystem;
impl<'a> System<'a> for PickupSystem {
    type SystemData = (
        Entities<'a>,
        Option<Write<'a, Lives>>,
        ReadStorage<'a, Pickup>,
        ReadStorage<'a, Projectile>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BoundingBox>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, lives, pickup, projectile, pos, bb, updater): Self::SystemData) {
        for (entity, pos, _) in (&*entities, &pos, &pickup).join() {
            if pos.0.x <= 0.0 {
                let _ = entities.delete(entity);
            }
        }

        if let Some(mut lives) = lives {
            for (pickup_entity, pickup_pos, pickup_bb, _) in (&*entities, &pos, &bb, &pickup).join()
            {
                let pickup_aabr = pickup_bb.to_aabr(pickup_pos);

                for (projectile_entity, projectile_pos, projectile_bb, _) in
                    (&*entities, &pos, &bb, &projectile).join()
                {
                    let projectile_aabr = projectile_bb.to_aabr(projectile_pos);

                    if pickup_aabr.collides_with_aabr(projectile_aabr) {
                        lives.increase();

                        let flash = entities.create();
                        updater.insert(flash, ScreenFlash::new(color::GREEN));
                        updater.insert(flash, Lifetime::new(3.0));

                        let _ = entities.delete(projectile_entity);
                        let _ = entities.delete(pickup_entity);
                    }
                }
            }
        }
    }
}
