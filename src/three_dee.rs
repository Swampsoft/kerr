use std::f32::consts::PI;

use ggez::graphics::Point2;

pub fn projection(pos: Cylindric) -> Cartesian {
    let a = pos.w() * 2.0 * PI;
    let f = projection_factor(pos.z());
    let r = pos.r() * f;
    Cartesian::new(a.sin() * r, a.cos() * r, f)
}

pub fn projection_factor(z: f32) -> f32 {
    //10.0 / (9.0 + z + 0.1 * (z.abs() * z - 1.0))
    //10.0 / (9.0 + z.abs() * z / 5.0)
    10.0 / (z + 1.0 * (z.abs() * z))
}

#[derive(Debug, Copy, Clone)]
pub struct Cartesian {
    x: f32,
    y: f32,
    z: f32,
}

impl Cartesian {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Cartesian { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}

impl From<Cartesian> for Point2 {
    fn from(xyz: Cartesian) -> Self {
        Point2::new(xyz.x, xyz.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cylindric {
    r: f32,
    w: f32,
    z: f32,
}

impl Cylindric {
    pub fn new(r: f32, w: f32, z: f32) -> Self {
        Cylindric { r, w, z }
    }

    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn w(&self) -> f32 {
        self.w
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}
