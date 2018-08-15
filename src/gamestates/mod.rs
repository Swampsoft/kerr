pub mod hello;
pub mod wormhole;

use ggez::{
    event::{EventHandler, Keycode, Mod}, Context, GameResult,
};

use sdl2::keyboard::Scancode;

pub trait GameState {
    fn transition(&self) -> StateTransition {
        StateTransition::None
    }

    /// update. return `Ok(true)` if the state below should be updated too.
    fn update(&mut self, ctx: &mut Context) -> GameResult<bool>;
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;

    /// override if this state is not completely opaque and the previous state should be drawn too
    fn draw_previous(&self) -> bool {
        false
    }

    /// return `true` if event should be passed down to previous state
    fn key_down_event(
        &mut self,
        scancode: Scancode,
        keycode: Keycode,
        keymod: Mod,
        repeat: bool,
    ) -> bool {
        false
    }

    /// return `true` if event should be passed down to previous state
    fn key_up_event(
        &mut self,
        scancode: Scancode,
        keycode: Keycode,
        keymod: Mod,
        repeat: bool,
    ) -> bool {
        false
    }
}

pub enum StateTransition {
    None,
    Pop,
    PopN(u8),
    Push(Box<GameState>),
}

pub struct StateManager {
    states: Vec<Box<GameState>>,
}

impl StateManager {
    pub fn new<T: GameState + 'static>(initial_state: T) -> Self {
        StateManager {
            states: vec![Box::new(initial_state)],
        }
    }

    pub fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        false
    }

    pub fn key_down_event(
        &mut self,
        ctx: &mut Context,
        scancode: Scancode,
        keycode: Keycode,
        keymod: Mod,
        repeat: bool,
    ) {
        for state in self.states.iter_mut().rev() {
            if !state.key_down_event(scancode, keycode, keymod, repeat) {
                break;
            }
        }
    }

    pub fn key_up_event(
        &mut self,
        ctx: &mut Context,
        scancode: Scancode,
        keycode: Keycode,
        keymod: Mod,
        repeat: bool,
    ) {
        for state in self.states.iter_mut().rev() {
            if !state.key_up_event(scancode, keycode, keymod, repeat) {
                break;
            }
        }
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.states.is_empty() {
            return ctx.quit();
        }

        for state in self.states.iter_mut().rev() {
            if !state.update(ctx)? {
                break;
            }
        }

        match self.states.last().unwrap().transition() {
            StateTransition::None => {}
            StateTransition::Pop => {
                self.states.pop();
            }
            StateTransition::PopN(n) => {
                let r = self.states.len().saturating_sub(n as usize);
                self.states.truncate(r);
            },
            StateTransition::Push(bs) => self.states.push(bs),
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        StateManager::draw_recursive(&mut self.states, ctx)
    }

    fn draw_recursive(states: &mut [Box<GameState>], ctx: &mut Context) -> GameResult<()> {
        if let Some((last, prev_states)) = states.split_last_mut() {
            if last.draw_previous() {
                StateManager::draw_recursive(prev_states, ctx)?;
            }
            last.draw(ctx)?;
        }
        Ok(())
    }
}
