use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Wrap,
    },
};

const SELECTED_STYLE: Style = Style::new().bg(Color::Gray).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = Color::White;
const COMPLETED_TEXT_FG_COLOR: Color = Color::LightCyan;
pub const LIGHT_RED: Color = Color::LightRed;
pub const DARK_GRAY: Color = Color::DarkGray;

pub struct GithubContent {
    pub owner: String,
    pub repos: Vec<Repository>,
    pub list_state: ListState,
}

#[derive(Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    Selected,
    Unselected,
}

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

pub fn render_selected_repositories(frame: &mut Frame, repos: &[Repository]) {
    let selected_repos: Vec<String> = repos
        .iter()
        .filter(|r| r.status == Status::Selected)
        .map(|r| r.name.clone())
        .collect();

    let content = selected_repos.join(", ");
    let text = Text::from(content);
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red))
        .wrap(Wrap { trim: true });
    let area = popup_area(frame.area(), 80, 40);
    frame.render_widget(paragraph, area);
}

pub fn render_all_repositories(github_content: &mut GithubContent, area: Rect, buf: &mut Buffer) {
    let block = Block::new()
        .title(Line::raw("Your public repositories").centered())
        .borders(Borders::TOP)
        .style(Style::default().fg(Color::LightRed));

    let items: Vec<ListItem> = github_content
        .repos
        .iter()
        .map(|repo_item| ListItem::from(repo_item).bg(Color::Reset))
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
    // same method name `render`.
    StatefulWidget::render(list, area, buf, &mut github_content.list_state);
}

impl From<&Repository> for ListItem<'_> {
    fn from(value: &Repository) -> Self {
        let line = match value.status {
            Status::Unselected => Line::styled(format!(" ☐ {}", value.name), TEXT_FG_COLOR),
            Status::Selected => Line::styled(format!(" ✓ {}", value.name), COMPLETED_TEXT_FG_COLOR),
        };
        ListItem::new(line)
    }
}
