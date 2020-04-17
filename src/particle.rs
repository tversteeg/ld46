use crate::physics::{Position, Velocity};
use specs_blit::{
    specs::{
        Component, DenseVecStorage, Entities, Join, LazyUpdate, Read, ReadStorage, System,
        VecStorage, WriteStorage,
    },
    Sprite, SpriteRef,
};

/// A particle that moves around but doesn't collide.
#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Particle {
    /// When this is zero the particle will be destroyed.
    time_left: f64,
}

/// A component that emits particles while it lives.
#[derive(Component, Debug)]
pub struct ParticleEmitter {
    /// Amount of particles to emit every second.
    amount: f64,
    /// How long the particle lifes.
    lifetime: f64,
    /// The maximum velocity of the particles.
    dispersion: f64,
    /// The sprite to use.
    sprite: SpriteRef,
}

impl ParticleEmitter {
    pub fn new(lifetime: f64, sprite: SpriteRef) -> Self {
        Self {
            amount: 1.0,
            dispersion: 0.5,
            lifetime,
            sprite,
        }
    }
}

/// System handles particle life.
pub struct ParticleSystem;
impl<'a> System<'a> for ParticleSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, Particle>);

    fn run(&mut self, (entities, mut particle): Self::SystemData) {
        for (entity, particle) in (&*entities, &mut particle).join() {
            particle.time_left -= 1.0;
            if particle.time_left <= 0.0 {
                // Remove the particle entity when it's dead
                let _ = entities.delete(entity);
            }
        }
    }
}

/// System that will spawn particles.
pub struct ParticleEmitterSystem;
impl<'a> System<'a> for ParticleEmitterSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, ParticleEmitter>,
        ReadStorage<'a, Position>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, emitter, pos, updater): Self::SystemData) {
        for (emitter, pos) in (&emitter, &pos).join() {
            // Spawn a new particle
            let particle = entities.create();
            updater.insert(
                particle,
                Particle {
                    time_left: emitter.lifetime,
                },
            );
            // Clone the position of the emitter
            updater.insert(particle, pos.clone());
            // Add a new random velocity
            updater.insert(particle, Velocity::from_random_range(emitter.dispersion));
            // Use the sprite reference of the emitter
            updater.insert(particle, Sprite::new(emitter.sprite.clone()));
        }
    }
}
