
use specs::prelude::*;

use resources::Resources;

pub fn register_components(world: &mut World) {
    world.register::<Pos>();
    world.register::<Sprite>();
    world.register::<Vel>();

    world.add_resource(DeltaTime(0.0));
    world.add_resource(Resources::new());
}

pub struct DeltaTime(pub f64);

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

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Sprite(pub usize);
