use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    layout::{Constraint, Direction, Layout},
    prelude::Backend,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
};
use reqwest::StatusCode;
use std::io::{self};

use crate::{
    github::RepositoryClient,
    ui::{
        DARK_GRAY, GithubContent, LIGHT_RED, Status, draw_token_input, render_all_repositories,
        render_selected_repositories,
    },
};

pub struct App {
    // Running / Quit state
    pub state: RunningState,
    // Position of cursor in the editor area.
    pub character_index: usize,
    // Token once confirmed
    pub token: String,
    // Current value of the input box
    pub token_input: String,
    // Are we waiting for the token
    pub waiting_for_token: bool,
    // Current mode of the app
    pub mode: Mode,
    // Are we waiting for repos
    pub waiting_for_repos: bool,
    // Data that is being fetched from github
    pub github_content: Option<GithubContent>,
    // Error state for the app
    pub error_state: Option<Error>,
    // Client to get all repositories
    pub repository_client: Option<RepositoryClient>,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Mode {
    Welcome,
    Auth,
    Select,
    Confirm,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Error {
    DeleteRepository,
    GetRepositoryOwner,
    GetRepositories,
    NoRepositorySelected,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Quit,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            state: RunningState::Running,
            character_index: 0,
            token: String::new(),
            token_input: String::new(),
            waiting_for_token: false,
            mode: Mode::Welcome,
            waiting_for_repos: false,
            github_content: None,
            error_state: None,
            repository_client: None,
        }
    }

    pub async fn run(
        &mut self,
        mut terminal: Terminal<impl Backend>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.is_running() {
            self.draw(&mut terminal)?;
            self.handle_events().await?;
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|frame| {
            self.render(frame);
        })?;
        Ok(())
    }

    async fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.modifiers.contains(KeyModifiers::CONTROL)
                && key_event.code == KeyCode::Char('c')
            {
                self.exit();
            }

