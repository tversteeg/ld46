mod background;
mod color;
mod effect;
mod enemy;
mod entity;
mod gui;
mod input;
mod lives;
mod money;
mod movement;
mod particle;
mod phase;
mod physics;
mod player;
mod projectile;
mod random;
mod render;
mod ship;
mod sprite;
mod upgrade;

use crate::{
    background::Background, enemy::EnemiesLeft, gui::Gui, input::Input, lives::Lives,
    money::Wallet, phase::Phase, physics::Position, render::Render, sprite::Sprites,
    upgrade::Upgrades,
};
use anyhow::Result;
use miniquad::{
    conf::{Conf, Loading},
    Context, EventHandler, MouseButton, UserData,
};
use specs_blit::{specs::prelude::*, PixelBuffer, Sprite};

pub const WIDTH: usize = 400;
pub const HEIGHT: usize = 300;

/// Our game state.
struct Game<'a, 'b> {
    /// The specs world.
    world: World,
    /// The specs dispatcher, it needs these lifetimes.
    dispatcher: Dispatcher<'a, 'b>,
    /// Our wrapper around the OpenGL calls.
    render: Render,

    level: usize,
    background: Background,
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
        world.register::<physics::Drag>();
        world.register::<physics::BoundingBox>();

        world.register::<player::Player>();

        world.register::<enemy::Enemy>();
        world.register::<enemy::EnemyEmitter>();

        world.register::<money::Money>();

        world.register::<movement::Zigzag>();

        world.register::<particle::Particle>();
        world.register::<particle::ParticleEmitter>();

        world.register::<projectile::Projectile>();
        world.register::<projectile::ProjectileEmitter>();
        world.register::<projectile::SplitInto>();

        world.register::<entity::Lifetime>();

        world.register::<upgrade::HoldProjectile>();

        world.register::<effect::ScreenFlash>();

        world.register::<sprite::RotationFollowsVelocity>();

        // Load the sprite rendering component
        world.register::<Sprite>();

        // Add the pixel buffer as a resource so it can be accessed from the RenderSystem later, to be
        // updated every frame
        world.insert(PixelBuffer::new(WIDTH, HEIGHT));

        // Add the input system
        world.insert(Input::default());

        // Add the gui system
        world.insert(Gui::new(WIDTH, HEIGHT));

        // The current phase
        world.insert(Phase::default());

        // Enemies left
        world.insert(EnemiesLeft::default());

        // Money
        world.insert(Wallet::default());

        // The upgrades
        world.insert(Upgrades::default());

        // Setup the dispatcher with the blit system
        let dispatcher = DispatcherBuilder::new()
            .with(
                projectile::ProjectileEmitterSystem,
                "projectile_emitter",
                &[],
            )
            .with(particle::ParticleEmitterSystem, "particle_emitter", &[])
            .with(entity::LifetimeSystem, "lifetime", &[])
            .with(player::PlayerSystem, "player", &[])
            .with(projectile::ProjectileSystem, "projectile", &["player"])
            .with(enemy::EnemySystem, "enemy", &[])
            .with(enemy::EnemyEmitterSystem, "enemy_emitter", &[])
            .with(movement::MovementSystem, "movement", &[])
            .with(physics::VelocitySystem, "velocity", &["player", "movement"])
            .with(physics::DragSystem, "drag", &["velocity"])
            .with(physics::BoundingBoxSystem, "bb", &["velocity"])
            .with(enemy::EnemyCollisionSystem, "enemy_collision", &["bb"])
            .with(sprite::SpritePositionSystem, "sprite_pos", &["velocity"])
            .with(sprite::SpriteRotationSystem, "sprite_rot", &["velocity"])
            .with_thread_local(specs_blit::RenderSystem)
            .with_thread_local(effect::ScreenFlashSystem)
            .build();

        // Setup the OpenGL render part
        let render = Render::new(ctx, WIDTH, HEIGHT);

        // Load some sprites
        world.insert(Sprites::generate().expect("Could not generate sprites"));

        let mut game = Self {
            world,
            dispatcher,
            render,
            level: 0,
            background: Background::new(),
        };
        game.switch_phase(Phase::default());

