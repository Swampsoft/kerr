
use ggez::{graphics::{self, DrawMode, Point2, Mesh, MeshBuilder, Rect}, Context, GameResult};

use super::GameState;

pub struct WormholeState {
    text: graphics::TextCached,

    z_pos: f32,
}

impl WormholeState {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        let s = WormholeState {
            text: graphics::TextCached::new("Hello!")?,
            z_pos: 0.0,
        };
        Ok(s)
    }
}

impl GameState for WormholeState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<bool> {
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

            mb.circle(DrawMode::Line(0.01 / z), Point2::new(0.0, 0.0) / z, 1.0 / z, 0.001);
        }

        let z1 = self.z_pos;
        let z2 = self.z_pos + 0.1;

        let z1 = (z1 + z1 * z1 / 2.0) * 0.3;
        let z2 = (z2 + z2 * z2 / 2.0) * 0.3;

        mb.line(&[Point2::new(0.7, 0.7) / z1, Point2::new(0.7, 0.7) / z2], 0.02 / z1);
        mb.line(&[Point2::new(0.8, 0.7) / z1, Point2::new(0.8, 0.7) / z2], 0.02 / z1);

        let mesh = mb.build(ctx)?;

        graphics::draw(ctx, &mesh, Point2::new(0.0, 0.0), 0.0)?;

        graphics::present(ctx);
        Ok(())
    }
}

fn project(p: [f32; 3]) -> Point2 {
    Point2::new(
        p[0] / p[2],
        p[1] / p[2],
    )

}
