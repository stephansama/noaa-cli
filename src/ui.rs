use crate::{events::handle_event, state::State};
use crossterm::{
    event::{poll, read, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, List, Paragraph, Widget,
    },
    Frame, Terminal,
};
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

fn move_box(area: &mut Rect, size: Rect) {
    area.x += area.width;
    let (max_x, max_y) = (size.width, size.height);
    if area.x >= max_x {
        area.x = 0;
        area.y += area.height;
    }
    if area.y >= max_y {
        area.y = 0;
    }
}

pub fn render_ui(state: &State, frame: &mut Frame) {
    let mut area = frame.size();
    area.width = area.width / 4;
    area.height = area.height / 4;
    state
        .current_temperature
        .as_ref()
        .unwrap()
        .periods
        .iter()
        .skip(state.skip_by.into())
        .take(16)
        .for_each(|period| {
            let title_block = Block::default()
                .title(
                    Title::from(format!(
                        "{}°{}",
                        period.temperature, period.temperature_unit
                    ))
                    .alignment(Alignment::Center),
                )
                .title(
                    Title::from(format!(
                        "{}",
                        period.start_time.format("%Y-%m-%d Hour: %H").to_string()
                    ))
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
                )
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
                .white();
            let paragraph = Paragraph::new(vec![
                Line::from(vec![Span::raw(format!(
                    "forecast: {:?}",
                    period.short_forecast
                ))]),
                Line::from(vec![Span::raw(format!(
                    "humidity: {:?}%",
                    period.relative_humidity.value
                ))]),
                Line::from(vec![Span::raw(format!(
                    "precipitation: {:?}%",
                    period.probability_of_precipitation.value
                ))]),
                // Line::from(vec![Span::raw(format!("skip_by: {:?}", state.skip_by))]),
                Line::from(vec![Span::raw(format!(
                    "wind direction: {}",
                    period.wind_direction
                ))]),
                Line::from(vec![Span::raw(format!(
                    "wind speed: {}",
                    period.wind_speed
                ))]),
            ])
            .clone()
            .block(title_block.clone());
            frame.render_widget(title_block, area);
            frame.render_widget(paragraph, area);
            move_box(&mut area, frame.size());
            // Line::from(vec![Span::styled(
            //     Style::new().green().italic(),
            // )]),
        });
}

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
