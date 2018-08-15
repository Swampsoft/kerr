use std::f32::consts::PI;

use ggez::{
    graphics::{DrawParam, Drawable, Point2}, Context,
};

use specs::prelude::*;

use components::{Controlled, Sprite, Pos};
use inputstate::{Input, InputState};
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
            let a = p.0.w * 2.0 * PI;
            let screen_pos = projection(p.0);

            res.get_image(s.0)
                .draw_ex(
                    self.ctx,
                    fix_sprite(DrawParam {
                        dest: screen_pos.into(),
                        rotation: -a,
                        offset: Point2::new(0.5, 0.5),
                        scale: Point2::new(0.004, 0.002) * screen_pos.z,
                        ..Default::default()
                    }),
                )
                .unwrap();
        }
    }
}

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (Read<'a, InputState>, ReadStorage<'a, Controlled>, WriteStorage<'a, Pos>);

    fn run(&mut self, (inp, ctr, mut pos): Self::SystemData) {
        for (c, p) in (&ctr, &mut pos).join() {
            p.0.w = p.0.w % 1.0;
            if p.0.w < 0.0 {
                p.0.w = 1.0 + p.0.w;
            }

            let mut delta = 0;

            let target = match (inp.is_set(Input::Left), inp.is_set(Input::Right), inp.is_set(Input::Up), inp.is_set(Input::Down)) {
                (true, false, false, false) => 0.75,
                (false, true, false, false) => 0.25,
                (false, false, true, false) => 0.5,
                (false, false, false, true) => 0.0,
                (true, false, true, false) => 0.625,
                (true, false, false, true) => 0.875,
                (false, true, true, false) => 0.375,
                (false, true, false, true) => 0.125,
                _ => continue,
            };

            let mut distance = target - p.0.w;

            if distance.abs() >= 0.5 {
                distance = -distance;
            }

            let mut direction = distance.min(0.01).max(-0.01);

            p.0.w += direction;
        }
    }
}
