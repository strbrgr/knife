use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::{
    app::{App, Mode},
    components::{
        list::{RepoItem, Status, render_list},
        logo::Logo,
    },
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Percentage(90),
            Constraint::Fill(1),
        ])
        .split(frame.area());

    // Center vertically and create the main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),    // 0: Top spacing
            Constraint::Length(10), // 1: Logo area
            Constraint::Length(1),  // 2: Padding between logo and welcome text
            Constraint::Length(3),  // 3: Welcome text, fixed height
            Constraint::Length(2),  // 4: Padding between welcome text and footer
            Constraint::Length(1),  // 5: Footer
            Constraint::Fill(1),    // 6: Bottom spacing
        ])
        .split(horizontal_chunks[1]);

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
        Mode::Select => Line::from(vec![Span::styled(
            "Use ↓↑ or 'j' and 'k' to move, Spacebar to toggle status, and Enter to confirm",
            Style::default().fg(Color::DarkGray),
        )]),
        Mode::Confirm => Line::from(vec![Span::styled(
            "Press enter to delete the selected repos",
            Style::default().fg(Color::DarkGray),
        )]),
        _ => Line::from("Unknown mode"),
    };

    let footer = Paragraph::new(footer_text)
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default());

    let logo = Logo::adaptive(chunks[1]);

    let info_text = vec![
        Line::from(String::from(
            "Welcome to knife, a terminal application to delete GitHub repositories.",
        )),
        Line::from(String::from(
            "After hitting Enter, your default browser will open and redirect you to the personal access token (PAT) page on Github.",
        )),
        Line::from(String::from(
            "Please use the pre-selected settings and copy the PAT.",
        )),
    ];

    let welcome_text = Paragraph::new(Text::from(info_text))
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default())
        .block(Block::new());

    if app.mode == Mode::Welcome {
        frame.render_widget(logo, chunks[1]);
        frame.render_widget(welcome_text, chunks[3]);
    }
    if app.mode != Mode::Auth {
        frame.render_widget(footer, chunks[5]);
    }

    match app.mode {
        Mode::Welcome => {}
        Mode::Auth => {
            if app.waiting_for_token {
                draw_token_input(
                    frame,
                    &app.token_input,
                    app.character_index as u16,
                    app.token_limit_reached(),
                );
            }
        }
        Mode::Select => {
            if !app.waiting_for_repos {
                if let Some(repositories) = app.repositories.as_mut() {
                    render_list(&mut repositories.repo_items, chunks[1], frame.buffer_mut());
                }
            }
        }
        Mode::Confirm => {
            if let Some(repositories) = &app.repositories {
                render_popup_content(frame, &repositories.repo_items.repos);
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

fn draw_token_input(
    frame: &mut Frame,
    input: &str,
    character_index: u16,
    token_limit_reached: bool,
) {
    let input_area = centered_rect(60, 10, frame.area());

    let (title, style) = if token_limit_reached {
        (
            " Token length limit reached ",
            Style::default().fg(Color::Red),
        )
    } else {
        (" Please paste your token here ", Style::default())
    };

    let key_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(style);

    let token_text = Paragraph::new(input).style(style).block(key_block).clone();
    frame.render_widget(token_text, input_area);
    frame.set_cursor_position(Position::new(
        // Draw the cursor at the current position in the input field.
        // This position is can be controlled via the left and right arrow key
        input_area.x + character_index + 1,
        // Move one line down, from the border to the input line
        input_area.y + 1,
    ));
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

// TODO: Make this scrollable
fn render_popup_content(frame: &mut Frame, repos: &Vec<RepoItem>) {
    let mut lines = vec![];
    for r in repos {
        if r.status == Status::Selected {
            lines.push(Line::from(r.name.clone()))
        }
    }
    let text = Text::from(lines);
    let p = Paragraph::new(text)
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::bordered());

    let area = popup_area(frame.area(), 60, 40);
    frame.render_widget(Clear, area); // this clears out the background
    frame.render_widget(p, area);
}
