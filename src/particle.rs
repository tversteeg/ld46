use crate::{
    entity::Lifetime,
    physics::{Position, Velocity},
};
use specs_blit::{
    specs::{
        Component, DenseVecStorage, Entities, Join, LazyUpdate, NullStorage, Read, ReadStorage,
        System,
    },
    Sprite, SpriteRef,
};

type Vec2 = vek::Vec2<f64>;

/// A particle that moves around but doesn't collide.
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Particle;

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
    /// Optional offset.
    offset: Vec2,
}

impl ParticleEmitter {
    pub fn new(lifetime: f64, sprite: SpriteRef) -> Self {
        Self {
            amount: 1.0,
            dispersion: 0.5,
            lifetime,
            sprite,
            offset: Vec2::new(0.0, 0.0),
        }
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;

        self
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
            updater.insert(particle, Particle);
            // Set the destruction time of the particle
            updater.insert(particle, Lifetime::new(emitter.lifetime));
            // Clone the position of the emitter
            updater.insert(particle, pos.add_offset(emitter.offset));
            // Add a new random velocity
            updater.insert(particle, Velocity::from_random_range(emitter.dispersion));
            // Use the sprite reference of the emitter
            updater.insert(particle, Sprite::new(emitter.sprite.clone()));
        }
    }
}
