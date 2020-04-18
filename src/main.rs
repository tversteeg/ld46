mod color;
mod effect;
mod enemy;
mod entity;
mod gui;
mod input;
mod lives;
mod movement;
mod particle;
mod phase;
mod physics;
mod player;
mod random;
mod render;
mod ship;
mod sprite;

use crate::{
    enemy::EnemiesLeft, gui::Gui, input::Input, lives::Lives, phase::Phase, render::Render,
};
use anyhow::Result;
use miniquad::{
    conf::{Conf, Loading},
    Context, EventHandler, KeyCode, KeyMods, MouseButton, UserData,
};
use specs_blit::{specs::prelude::*, PixelBuffer};

pub const WIDTH: usize = 400;
pub const HEIGHT: usize = 300;

const LEVEL_TIME_SCALE: f64 = 120.0;
const LEVEL_RESOURCES_SCALE: f64 = 100.0;

/// Our game state.
struct Game<'a, 'b> {
    /// The specs world.
    world: World,
    /// The specs dispatcher, it needs these lifetimes.
    dispatcher: Dispatcher<'a, 'b>,
    /// Our wrapper around the OpenGL calls.
    render: Render,

    level: f64,
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

        world.register::<movement::Zigzag>();

        world.register::<particle::Particle>();
        world.register::<particle::ParticleEmitter>();

        world.register::<entity::Lifetime>();

        world.register::<effect::ScreenFlash>();

        // Load the sprite rendering component
        world.register::<specs_blit::Sprite>();

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

        // Setup the dispatcher with the blit system
        let dispatcher = DispatcherBuilder::new()
            .with(particle::ParticleEmitterSystem, "particle_emitter", &[])
            .with(entity::LifetimeSystem, "lifetime", &[])
            .with(player::PlayerSystem, "player", &[])
            .with(enemy::EnemySystem, "enemy", &[])
            .with(enemy::EnemyEmitterSystem, "enemy_emitter", &[])
            .with(movement::MovementSystem, "movement", &[])
            .with(physics::VelocitySystem, "velocity", &["player", "movement"])
            .with(physics::DragSystem, "drag", &["velocity"])
            .with(physics::BoundingBoxSystem, "bb", &["velocity"])
            .with(enemy::EnemyCollisionSystem, "enemy_collision", &["bb"])
            .with(sprite::SpritePositionSystem, "sprite_pos", &["velocity"])
            .with_thread_local(specs_blit::RenderSystem)
            .with_thread_local(effect::ScreenFlashSystem)
            .build();

        // Setup the OpenGL render part
        let render = Render::new(ctx, WIDTH, HEIGHT);

        // Load some sprites
        world.insert(sprite::Sprites::generate().expect("Could not generate sprites"));

        let mut game = Self {
            world,
            dispatcher,
            render,
            level: 1.0,
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
                // Generate the ships
                self.world.insert(ship::Ships::generate());

                self.switch_phase(Phase::Play);
            }
            Phase::Setup => {
                self.level += 1.0;
            }
            Phase::Play => {
                self.world
                    .create_entity()
                    .with(effect::ScreenFlash::new(color::FOREGROUND))
                    .with(entity::Lifetime::new(5.0))
                    .build();

                self.world.insert(Lives::new(3));

                self.world
                    .create_entity()
                    .with(enemy::EnemyEmitter::new(
                        200.0 + self.level * LEVEL_RESOURCES_SCALE,
                        20.0 * 60.0 + self.level * LEVEL_TIME_SCALE,
                    ))
                    .build();

                // Spawn the paddle
                player::spawn_player(&mut self.world).expect("Couldn't spawn player");
            }
            _ => (),
        }
    }

    pub fn render_phase(&mut self) {
        let phase = self.world.read_resource::<Phase>();

        let mut buffer = self.world.write_resource::<PixelBuffer>();
        let mut gui = self.world.write_resource::<Gui>();
        match *phase {
            Phase::Menu => {
                // Render the GUI
                gui.draw_label(&mut buffer, "Press SPACE to play!", 110, 200);
            }
            Phase::Setup => {
                gui.draw_label(&mut buffer, format!("Level: {}", self.level), 160, 20);
                gui.draw_label(&mut buffer, "Press SPACE to start!", 105, 200);
            }
            Phase::Play | Phase::WaitingForLastEnemy => {
                let lives = self.world.read_resource::<Lives>();
                lives.render(&mut buffer, 100, 5);

                gui.draw_label(&mut buffer, format!("Level: {}", self.level), 20, 5);

                if *phase == Phase::Play {
                    gui.draw_label(
                        &mut buffer,
                        format!("Enemies: {}", self.world.read_resource::<EnemiesLeft>().0),
                        200,
                        5,
                    );
                }
            }
            Phase::GameOver => {
                gui.draw_label(&mut buffer, "GAME OVER!", 20, 20);
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

        // Clear the buffer with a black color
        buffer.clear(color::BACKGROUND);
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
        // Start the game when space is pressed
        let phase = (*self.world.read_resource::<Phase>()).clone();
        if phase == Phase::Menu && keycode == KeyCode::Space {
            self.switch_phase(Phase::Initialize);
        } else if phase == Phase::Setup && keycode == KeyCode::Space {
            self.switch_phase(Phase::Play);
        }

        // Pass the input to the resource
        (*self.world.write_resource::<Input>()).handle_key(keycode, true);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
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
            window_width: WIDTH as i32 * 2,
            window_height: HEIGHT as i32 * 2,
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
