use state::State;
use ui::{init_tui, poll_events, render_ui, shutdown_tui};

mod api;
mod args;
mod events;
mod geocode;
mod noaa;
mod state;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = init_tui()?;
    let mut state = State::new()?;
    state.find_temperature().await?;

    loop {
        terminal.draw(|mut frame| render_ui(&state, &mut frame))?;
        poll_events(&mut state)?;
        if state.exited {
            break;
        }
    }

    shutdown_tui()?;
    println!("{:?} seconds elapsed", state.elapsed());
    Ok(())
}
