use crate::state::State;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub fn handle_event(state: &mut State, key: KeyEvent) {
    match (key.kind, key.code) {
        (KeyEventKind::Press, KeyCode::Char('h')) => {
            if state.skip_by > 0 {
                state.skip_by -= 16
            }
        }
        (KeyEventKind::Press, KeyCode::Char('l')) => state.skip_by += 16,
        (KeyEventKind::Press, KeyCode::Char('q')) => state.exited = true,
        (_, _) => (),
    }
}
