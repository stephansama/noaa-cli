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

fn move_box(area: &mut Rect, (width, height): (u16, u16)) {
    area.x += area.width;
    if area.x >= width {
        area.x = 0;
        area.y += area.height;
    }
    if area.y >= height {
        area.y = 0;
    }
}

fn find_position(num: u16, modulus: Option<u16>) -> u16 {
    if num % modulus.unwrap_or(4) == 0 {
        num
    } else {
        find_position(num - 1, modulus)
    }
}

pub fn render_ui<'a>(state: &'a State) -> Box<dyn Fn(&mut Frame) + 'a> {
    return Box::new(|frame: &mut Frame| {
        let grid_x = 3;
        let grid_y = 4;
        let mut area = frame.size();
        let (max_width, max_height) = (
            find_position(area.width, Some(3)),
            find_position(area.height, None),
        );
        area.width = max_width / grid_x;
        area.height = max_height / grid_y;
        state
            .current_temperature
            .as_ref()
            .unwrap()
            .periods
            .iter()
            .skip(state.skip_by)
            .take((grid_x * grid_y).into())
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
                        Title::from(period.start_time.format("%m-%d HR: %H").to_string())
                            .alignment(Alignment::Center)
                            .position(Position::Bottom),
                    )
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(match period.temperature {
                        temp if temp < 30 => Color::Red,
                        temp if (30..40).contains(&temp) => Color::Yellow,
                        temp if (40..50).contains(&temp) => Color::Blue,
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
                move_box(&mut area, (max_width, max_height));
            });
    });
}