            match self.mode {
                Mode::Welcome => match key_event.code {
                    KeyCode::Enter => {
                        const PATH: &str = "https://github.com/settings/tokens/new?scopes=delete_repo,repo&description=Repo%20Remover%20Token";
                        self.waiting_for_token = true;
                        self.mode = Mode::Auth;

                        if let Err(e) = open::that(PATH) {
                            eprintln!("Failed to open browser: {e}");
                        }
                    }
                    KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                    _ => {}
                },
                Mode::Auth => match key_event.code {
                    KeyCode::Char(to_insert) => {
                        self.enter_char(to_insert);
                    }
                    KeyCode::Enter => {
                        self.token = self.token_input.clone();
                        self.submit_message();
                        self.waiting_for_token = false;
                        self.waiting_for_repos = true;
                        let repository_client = RepositoryClient::new(&self.token);
                        self.repository_client = Some(repository_client);
                        if let Some(repository_client) = self.repository_client.as_mut() {
                            match repository_client.get_owner().await {
                                Ok(owner) => match repository_client.get_repos(&owner).await {
                                    Ok(github_content) => {
                                        self.github_content = Some(github_content);
                                        self.waiting_for_repos = false;
                                        self.mode = Mode::Select;
                                    }
                                    Err(_) => self.error_state = Some(Error::GetRepositories),
                                },
                                Err(_) => {
                                    self.error_state = Some(Error::GetRepositoryOwner);
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    KeyCode::Esc => {
                        self.mode = Mode::Welcome;
                        self.reset_cursor();
                        self.token = String::new();
                        self.token_input = String::new();
                    }
                    _ => {}
                },
                Mode::Select => match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                    KeyCode::Down | KeyCode::Char('j') => self.select_next(),
                    KeyCode::Up | KeyCode::Char('k') => self.select_previous(),
                    KeyCode::Char(' ') => {
                        self.toggle_status();
                    }
                    KeyCode::Enter => {
                        if let Some(github_content) = &self.github_content {
                            let at_least_one_selected = github_content
                                .repos
                                .iter()
                                .any(|repo| repo.status == Status::Selected);

                            if at_least_one_selected {
                                self.mode = Mode::Confirm
                            } else {
                                self.error_state = Some(Error::NoRepositorySelected);
                            }
                        }
                    }
                    _ => {}
                },
                Mode::Confirm => match key_event.code {
                    KeyCode::Enter => {
                        if let Some(repositories) = &mut self.github_content {
                            let selected_repos: Vec<String> = repositories
                                .repos
                                .iter()
                                .filter(|r| r.status == Status::Selected)
                                .map(|r| r.name.clone())
                                .collect();

                            for repo_name in &selected_repos {
                                let status_code = self
                                    .repository_client
                                    .as_mut()
                                    .unwrap()
                                    .delete_repo(&repositories.owner, repo_name)
                                    .await?;

                                if status_code.is_client_error() {
                                    self.error_state = Some(Error::DeleteRepository);
                                }

                                if status_code == StatusCode::NO_CONTENT {
                                    // Update repository list and remove the ones we just deleted
                                    repositories.repos = repositories
                                        .repos
                                        .iter()
                                        .filter(|repo| &repo.name != repo_name)
                                        .cloned()
                                        .collect();
                                }
                            }

                            // Once deleted we go back to Select Mode
                            self.mode = Mode::Select;
                        }
                    }
                    KeyCode::Esc => {
                        self.mode = Mode::Select;
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }

    pub fn exit(&mut self) {
        self.state = RunningState::Quit;
    }

    pub fn is_running(&self) -> bool {
        self.state == RunningState::Running
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        // Let's not add any more characters if we reached the limit
        if !self.token_limit_reached() {
            let index = self.byte_index();
            self.token_input.insert(index, new_char);
            self.move_cursor_right();
        }
    }

    // Returns the byte index based on the character position.
    //
    // Since each character in a string can be contain multiple bytes, it's necessary to calculate
    // the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.token_input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.token_input.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.token_input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.token_input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.token_input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.token_input.chars().count())
    }

    pub fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn submit_message(&mut self) {
        self.token_input.clear();
        self.reset_cursor();
    }

    pub fn select_next(&mut self) {
        if let Some(github_content) = self.github_content.as_mut() {
            github_content.list_state.select_next();
        }
    }

    pub fn select_previous(&mut self) {
        if let Some(github_content) = self.github_content.as_mut() {
            github_content.list_state.select_previous();
        }
    }

    pub fn toggle_status(&mut self) {
        if let Some(github_content) = self.github_content.as_mut() {
            if let Some(i) = github_content.list_state.selected() {
                github_content.repos[i].status = match github_content.repos[i].status {
                    Status::Selected => Status::Unselected,
                    Status::Unselected => Status::Selected,
                };
            }
        }
    }

    pub fn token_limit_reached(&self) -> bool {
        self.token_input.len() > 40
    }
}

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Percentage(90),
                Constraint::Fill(1),
            ])
            .split(frame.area());

        let body_constraint = match self.mode {
            Mode::Select => Constraint::Length(15),
            Mode::Confirm => Constraint::Length(12),
            _ => Constraint::Length(5),
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),    // 0: Top spacing
                Constraint::Length(10), // 1: Logo area
                Constraint::Length(1),  // 2: Padding between logo and welcome text
                body_constraint,        // 3: Dynamic body height
                Constraint::Length(2),  // 4: Padding between welcome text and footer
                Constraint::Length(1),  // 5: Footer
                Constraint::Fill(1),    // 6: Bottom spacing
            ])
            .split(horizontal_chunks[1]);

        let header = chunks[1];
        let body = chunks[3];
        let footer = chunks[5];

        match self.mode {
            Mode::Auth => {
                let character_index = self.character_index as u16;
                draw_token_input(
                    frame,
                    &self.token_input,
                    character_index,
                    self.token_limit_reached(),
                );
            }
            Mode::Welcome => {
                self.logo().render(header, frame.buffer_mut());
                self.description().render(body, frame.buffer_mut());
                self.footer().render(footer, frame.buffer_mut());
            }
            Mode::Select => {
                if !self.waiting_for_repos {
                    if let Some(github_content) = self.github_content.as_mut() {
                        render_all_repositories(github_content, body, frame.buffer_mut());
                        self.footer().render(footer, frame.buffer_mut());
                    }
                }
            }
            Mode::Confirm => {
                if let Some(github_content) = &self.github_content {
                    render_selected_repositories(frame, &github_content.repos);
                    self.footer().render(footer, frame.buffer_mut());
                }
            }
        }
    }

    fn logo(&self) -> impl Widget {
        let ascii_art = r#"
      :::    ::: ::::    ::: ::::::::::: :::::::::: :::::::::: 
     :+:   :+:  :+:+:   :+:     :+:     :+:        :+:         
    +:+  +:+   :+:+:+  +:+     +:+     +:+        +:+          
   +#++:++    +#+ +:+ +#+     +#+     :#::+::#   +#++:++#      
  +#+  +#+   +#+  +#+#+#     +#+     +#+        +#+            
 #+#   #+#  #+#   #+#+#     #+#     #+#        #+#             
###    ### ###    #### ########### ###        ##########       
"#;
        ascii_art.to_string();

        Paragraph::new(ascii_art)
            .alignment(ratatui::layout::Alignment::Center)
            .style(Style::default().fg(LIGHT_RED))
            .block(Block::new())
    }

    fn description(&self) -> impl Widget {
        let info_text = vec![
            Line::from(String::from(
                "Welcome to knife, a terminal application to delete GitHub repositories.",
            )),
            Line::from(String::from(
                "After hitting 'Enter', your default browser will open and redirect you to the personal access token (PAT) page on Github.",
            )),
            Line::from(String::from(
                "Please use the pre-selected settings and copy the PAT.",
            )),
        ];

        Paragraph::new(Text::from(info_text))
            .alignment(ratatui::layout::Alignment::Center)
            .style(Style::default())
            .block(Block::new())
            .wrap(Wrap { trim: true })
    }

    fn footer(&self) -> impl Widget {
        let footer_text = match self.mode {
            Mode::Welcome => Line::from(vec![
                Span::styled("Hit ", Style::default().fg(DARK_GRAY)),
                Span::styled(
                    "'Enter'",
                    Style::default()
                        .fg(DARK_GRAY)
                        .add_modifier(Modifier::ITALIC)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " to get your Token from Github!",
                    Style::default().fg(DARK_GRAY),
                ),
            ]),
            Mode::Select => Line::from(vec![Span::styled(
                "Use '↓', '↑', 'j', or 'k' to move; 'Space' to toggle status; and 'Enter' to confirm.",
                Style::default().fg(DARK_GRAY),
            )]),
            Mode::Confirm => Line::from(vec![Span::styled(
                "Press 'Enter' to delete the selected repo(s)",
                Style::default().fg(DARK_GRAY),
            )]),
            _ => Line::from("Unknown mode"),
        };

        Paragraph::new(footer_text)
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_limit_reached_returns_true_when_too_long() {
        let mut app = App::new();
        app.token_input = "a".repeat(41);
        let result = app.token_limit_reached();
        assert!(result);
    }
}
