use std::f32::consts::PI;

use ggez::graphics::Point2;

pub fn projection(pos: Cylindric) -> Cartesian {
    let a = pos.w * 2.0 * PI;
    let f = projection_factor(pos.z);
    let r = pos.r * f;
    Cartesian::new(a.sin() * r, a.cos() * r, f)
}

pub fn projection_factor(z: f32) -> f32 {
    //10.0 / (9.0 + z + 0.1 * (z.abs() * z - 1.0))
    //10.0 / (9.0 + z.abs() * z / 5.0)
    10.0 / (z + 1.0 * (z.abs() * z))
}

pub fn cylindric_pos_to_cartesian(pos: Cylindric) -> Cartesian {
    let a = pos.w * 2.0 * PI;
    Cartesian::new(a.sin() * pos.r, -a.cos() * pos.r, pos.z)
}

pub fn cylindric_vel_to_cartesian(vel: Cylindric, pos: Cylindric) -> Cartesian {
    let a = pos.w * 2.0 * PI;
    let sin = a.sin();
    let cos = a.cos();
    Cartesian::new(vel.w * 2.0 * PI * pos.r * cos + vel.r * sin, vel.w * 2.0 * PI * pos.r * sin - vel.r * cos, vel.z)
}

#[derive(Debug, Copy, Clone)]
pub struct Cartesian {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Cartesian {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Cartesian { x, y, z }
    }
}

impl From<Cartesian> for Point2 {
    fn from(xyz: Cartesian) -> Self {
        Point2::new(xyz.x, xyz.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cylindric {
    pub r: f32,
    pub w: f32,
    pub z: f32,
}

impl Cylindric {
    pub fn new(r: f32, w: f32, z: f32) -> Self {
        Cylindric { r, w, z }
    }
}

impl From<Cartesian> for [f32; 3] {
    fn from(vec: Cartesian) -> Self {
        [vec.x, vec.y, vec.z]
    }
}