use std::time::Duration;

use specs::prelude::*;

use inputstate::InputState;
use resources::Resources;
use three_dee::Cylindric;

pub fn register_components(world: &mut World) {
    world.register::<Acc>();
    world.register::<Controlled>();
    world.register::<Pos>();
    world.register::<RocketLauncher>();
    world.register::<Sprite>();
    world.register::<Vel>();

    world.add_resource(DeltaTime(Duration::from_secs(0)));
    world.add_resource(InputState::new());
    world.add_resource(Resources::new());
}

#[derive(Default)]
pub struct DeltaTime(pub Duration);

#[derive(Debug, Component, Default)]
#[storage(NullStorage)]
pub struct Controlled;

#[derive(Debug, Copy, Clone, Component)]
#[storage(VecStorage)]
pub struct Pos(pub Cylindric);

impl Pos {
    pub fn new(r: f32, w: f32, z: f32) -> Self {
        Pos(Cylindric::new(r, w, z))
    }
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Vel(pub Cylindric);

impl Vel {
    pub fn new(r: f32, w: f32, z: f32) -> Self {
        Vel(Cylindric::new(r, w, z))
    }
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Acc(pub Cylindric);

impl Acc {
    pub fn new(r: f32, w: f32, z: f32) -> Self {
        Acc(Cylindric::new(r, w, z))
    }
}

#[derive(Debug)]
pub enum SpriteSize {
    Auto{scale: f32},
    Fixed{width: f32, height: f32},
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Sprite(pub usize, pub SpriteSize);

impl Sprite {
    pub fn new_auto(id: usize, scale: f32) -> Self {
        Sprite(id, SpriteSize::Auto{scale})
    }

    pub fn new_fixed(id: usize, width: f32, height: f32) -> Self {
        Sprite(id, SpriteSize::Fixed{width, height})
    }
}

#[derive(Debug, Component)]
pub enum RocketLauncher {
    Ready,
    Fire,
    Recharge(Duration),
}