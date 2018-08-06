use ggez::Context;

use specs::prelude::*;

pub fn register_components(world: &mut World) {
    world.register::<PlayerSprite>();
    world.register::<Pos>();
    world.register::<Vel>();

    world.add_resource(DeltaTime(0.0));
}

pub struct DeltaTime(pub f64);

#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
pub struct PlayerSprite;

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Pos {
    pub z: f32,
    pub r: f32,
    pub w: f32,
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Vel {
    pub z: f32,
    pub r: f32,
    pub w: f32,
}
