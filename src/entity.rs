use specs_blit::specs::{Component, Entities, Join, System, VecStorage, WriteStorage};

/// Component for something that will be destroyed when the time is up.
#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Lifetime(f64);

impl Lifetime {
    pub fn new(time: f64) -> Self {
        Self(time)
    }
}

/// System handles life.
pub struct LifetimeSystem;
impl<'a> System<'a> for LifetimeSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, Lifetime>);

    fn run(&mut self, (entities, mut lifetime): Self::SystemData) {
        for (entity, lifetime) in (&*entities, &mut lifetime).join() {
            lifetime.0 -= 1.0;
            if lifetime.0 <= 0.0 {
                // Remove the lifetime entity when it's dead
                let _ = entities.delete(entity);
            }
        }
    }
}
