use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    app::{App, Mode},
    components::{list::render_list, logo::Logo},
};

// TODO: Make separate Widget components
pub fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(15),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // TODO: Move this into comonents folder
    let footer_text = match app.mode {
        Mode::Welcome => Line::from(vec![
            Span::styled("Hit ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " to get your Token from Github!",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Mode::Auth => Line::from(vec![Span::styled(
            "Waiting for token",
            Style::default().fg(Color::DarkGray),
        )]),
        Mode::Select => Line::from(vec![Span::styled(
            "Use ↓↑ to move, , Enter to toggle status, g/G to go top/bottom.",
            Style::default().fg(Color::DarkGray),
        )]),
    };

    // TODO: Move this into comonents folder
    let mode_footer = Paragraph::new(footer_text)
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default());

    let logo = Logo::new();

    // TODO: Move this into comonents folder
    let info_text = vec![
        Line::from(String::from(
            "Welcome to knife, a terminal application to delete old deserted GitHub repositories.",
        )),
        Line::from(String::from(
            "After hitting Enter, your default browser will open and redirect you to the personal access token webpage on Github.",
        )),
    ];

    // TODO: Move this into comonents folder
    let welcome_text = Paragraph::new(Text::from(info_text))
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::new());

    frame.render_widget(logo, chunks[0]);
    frame.render_widget(welcome_text, chunks[1]);
    frame.render_widget(mode_footer, chunks[2]);

    match app.mode {
        Mode::Welcome => {}
        Mode::Auth => {
            if app.waiting_for_token {
                draw_token_input(frame, &app.token_input);
            }
        }
        Mode::Select => {
            if !app.waiting_for_repos {
                render_list(&mut app.repo_list, chunks[1], frame.buffer_mut());
            }
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

fn draw_token_input(frame: &mut Frame, input: &str) {
    // TODO: Show a cursor and let a user delete an entry and move within the input
    let input_area = centered_rect(60, 10, frame.area());

    let key_block = Block::default()
        .title(" Please paste your token here ")
        .borders(Borders::ALL);

    let token_text = Paragraph::new(input).block(key_block).clone();
    frame.render_widget(token_text, input_area);
}
