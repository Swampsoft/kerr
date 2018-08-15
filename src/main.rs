use std::time::Duration;

extern crate ggez;
extern crate sdl2;
extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate steamy_controller;

mod components;
mod gamestates;
mod inputstate;
mod resources;
mod systems;
mod three_dee;
mod utils;

use ggez::{
    conf, event::{Event, EventHandler, Events}, GameResult,
};

use steamy_controller::Manager;

use gamestates::{wormhole::WormholeState, StateManager};

fn main() -> GameResult<()> {
    let c = conf::Conf {
        window_mode: conf::WindowMode::default().dimensions(1024, 768),
        window_setup: conf::WindowSetup::default().title("Rock Project"),
        backend: conf::Backend::OpenGL { major: 3, minor: 2 },
    };

    let ctx = &mut ggez::Context::load_from_conf("Kerr", "Swampsoft Games", c).unwrap();

    let states = &mut StateManager::new(WormholeState::new(ctx)?);

    /*let mut scm = Manager::new().unwrap();
    let mut ctr = scm.open().unwrap();
    let details = ctr.details();
    println!("{:#?}", details);
    if ctr.is_remote() { println!("remote"); }
    if ctr.is_wired() { println!("wired"); }
    if ctr.is_connected() { println!("connected"); }

    for _ in 0..1000 {
        let state = ctr.state(Duration::from_millis(10));
        println!("{:?}", state);
    }*/

    //ggez::event::run(ctx, states)

    let mut events = Events::new(ctx)?;

    let mut maxcode: u32 = 0;

    let mut continuing = true;
    while continuing {
        ctx.timer_context.tick();
        for event in events.poll() {
            ctx.process_event(&event);
            match event {
                Event::Quit { .. } => {
                    continuing = states.quit_event(ctx);
                }

                Event::KeyDown {
                    scancode,
                    keycode,
                    keymod,
                    repeat,
                    ..
                } => states.key_down_event(ctx, scancode.unwrap(), keycode.unwrap(), keymod, repeat),

                Event::KeyUp {
                    scancode,
                    keycode,
                    keymod,
                    repeat,
                    ..
                } => states.key_up_event(ctx, scancode.unwrap(), keycode.unwrap(), keymod, repeat),

                e => {} //println!("Event fired: {:?}", e),
            }
        }
        states.update(ctx)?;
        states.draw(ctx)?;
    }

    Ok(())
}
