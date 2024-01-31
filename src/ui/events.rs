use crate::{events::handle_event, state::State};
use crossterm::event::{poll, read, Event, KeyEventKind};

pub fn poll_events(state: &mut State) -> Result<(), std::io::Error> {
    if poll(std::time::Duration::from_millis(16))? {
        if let Event::Key(key) = read()? {
            if key.kind == KeyEventKind::Press {
                handle_event(state, key)
            }
        }
    }
    Ok(())
}
