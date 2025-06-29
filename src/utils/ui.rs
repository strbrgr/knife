use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    app::LIGHT_RED,
    components::list::{RepoItem, Status},
};

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn draw_token_input(
    frame: &mut Frame,
    input: &str,
    character_index: u16,
    token_limit_reached: bool,
) {
    let input_area = centered_rect(60, 10, frame.area());

    let (title, style) = if token_limit_reached {
        (
            " Token length limit reached ",
            Style::default().fg(LIGHT_RED),
        )
    } else {
        (" Please paste your token here ", Style::default())
    };

    let key_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(style);

    let token_text = Paragraph::new(input).style(style).block(key_block);
    frame.render_widget(token_text, input_area);
    frame.set_cursor_position(Position::new(
        // Draw the cursor at the current position in the input field.
        // This position can be controlled via the left and right arrow key
        input_area.x + character_index + 1,
        // Move one line down, from the border to the input line
        input_area.y + 1,
    ));
}

pub fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

pub fn render_popup_content(frame: &mut Frame, repos: &Vec<RepoItem>) {
    let mut lines = vec![];
    for r in repos {
        if r.status == Status::Selected {
            lines.push(Line::from(r.name.clone()))
        }
    }
    let text = Text::from(lines);

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick),
        )
        .style(Style::default().fg(Color::Red));

    let area = popup_area(frame.area(), 60, 20);
    frame.render_widget(paragraph, area);
}
