use derive_deref::{Deref, DerefMut};
use specs_blit::specs::{Component, Join, ReadStorage, System, VecStorage, WriteStorage};

type Vec2 = vek::Vec2<f64>;

#[derive(Component, Debug, Default, Deref, DerefMut, Clone)]
#[storage(VecStorage)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Vec2::new(x, y))
    }
}

#[derive(Component, Debug, Default, Deref, DerefMut, Clone)]
#[storage(VecStorage)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Vec2::new(x, y))
    }

    /// Construct a new velocity where the X and Y velocity are randomly placed inside the supplied
    /// range.
    pub fn from_random_range(range: f64) -> Self {
        let rand_x =
            unsafe { miniquad::rand() as f64 / miniquad::RAND_MAX as f64 } * 2.0 - 1.0 * range;
        let rand_y =
            unsafe { miniquad::rand() as f64 / miniquad::RAND_MAX as f64 } * 2.0 - 1.0 * range;

        Self(Vec2::new(rand_x, rand_y))
    }
}

#[derive(Component, Debug, Default, Deref, DerefMut, Clone)]
#[storage(VecStorage)]
pub struct Drag(pub f64);

#[derive(Component, Debug, Default, Deref, DerefMut)]
#[storage(VecStorage)]
pub struct Speed(pub f64);

pub struct DragSystem;
impl<'a> System<'a> for DragSystem {
    type SystemData = (ReadStorage<'a, Drag>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (drag, mut vel): Self::SystemData) {
        for (drag, vel) in (&drag, &mut vel).join() {
            vel.0 *= drag.0;
        }
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
