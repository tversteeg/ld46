use specs_blit::{
    specs::{Component, DenseVecStorage, Join, ReadStorage, System, Write},
    PixelBuffer,
};

/// A particle that moves around but doesn't collide.
#[derive(Component, Debug, Default)]
pub struct ScreenFlash(u32);

impl ScreenFlash {
    /// Flash the screen with a color.
    ///
    /// Used in conjunction with entity::Lifetime.
    pub fn new(color: u32) -> Self {
        Self(color)
    }
}

/// System that will flash the screen.
pub struct ScreenFlashSystem;
impl<'a> System<'a> for ScreenFlashSystem {
    type SystemData = (Write<'a, PixelBuffer>, ReadStorage<'a, ScreenFlash>);

    fn run(&mut self, (mut buffer, flash): Self::SystemData) {
        for flash in flash.join() {
            buffer.clear(flash.0)
        }
    }
}
