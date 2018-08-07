pub mod hello;
pub mod wormhole;

use ggez::{event::EventHandler, Context, GameResult};

pub trait GameState {
    /// update. return `Ok(true)` if the state below should be updated too.
    fn update(&mut self, ctx: &mut Context) -> GameResult<bool>;
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;

    /// override if this state is not completete opaque and the previous state should be drown too
    fn draw_previous(&self) -> bool {
        false
    }
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

impl EventHandler for StateManager {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.states.is_empty() {
            return ctx.quit();
        }

        for state in self.states.iter_mut().rev() {
            if !state.update(ctx)? {
                break;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        StateManager::draw_recursive(&mut self.states, ctx)
    }
}
