use derive_deref::{Deref, DerefMut};
use specs_blit::specs::{Component, DenseVecStorage, Join, ReadStorage, System, WriteStorage};

type Vec2 = vek::Vec2<f64>;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Vec2::new(x, y))
    }
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Vec2::new(x, y))
    }
}

pub struct VelocitySystem;
impl<'a> System<'a> for VelocitySystem {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.0 += vel.0;
        }
    }
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Speed(pub f64);
