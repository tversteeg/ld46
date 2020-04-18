use crate::physics::*;
use specs_blit::specs::*;

#[derive(Component, Debug)]
pub struct Zigzag {
    time: f64,
    amount: f64,
    time_div: f64,
}

impl Zigzag {
    pub fn new(amount: f64, time_div: f64) -> Self {
        Self {
            time: 0.0,
            amount,
            time_div,
        }
    }
}

pub struct MovementSystem;
impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Velocity>, WriteStorage<'a, Zigzag>);

    fn run(&mut self, (mut vel, mut zigzag): Self::SystemData) {
        for (vel, zigzag) in (&mut vel, &mut zigzag).join() {
            zigzag.time += 1.0;
            vel.0.y = (zigzag.time * zigzag.time_div).sin() * zigzag.amount;
        }
    }
}
