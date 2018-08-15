use specs::prelude::*;

use inputstate::InputState;
use resources::Resources;
use three_dee::Cylindric;

pub fn register_components(world: &mut World) {
    world.register::<Controlled>();
    world.register::<Pos>();
    world.register::<Sprite>();
    world.register::<Vel>();

    world.add_resource(DeltaTime(0.0));
    world.add_resource(InputState::new());
    world.add_resource(Resources::new());
}

pub struct DeltaTime(pub f64);

#[derive(Debug, Component, Default)]
#[storage(NullStorage)]
pub struct Controlled;

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Pos(pub Cylindric);

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Vel(pub Cylindric);

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Sprite(pub usize);

impl Pos {
    pub fn new(r: f32, w: f32, z: f32) -> Self {
        Pos(Cylindric::new(r, w, z))
    }
}

impl Vel {
    pub fn new(r: f32, w: f32, z: f32) -> Self {
        Vel(Cylindric::new(r, w, z))
    }
}
