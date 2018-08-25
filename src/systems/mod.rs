use std::f32::consts::PI;
use std::time::Duration;

use ggez::{
    graphics::{self, DrawParam, Drawable, Point2}, Context,
    timer::duration_to_f64,
};

use specs::prelude::*;

use components::{Acc, Controlled, DeltaTime, Pos, Vel, RocketLauncher, Sprite, SpriteSize};
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

            let img = res.get_image(s.0);

            let scale = match s.1 {
                SpriteSize::Auto{scale} => Point2::new(0.002, 0.002) * scale,
                SpriteSize::Fixed{width, height} => Point2::new(width * 0.5 / img.width() as f32, height * 0.5 / img.height() as f32),
            } * screen_pos.z;

            img.draw_ex(
                self.ctx,
                fix_sprite(DrawParam {
                    dest: screen_pos.into(),
                    rotation: -a,
                    offset: Point2::new(0.5, 0.5),
                    scale,
                    ..Default::default()
                }),
            )
            .unwrap();
        }
    }
}
/*
pub struct RectangleRenderSystem<'c> {
    ctx: &'c mut Context,
}

impl<'c> RectangleRenderSystem<'c> {
    pub fn new(ctx: &'c mut Context) -> Self {
        RectangleRenderSystem { ctx }
    }
}

impl<'a, 'c> System<'a> for RectangleRenderSystem<'c> {
    type SystemData = (Read<'a, Resources>, ReadStorage<'a, Pos>);

    fn run(&mut self, (res, pos): Self::SystemData) {
        for p in pos.join() {
            let a = p.0.w * 2.0 * PI;
            let screen_pos = projection(p.0);

            graphics::rectangle()

            graphics::draw_ex(self.ctx, &graphics::Rect::new(-1.0, -1.0, 1.0, 1.0), fix_sprite(DrawParam {
                dest: screen_pos.into(),
                rotation: -a,
                offset: Point2::new(0.5, 0.5),
                scale: Point2::new(0.002, 0.002) * screen_pos.z,
                ..Default::default()
            }));
        }
    }
}
*/
pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (Read<'a, InputState>, ReadStorage<'a, Controlled>, WriteStorage<'a, RocketLauncher>, WriteStorage<'a, Pos>, Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (inp, ctr, mut launcher, mut pos, ent, updater): Self::SystemData) {
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

        for (c, l) in (&ctr, &mut launcher).join() {
            if inp.is_set(Input::Fire) {
                match l {
                    RocketLauncher::Ready => *l = RocketLauncher::Fire,
                    RocketLauncher::Recharge(_) => {},
                    RocketLauncher::Fire => {}
                }
            }
        }
    }
}

pub struct RocketLauncherSystem;

impl<'a> System<'a> for RocketLauncherSystem {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, RocketLauncher>, ReadStorage<'a, Pos>, Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (dt, mut launcher, pos, ent, updater): Self::SystemData) {
        for (l, p) in (&mut launcher, &pos).join() {
            *l = match l {
                RocketLauncher::Ready => RocketLauncher::Ready,
                RocketLauncher::Recharge(d) => {
                    if *d == Duration::from_secs(0) {
                        RocketLauncher::Ready
                    } else if *d < dt.0 {
                        RocketLauncher::Recharge(Duration::from_secs(0))
                    } else {
                        RocketLauncher::Recharge(*d - dt.0)
                    }
                }
                RocketLauncher::Fire => {
                    let e = ent.create();

                    updater.insert(e, *p);
                    updater.insert(e, Vel::new(0.0, 0.0, 0.0));
                    updater.insert(e, Acc::new(0.0, 0.0, 0.5));
                    updater.insert(e, Sprite::new_auto(3, 0.5));

                    RocketLauncher::Recharge(Duration::from_millis(500))
                }
            };
        }
    }
}

pub struct KinematicSystem;

impl<'a> System<'a> for KinematicSystem {
    type SystemData = (Read<'a, DeltaTime>, ReadStorage<'a, Acc>, WriteStorage<'a, Vel>, WriteStorage<'a, Pos>);

    fn run(&mut self, (dt, acc, mut vel, mut pos): Self::SystemData) {
        let dt = duration_to_f64(dt.0) as f32;

        for (a, v) in (&acc, &mut vel).join() {
            v.0.r += a.0.r * dt;
            v.0.w += a.0.w * dt;
            v.0.z += a.0.z * dt;
        }

        for (v, p) in (&vel, &mut pos).join() {
            p.0.r += v.0.r * dt;
            p.0.w += v.0.w * dt;
            p.0.z += v.0.z * dt;

            p.0.w = p.0.w % 1.0;
            if p.0.w < 0.0 {
                p.0.w = 1.0 + p.0.w;
            }
        }
    }
}
