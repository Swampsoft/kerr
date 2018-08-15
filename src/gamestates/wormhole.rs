use std::f32;

use ggez::{
    graphics::{self, DrawMode, MeshBuilder, Point2, Rect}, Context,
    GameResult,
};

use sdl2::keyboard::{Keycode, Scancode, Mod};

use specs::{Builder, Dispatcher, DispatcherBuilder, RunNow, World};

use super::{GameState, StateTransition};
use components::{Controlled, register_components, Pos, Sprite};
use inputstate::InputState;
use resources::Resources;
use systems::{InputSystem, SpriteRenderSystem};
use three_dee::projection_factor;

pub struct WormholeState {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,

    quit: bool,

    z_pos: f32,
}

impl WormholeState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut world = World::new();

        register_components(&mut world);

        let player_sprite = world.write_resource::<Resources>().add_image(ctx, "/ship_perspective.png")?;

        world
            .create_entity()
            .with(Pos::new(1.0, 0.0, 2.2))
            .with(Sprite(player_sprite))
            .with(Controlled);

        world
            .create_entity()
            .with(Pos::new(1.0, 0.1, 3.0))
            .with(Sprite(player_sprite));

        world
            .create_entity()
            .with(Pos::new(1.0, 0.2, 4.0))
            .with(Sprite(player_sprite));

        world
            .create_entity()
            .with(Pos::new(1.0, 0.3, 5.0))
            .with(Sprite(player_sprite));

        world
            .create_entity()
            .with(Pos::new(1.0, 0.4, 6.0))
            .with(Sprite(player_sprite));

        world
            .create_entity()
            .with(Pos::new(1.0, 0.5, 7.0))
            .with(Sprite(player_sprite));

        let dispatcher = DispatcherBuilder::new()
            .with(InputSystem, "input", &[])
            .build();

        let s = WormholeState {
            world,
            dispatcher,
            z_pos: 0.0,
            quit: false,
        };
        Ok(s)
    }
}

impl GameState for WormholeState {
    fn transition(&self) -> StateTransition {
        if self.quit {
            StateTransition::Pop
        } else {
            StateTransition::None
        }
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<bool> {
        self.dispatcher.dispatch(&self.world.res);

        self.z_pos -= 0.01;
        while self.z_pos <= 0.0 {
            self.z_pos += 2.0;
        }
        Ok(false)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_screen_coordinates(ctx, Rect::new(-2.0, -1.5, 4.0, 3.0))?;

        let mut mb = MeshBuilder::new();

        for z in 0..20 {
            let z = 0.1 + (z as f32 + self.z_pos % 1.0) * 1.0;

            let p = projection_factor(z);

            mb.circle(
                DrawMode::Line((0.03 * p).max(0.01)),
                Point2::new(0.0, 0.0) * p,
                1.0 * p,
                0.001,
            );
        }

        let p1 = projection_factor(self.z_pos * 10.0 );
        let p2 = projection_factor(self.z_pos * 10.0 + 1.0);

        mb.line(
            &[Point2::new(0.7, 0.7) * p1, Point2::new(0.7, 0.7) * p2],
            0.02 * p1,
        );
        mb.line(
            &[Point2::new(0.8, 0.7) * p1, Point2::new(0.8, 0.7) * p2],
            0.02 * p1,
        );

        let mesh = mb.build(ctx)?;

        graphics::draw(ctx, &mesh, Point2::new(0.0, 0.0), 0.0)?;

        // too bad we cannot use the dispatcher for the rendering systems...
        {
            SpriteRenderSystem::new(ctx).run_now(&self.world.res);
        }

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, scancode: Scancode, _keycode: Keycode, _keymod: Mod, _repeat: bool) -> bool {
        if scancode == Scancode::Escape {
            self.quit = true;
        }

        self.world.write_resource::<InputState>().set(scancode);

        false
    }

    fn key_up_event(&mut self, scancode: Scancode, _keycode: Keycode, _keymod: Mod, _repeat: bool) -> bool {
        self.world.write_resource::<InputState>().unset(scancode);
        false
    }
}

fn project(p: [f32; 3]) -> Point2 {
    Point2::new(p[0] / p[2], p[1] / p[2])
}
