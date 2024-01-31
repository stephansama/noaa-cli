use crate::state::State;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Paragraph,
    },
    Frame,
};

fn move_box(area: &mut Rect, size: Rect) {
    area.x += area.width;
    if area.x >= size.width {
        area.x = 0;
        area.y += area.height;
    }
    if area.y >= size.height {
        area.y = 0;
    }
}

fn find_position(num: u16) -> u16 {
    if num % 4 == 0 {
        num
    } else {
        find_position(num - 1)
    }
}

pub fn render_ui(state: &State, frame: &mut Frame) {
    let mut area = frame.size();
    let (width, height) = (find_position(area.width), find_position(area.height));
    area.width = width / 4;
    area.height = height / 4;
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
                        period.start_time.format("%m-%d Hour: %H").to_string()
                    ))
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
                )
                .borders(Borders::ALL)
                .border_style(Style::default().fg(match period.temperature {
                    temp if temp < 30 => Color::Red,
                    temp if temp >= 30 && temp < 40 => Color::Yellow,
                    temp if temp >= 40 && temp < 50 => Color::Blue,
                    temp if temp >= 50 => Color::Green,
                    _ => Color::White,
                }))
                .border_type(BorderType::Rounded);
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
            move_box(
                &mut area,
                Rect {
                    x: 0,
                    y: 0,
                    width,
                    height,
                },
            );
        });
}
