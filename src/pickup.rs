use specs_blit::{specs::*, Sprite, SpriteRef};

#[derive(Component, Debug)]
pub struct UpgradeEmitter {}

impl UpgradeEmitter {
    pub fn new() -> Self {
        Self {}
    }
}

/// System that will spawn particles.
pub struct UpgradeEmitterSystem;
impl<'a> System<'a> for ParticleEmitterSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, ParticleEmitter>,
        ReadStorage<'a, Position>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, emitter, pos, updater): Self::SystemData) {
        for (emitter, pos) in (&emitter, &pos).join() {
            for _ in 0..emitter.amount {
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
}
