extern crate ggez;

mod gamestates;

use ggez::{conf, GameResult};

use gamestates::{wormhole::WormholeState, StateManager};

fn main() -> GameResult<()> {
    let c = conf::Conf {
        window_mode: conf::WindowMode::default().dimensions(1024, 768),
        window_setup: conf::WindowSetup::default().title("Rock Project"),
        backend: conf::Backend::OpenGL { major: 3, minor: 2 },
    };

    let ctx = &mut ggez::Context::load_from_conf("Kerr", "Swampsoft Games", c).unwrap();

    let main_state = &mut StateManager::new(WormholeState::new(ctx)?);

    ggez::event::run(ctx, main_state)
}
