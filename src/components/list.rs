use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize, palette::material::GREEN},
    text::Line,
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget},
};

const SELECTED_STYLE: Style = Style::new().bg(Color::Gray).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = Color::White;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

pub struct Repositories {
    pub repo_owner: String,
    pub repo_items: RepositoryInfo,
}

pub struct RepositoryInfo {
    pub repos: Vec<RepoItem>,
    pub list_state: ListState,
}

#[derive(Debug, Clone)]
pub struct RepoItem {
    pub name: String,
    pub status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    Selected,
    Unselected,
}

impl FromIterator<(Status, &'static str, &'static str)> for RepositoryInfo {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
        let repos = iter
            .into_iter()
            .map(|(status, repo, _)| RepoItem::new(status, repo))
            .collect();
        let list_state = ListState::default();
        Self { repos, list_state }
    }
}

impl RepoItem {
    fn new(status: Status, name: &str) -> Self {
        Self {
            status,
            name: name.to_string(),
        }
    }
}

pub fn render_list(repo_list: &mut RepositoryInfo, area: Rect, buf: &mut Buffer) {
    let block = Block::new()
        .title(Line::raw("Your public repositories").centered())
        .borders(Borders::TOP)
        .style(Style::default().fg(Color::LightRed));

    let items: Vec<ListItem> = repo_list
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
    StatefulWidget::render(list, area, buf, &mut repo_list.list_state);
}

impl From<&RepoItem> for ListItem<'_> {
    fn from(value: &RepoItem) -> Self {
        let line = match value.status {
            Status::Unselected => Line::styled(format!(" ☐ {}", value.name), TEXT_FG_COLOR),
            Status::Selected => Line::styled(format!(" ✓ {}", value.name), COMPLETED_TEXT_FG_COLOR),
        };
        ListItem::new(line)
    }
}
