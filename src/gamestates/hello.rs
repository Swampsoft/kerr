
use ggez::{graphics::{self, Point2}, Context, GameResult};

use super::GameState;

pub struct HelloState {
    text: graphics::TextCached,
}

impl HelloState {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        let s = HelloState {
            text: graphics::TextCached::new("Hello!")?,
        };
        Ok(s)
    }
}

impl GameState for HelloState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<bool> {
        Ok(false)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::draw(ctx, &self.text, Point2::new(512.0, 384.0), 0.0)?;
        graphics::present(ctx);
        Ok(())
    }
}
