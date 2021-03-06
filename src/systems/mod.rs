use std::f32::consts::PI;
use std::time::Duration;

use ambisonic::sources::Noise;

use ggez::{
    graphics::{self, DrawParam, Drawable, Point2}, Context,
    timer::duration_to_f64,
};

use rodio::Source;

use specs::prelude::*;

use audio::Audio;
use components::{Acc, Controlled, DeltaTime, Pos, Vel, RocketLauncher, RocketProjectile, SoundEmitter, Sprite, SpriteSize};
use inputstate::{Input, InputState};
use resources::Resources;
use three_dee::{cylindric_pos_to_cartesian, cylindric_vel_to_cartesian, projection};
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
            if p.0.z < 0.0 {
                continue
            }

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
    type SystemData = (Read<'a, DeltaTime>, Read<'a, InputState>, ReadStorage<'a, Controlled>, WriteStorage<'a, RocketLauncher>, WriteStorage<'a, Pos>, WriteStorage<'a, Vel>);

    fn run(&mut self, (dt, inp, ctr, mut launcher, mut pos, mut vel): Self::SystemData) {
        let dt = duration_to_f64(dt.0) as f32;

        for (c, mut p, mut v) in (&ctr, &mut pos, &mut vel).join() {
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
                _ => {
                    v.0.w = 0.0;
                    continue
                },
            };

            let mut distance = target - p.0.w;

            if distance.abs() >= 0.5 {
                distance = -distance;
            }

            let mut direction = distance.min(0.01).max(-0.01);

            //p.0.w += direction;
            v.0.w = direction / dt;
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
    type SystemData = (Read<'a, DeltaTime>, Read<'a, Audio>, WriteStorage<'a, RocketLauncher>, ReadStorage<'a, Pos>, Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (dt, audio, mut launcher, pos, ents, updater): Self::SystemData) {
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

                    let se = SoundEmitter::new(&audio.ambisonic);
                    se.mixer_controller.add(Noise::new(48000).amplify(0.1));

                    let e = updater.create_entity(&ents)
                        .with(*p)
                        .with(Vel::new(0.0, 0.0, 0.5))
                        .with(Acc::new(0.0, 0.0, 0.0))
                        .with(Sprite::new_auto(3, 0.5))
                        .with(RocketProjectile::Launching(25.0, Duration::from_millis(200)))
                        .with(se)
                        .build();

                    RocketLauncher::Recharge(Duration::from_millis(350))
                }
            };
        }
    }
}

pub struct RocketProjectileSystem;

impl<'a> System<'a> for RocketProjectileSystem {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, RocketProjectile>, WriteStorage<'a, Acc>, WriteStorage<'a, SoundEmitter>, Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (dt, mut rockets, mut accs, mut sounds, ents, updater): Self::SystemData) {
        for (rocket, mut acc, mut se, ent) in (&mut rockets, &mut accs, &mut sounds, &*ents).join() {
            *rocket = match *rocket {
                RocketProjectile::Launching(a, mut d) => {
                    if d > dt.0 {
                        RocketProjectile::Launching(a, d - dt.0)
                    } else {
                        acc.0.z = a;
                        se.mixer_controller.add(Noise::new(48000));
                        RocketProjectile::Accelerating(Duration::from_millis(200))
                    }
                }
                RocketProjectile::Accelerating(mut d) => {
                    if d > dt.0 {
                        RocketProjectile::Accelerating(d - dt.0)
                    } else {
                        acc.0.z = 0.0;
                        RocketProjectile::Flying(Duration::from_millis(600))
                    }
                }
                RocketProjectile::Flying(mut d) => {
                    if d > dt.0 {
                        RocketProjectile::Flying(d - dt.0)
                    } else {
                        se.spatial_controller.stop();
                        ents.delete(ent);
                        RocketProjectile::Flying(d)
                    }
                }
            }
        }
    }
}

pub struct KinematicSystem;

impl<'a> System<'a> for KinematicSystem {
    type SystemData = (Read<'a, DeltaTime>, ReadStorage<'a, Acc>, WriteStorage<'a, Vel>, WriteStorage<'a, Pos>);

    fn run(&mut self, (dt, acc, mut vel, mut pos): Self::SystemData) {
        let dt = duration_to_f64(dt.0) as f32;

        for (a, mut v) in (&acc, &mut vel).join() {
            v.0.r += a.0.r * dt;
            v.0.w += a.0.w * dt;
            v.0.z += a.0.z * dt;
        }

        for (v, mut p) in (&vel, &mut pos).join() {
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

pub struct SpatialAudioSystem;


impl<'a> System<'a> for SpatialAudioSystem {
    type SystemData = (ReadStorage<'a, Vel>, ReadStorage<'a, Pos>, WriteStorage<'a, SoundEmitter>);

    fn run(&mut self, (vel, pos, mut emitter): Self::SystemData) {
        for (p, e) in (&pos, &mut emitter).join() {
            let tmp = cylindric_pos_to_cartesian(p.0);
            // in Ambisonic z points up, but our z points into the screen
            e.spatial_controller.adjust_position([tmp.x, tmp.z, tmp.y]);
        }

        for (v, p, e) in (&vel, &pos, &mut emitter).join() {
            let tmp = cylindric_vel_to_cartesian(v.0, p.0);
            // in Ambisonic z points up, but our z points into the screen
            e.spatial_controller.set_velocity([tmp.x, tmp.z, tmp.y])
        }
    }
}