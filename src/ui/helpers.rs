use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::stdout;

pub fn init_tui() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>, std::io::Error> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn shutdown_tui() -> Result<(), std::io::Error> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()
}
