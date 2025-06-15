use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{
        Color, Modifier, Style, Stylize,
        palette::{
            material::{BLUE, GREEN},
            tailwind::SLATE,
        },
    },
    symbols,
    text::Line,
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget},
};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

pub struct RepoList {
    pub repos: Option<Vec<RepoItem>>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct RepoItem {
    pub repo: String,
    pub status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    Selected,
    Unselected,
}

impl FromIterator<(Status, &'static str, &'static str)> for RepoList {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
        let repos = iter
            .into_iter()
            .map(|(status, repo, _)| RepoItem::new(status, repo))
            .collect();
        let state = ListState::default();
        Self {
            repos: Some(repos),
            state,
        }
    }
}

impl RepoItem {
    fn new(status: Status, repo: &str) -> Self {
        Self {
            status,
            repo: repo.to_string(),
        }
    }
}

pub fn render_list(repo_list: &mut RepoList, area: Rect, buf: &mut Buffer) {
    let block = Block::new()
        .title(Line::raw("Repo List").centered())
        .borders(Borders::TOP)
        .border_set(symbols::border::EMPTY)
        .border_style(TODO_HEADER_STYLE)
        .bg(NORMAL_ROW_BG);

    let items: Vec<ListItem> = repo_list
        .repos
        .as_ref()
        .unwrap_or(&vec![])
        .iter()
        .enumerate()
        .map(|(i, repo_item)| {
            let color = alternate_colors(i);
            ListItem::from(repo_item).bg(color)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
    // same method name `render`.
    StatefulWidget::render(list, area, buf, &mut repo_list.state);
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl From<&RepoItem> for ListItem<'_> {
    fn from(value: &RepoItem) -> Self {
        let line = match value.status {
            Status::Unselected => Line::styled(format!(" ☐ {}", value.repo), TEXT_FG_COLOR),
            Status::Selected => Line::styled(format!(" ✓ {}", value.repo), COMPLETED_TEXT_FG_COLOR),
        };
        ListItem::new(line)
    }
}
