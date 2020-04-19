use crate::random;
use derive_deref::{Deref, DerefMut};
use specs_blit::specs::{Component, Join, ReadStorage, System, VecStorage, WriteStorage};

type Vec2 = vek::Vec2<f64>;
type Aabr = vek::Aabr<f64>;

#[derive(Component, Debug, Default, Deref, DerefMut, Clone)]
#[storage(VecStorage)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn from_vec2(v: Vec2) -> Self {
        Self(v)
    }

    pub fn add_offset(&self, offset: Vec2) -> Self {
        Self(self.0 + offset)
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
        let rand_x = random::range(-range, range);
        let rand_y = random::range(-range, range);

        Self(Vec2::new(rand_x, rand_y))
    }
}

#[derive(Component, Debug, Default, Deref, DerefMut, Clone)]
#[storage(VecStorage)]
pub struct Drag(pub f64);

#[derive(Component, Debug, Default, Deref, DerefMut, Clone)]
#[storage(VecStorage)]
pub struct Speed(pub f64);

#[derive(Component, Debug, Default, Deref, DerefMut, Clone)]
#[storage(VecStorage)]
pub struct BoundingBox(pub Vec2);

impl BoundingBox {
    pub fn new(width: f64, height: f64) -> Self {
        Self(Vec2::new(width, height))
    }

    pub fn to_aabr(&self, pos: &Position) -> Aabr {
        Aabr {
            min: pos.0,
            max: pos.0 + self.0,
        }
    }

    pub fn center_offset(&self) -> Vec2 {
        self.to_aabr(&Position::new(0.0, 0.0)).center()
    }
}

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

pub struct BoundingBoxSystem;
impl<'a> System<'a> for BoundingBoxSystem {
    type SystemData = (
        ReadStorage<'a, BoundingBox>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (bb, mut vel, mut pos): Self::SystemData) {
        for (bb, pos, vel) in (&bb, &mut pos, (&mut vel).maybe()).join() {
            if pos.y < 0.0 {
                // Collide with top
                pos.y = 0.0;
                if let Some(vel) = vel {
                    vel.y = vel.y.abs();
                }
            } else if pos.y + bb.y > crate::HEIGHT as f64 {
                // Collide with bottom
                pos.y = crate::HEIGHT as f64 - bb.y;
                if let Some(vel) = vel {
                    vel.y = -vel.y.abs();
                }
            }
        }
    }
}
