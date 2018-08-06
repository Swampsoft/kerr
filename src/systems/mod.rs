use std::f32::consts::PI;

use ggez::{
    Context,
    graphics::{
        Drawable,
        DrawParam,
        Image,
        Point2,
    },
};

use specs::prelude::*;

use components::{Pos, PlayerSprite};
use utils::fix_sprite;

pub struct PlayerSpriteSystem<'c> {
    ctx: &'c mut Context,
    img: &'c Image,
}

impl<'c> PlayerSpriteSystem<'c> {
    pub fn new(ctx: &'c mut Context, img: &'c Image) -> Self {
        PlayerSpriteSystem {
            ctx,
            img,
        }
    }
}

impl<'a, 'c> System<'a> for PlayerSpriteSystem<'c> {
    type SystemData = (ReadStorage<'a, Pos>,
                       ReadStorage<'a, PlayerSprite>);

    fn run(&mut self, (pos, spr): Self::SystemData) {
        for (p, s) in (&pos, &spr).join() {
            let a = p.w * 2.0 * PI;
            self.img.draw_ex(self.ctx, fix_sprite(DrawParam {
                dest: Point2::new(a.sin() * p.r, a.cos() * p.r),
                rotation: -a,
                offset: Point2::new(0.5, 0.5),
                scale: Point2::new(0.004, 0.002),
                ..Default::default()
            })).unwrap();
        }

    }
}