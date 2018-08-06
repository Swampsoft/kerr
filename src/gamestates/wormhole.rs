use std::f32;

use ggez::{
    graphics::{self, DrawMode, DrawParam, Image, Mesh, MeshBuilder, Point2, Rect}, Context, GameResult,
};

use specs::{Builder, Dispatcher, DispatcherBuilder, RunNow, World};

use super::GameState;
use components::{register_components, Pos, PlayerSprite};
use systems::PlayerSpriteSystem;
use utils::fix_sprite;

pub struct WormholeState {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,

    player_sprite: Image,

    z_pos: f32,
}

impl WormholeState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut world = World::new();

        register_components(&mut world);

        world.create_entity()
            .with(Pos{z: 0.0, r: 1.3, w: 0.1})
            .with(PlayerSprite);

        let dispatcher = DispatcherBuilder::new().build();

        let player_sprite = Image::new(ctx, "/originals/faction5/F5S1.png")?;

        let s = WormholeState {
            world,
            dispatcher,
            player_sprite,
            z_pos: 0.0,
        };
        Ok(s)
    }
}

impl GameState for WormholeState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<bool> {
        self.dispatcher.dispatch(&self.world.res);

        self.z_pos -= 0.01;
        if self.z_pos <= 0.0 {
            self.z_pos = 10.0;
        }
        Ok(false)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_screen_coordinates(ctx, Rect::new(-2.0, -1.5, 4.0, 3.0))?;

        let mut mb = MeshBuilder::new();

        for z in 0..30 {
            let z = 0.1 + (z as f32 + self.z_pos % 1.0) / 5.0;

            let z = z + z * z / 2.0;
            //let z = z.sqrt();

            mb.circle(
                DrawMode::Line(0.01 / z),
                Point2::new(0.0, 0.0) / z,
                1.0 / z,
                0.001,
            );
        }

        let z1 = self.z_pos;
        let z2 = self.z_pos + 0.1;

        let z1 = (z1 + z1 * z1 / 2.0) * 0.3;
        let z2 = (z2 + z2 * z2 / 2.0) * 0.3;

        mb.line(
            &[Point2::new(0.7, 0.7) / z1, Point2::new(0.7, 0.7) / z2],
            0.02 / z1,
        );
        mb.line(
            &[Point2::new(0.8, 0.7) / z1, Point2::new(0.8, 0.7) / z2],
            0.02 / z1,
        );

        let mesh = mb.build(ctx)?;

        graphics::draw(ctx, &mesh, Point2::new(0.0, 0.0), 0.0)?;

        // too bad we cannot use the dispatcher for the rendering systems...
        {
            PlayerSpriteSystem::new(ctx, &self.player_sprite)
                .run_now(&self.world.res);
        }

        graphics::present(ctx);
        Ok(())
    }
}

fn project(p: [f32; 3]) -> Point2 {
    Point2::new(p[0] / p[2], p[1] / p[2])
}
