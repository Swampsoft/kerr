use std::f32::consts::PI;

use ggez::{
    graphics::{DrawParam, Drawable, Point2}, Context,
};

use specs::prelude::*;

use components::{Sprite, Pos};
use resources::Resources;
use three_dee::projection;
use utils::fix_sprite;

pub struct SpriteRenderSystem<'c> {
    ctx: &'c mut Context,
}

impl<'c> SpriteRenderSystem<'c> {
    pub fn new(ctx: &'c mut Context) -> Self {
        SpriteRenderSystem { ctx }
    }
}

impl<'a, 'c> System<'a> for SpriteRenderSystem<'c> {
    type SystemData = (Read<'a, Resources>, ReadStorage<'a, Pos>, ReadStorage<'a, Sprite>);

    fn run(&mut self, (res, pos, spr): Self::SystemData) {
        for (p, s) in (&pos, &spr).join() {
            let a = p.0.w() * 2.0 * PI;
            let screen_pos = projection(p.0);

            res.get_image(s.0)
                .draw_ex(
                    self.ctx,
                    fix_sprite(DrawParam {
                        dest: screen_pos.into(),
                        rotation: -a,
                        offset: Point2::new(0.5, 0.5),
                        scale: Point2::new(0.004, 0.002) * screen_pos.z(),
                        ..Default::default()
                    }),
                )
                .unwrap();
        }
    }
}
