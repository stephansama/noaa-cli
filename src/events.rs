use crate::state::State;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_event(state: &mut State, key: KeyEvent) {
    let skip_by_amount = 12;
    match key.code {
        KeyCode::Char('h') | KeyCode::Char('k') => {
            if state.skip_by > 0 {
                state.skip_by -= skip_by_amount
            }
        }
        KeyCode::Char('l') | KeyCode::Char('j') => {
            if state.skip_by
                < state.current_temperature.as_ref().unwrap().periods.len() - skip_by_amount
            {
                state.skip_by += skip_by_amount
            }
        }
        KeyCode::Char('g') => state.skip_by = 0,
        KeyCode::Char('G') => {
            state.skip_by =
                state.current_temperature.as_ref().unwrap().periods.len() - skip_by_amount
        }
        KeyCode::Char('q') => state.exited = true,
        _ => (),
    }
}
