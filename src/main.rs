/* mod audio; */
mod input;
mod physics;
mod player;
mod render;
mod sprite;

use crate::{input::Input, render::Render};
use anyhow::Result;
use miniquad::{
    conf::{Conf, Loading},
    Context, EventHandler, KeyCode, KeyMods, UserData,
};
use specs_blit::{specs::prelude::*, PixelBuffer};

const WIDTH: usize = 400;
const HEIGHT: usize = 300;

/// Our game state.
struct Game<'a, 'b> {
    /// The specs world.
    world: World,
    /// The specs dispatcher, it needs these lifetimes.
    dispatcher: Dispatcher<'a, 'b>,
    /// Our wrapper around the OpenGL calls.
    render: Render,
}

impl<'a, 'b> Game<'a, 'b> {
    /// Setup the ECS and load the systems.
    pub fn new(ctx: &mut Context) -> Result<Self> {
        // Setup the ECS system
        let mut world = World::new();

        // Load the game components
        world.register::<physics::Position>();
        world.register::<physics::Velocity>();
        world.register::<physics::Speed>();

        world.register::<player::Player>();

        // Load the sprite rendering component
        world.register::<specs_blit::Sprite>();

        // Add the pixel buffer as a resource so it can be accessed from the RenderSystem later, to be
        // updated every frame
        world.insert(PixelBuffer::new(WIDTH, HEIGHT));

        // Add the input system
        world.insert(Input::default());

        // Add the audio system
        /* world.insert(Audio::new()); */

        // Setup the dispatcher with the blit system
        let dispatcher = DispatcherBuilder::new()
            .with(player::PlayerSystem, "player", &[])
            .with(physics::VelocitySystem, "velocity", &["player"])
            .with(sprite::SpritePositionSystem, "sprite_pos", &["velocity"])
            .with_thread_local(specs_blit::RenderSystem)
            .build();

        /*
        {
            // Start the audio
            let mut audio = world.write_resource::<Audio>();
            audio.run();
        }
        */

        // Spawn the initial game elements
        player::spawn_player(&mut world)?;

        // Setup the OpenGL render part
        let render = Render::new(ctx, WIDTH, HEIGHT);

        Ok(Self {
            world,
            dispatcher,
            render,
        })
    }
}

impl<'a, 'b> EventHandler for Game<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context) {
        // Update specs
        self.dispatcher.dispatch(&self.world);

        // Add/remove entities added in dispatch through `LazyUpdate`
        self.world.maintain();
    }

    fn draw(&mut self, ctx: &mut Context) {
        // Get the pixel buffer to render it
        let mut buffer = self.world.write_resource::<PixelBuffer>();

        // Render the buffer
        self.render.render(ctx, &buffer);

        // Clear the buffer with a black color
        buffer.clear(0xFF000000);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        // Pass the input to the resource
        (*self.world.write_resource::<Input>()).handle_key(keycode, false);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        // Pass the input to the resource
        (*self.world.write_resource::<Input>()).handle_key(keycode, true);
    }
}

fn main() {
    miniquad::start(
        Conf {
            window_title: concat!("replace_me - ", env!("CARGO_PKG_VERSION")).to_string(),
            window_width: WIDTH as i32,
            window_height: HEIGHT as i32,
            loading: Loading::Embedded,
            ..Default::default()
        },
        |mut ctx| {
            UserData::owning(
                Game::new(&mut ctx).expect("Setting up game state failed"),
                ctx,
            )
        },
    );
}
