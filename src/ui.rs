use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Mode};

pub fn ui(frame: &mut Frame, app: &App) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(15),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Change this depending on Mode
    let footer_text = match app.mode {
        Mode::Welcome => Span::styled(
            "Hit enter to get your token",
            Style::default().fg(Color::DarkGray),
        ),
        Mode::Auth => Span::styled("Waiting for token", Style::default().fg(Color::DarkGray)),
        Mode::Select => Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray)),
    };

    let mode_footer = Paragraph::new(Line::from(footer_text))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default());

    let ascii_art = r#"
██ ▄█▀ ███▄    █  ██▓  █████▒▓█████ 
██▄█▒  ██ ▀█   █ ▓██▒▓██   ▒ ▓█   ▀ 
▓███▄░ ▓██  ▀█ ██▒▒██▒▒████ ░ ▒███   
▓██ █▄ ▓██▒  ▐▌██▒░██░░▓█▒  ░ ▒▓█  ▄ 
▒██▒ █▄▒██░   ▓██░░██░░▒█░    ░▒████▒
▒ ▒▒ ▓▒░ ▒░   ▒ ▒ ░▓   ▒ ░    ░░ ▒░ ░
░ ░▒ ▒░░ ░░   ░ ▒░ ▒ ░ ░       ░ ░  ░
░ ░░ ░    ░   ░ ░  ▒ ░ ░ ░       ░   
░  ░            ░  ░             ░  ░
"#;

    let logo = Paragraph::new(ascii_art)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::new());

    let info_text = vec![
        Line::from(String::from(
            "Welcome to knife, a terminal application to delete old deserted GitHub repositories.",
        )),
        Line::from(String::from(
            "After hitting enter your default browser will open and redirect you to the personal access token webpage on Github.",
        )),
    ];

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
        Mode::Select => {}
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
    let input_area = centered_rect(60, 10, frame.area());

    let key_block = Block::default()
        .title(" Please paste your token here ")
        .borders(Borders::ALL);

    let token_text = Paragraph::new(input).block(key_block).clone();
    frame.render_widget(token_text, input_area);
}