        Ok(game)
    }

    pub fn switch_phase(&mut self, phase: Phase) {
        {
            let mut old_phase = self.world.write_resource::<Phase>();
            *old_phase = phase.clone();
        }

        // Clear all entities
        self.world.delete_all();

        match phase {
            Phase::Menu => {}
            Phase::Initialize => {
                self.level = 1;
                self.world.write_resource::<Wallet>().reset();
                self.world.write_resource::<Upgrades>().reset();

                // Generate the ships
                self.world.insert(ship::Ships::generate());

                self.switch_phase(Phase::Play);
            }
            Phase::Setup => {
                self.level += 1;
            }
            Phase::Play => {
                self.world
                    .create_entity()
                    .with(effect::ScreenFlash::new(color::FOREGROUND))
                    .with(entity::Lifetime::new(5.0))
                    .build();

                // Render background planet
                let sprite = self.world.read_resource::<Sprites>().planet.clone();
                self.world
                    .create_entity()
                    .with(Sprite::new(sprite))
                    .with(Position::new(0.0, 0.0))
                    .build();

                self.world.insert(Lives::new(3));

                self.world
                    .create_entity()
                    .with(enemy::EnemyEmitter::new(Some(self.level)))
                    .build();

                // Spawn the paddle
                player::spawn_player(&mut self.world).expect("Couldn't spawn player");
            }
            _ => (),
        }
    }

    pub fn render_phase(&mut self) {
        let mut phase = self.world.write_resource::<Phase>();

        let mut buffer = self.world.write_resource::<PixelBuffer>();
        let mut gui = self.world.write_resource::<Gui>();
        match *phase {
            Phase::Menu => {
                // Render the GUI
                gui.draw_label(&mut buffer, "Click to play!", 130, 145);
            }
            Phase::Setup => {
                let input = self.world.read_resource::<Input>();
                gui.draw(&mut buffer, &input);

                let mut wallet = self.world.write_resource::<Wallet>();
                let mut upgrades = self.world.write_resource::<Upgrades>();
                upgrades.render(
                    &mut buffer,
                    &mut gui,
                    &mut wallet,
                    &mut phase,
                    &input,
                    self.level,
                );
            }
            Phase::Play | Phase::WaitingForLastEnemy => {
                let lives = self.world.read_resource::<Lives>();
                lives.render(&mut buffer, 20, 5);

                gui.draw_label(&mut buffer, format!("Level {}", self.level), 70, 5);

                gui.draw_label(
                    &mut buffer,
                    format!("Enemies {}", self.world.read_resource::<EnemiesLeft>().0),
                    150,
                    5,
                );

                gui.draw_label(
                    &mut buffer,
                    format!("Scrap {}", self.world.read_resource::<Wallet>().money()),
                    250,
                    5,
                );
            }
            Phase::GameOver => {
                gui.draw_label(&mut buffer, "GAME OVER!", 160, 130);
                gui.draw_label(&mut buffer, "Click to play again!", 120, 200);
            }
            _ => (),
        }
    }
}

impl<'a, 'b> EventHandler for Game<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context) {
        // Update specs
        self.dispatcher.dispatch(&self.world);

        // Add/remove entities added in dispatch through `LazyUpdate`
        self.world.maintain();

        let mut phase = (*self.world.read_resource::<Phase>()).clone();
        if (phase == Phase::Play || phase == Phase::WaitingForLastEnemy)
            && self.world.read_resource::<Lives>().is_dead()
        {
            phase = Phase::SwitchTo(Box::new(Phase::GameOver));
        }

        if let Phase::SwitchTo(new_phase) = phase {
            self.switch_phase(*new_phase);
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.render_phase();

        // Get the pixel buffer to render it
        let mut buffer = self.world.write_resource::<PixelBuffer>();

        // Render the buffer
        self.render.render(ctx, &buffer);

        self.background.copy(&mut buffer.pixels_mut());
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        // Start the game
        let phase = (*self.world.read_resource::<Phase>()).clone();
        if phase == Phase::Menu || phase == Phase::GameOver {
            self.switch_phase(Phase::Initialize);
        }

        (*self.world.write_resource::<Input>()).handle_mouse_button(true);
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        (*self.world.write_resource::<Input>()).handle_mouse_button(false);
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        // Translate the screen position to our canvas position
        let screen_size = ctx.screen_size();

        let x = x / screen_size.0 * WIDTH as f32;
        let y = y / screen_size.1 * HEIGHT as f32;

        (*self.world.write_resource::<Input>()).handle_mouse_move(x as i32, y as i32);
    }
}

#[cfg(target_os = "linux")]
extern "C" {
    // Seed random when on Linux
    fn srand(input: u32);
}
#[cfg(not(target_os = "linux"))]
fn srand(_: u32) {}

fn main() {
    unsafe {
        srand(miniquad::date::now() as u32);
    }

    miniquad::start(
        Conf {
            window_title: concat!("Fermi Paradox - ", env!("CARGO_PKG_VERSION")).to_string(),
            window_width: WIDTH as i32 * 3,
            window_height: HEIGHT as i32 * 3,
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
