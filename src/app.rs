use ratatui::widgets::ListState;

use crate::components::list::{RepoList, Status};

pub enum Mode {
    Welcome,
    Auth,
    Select,
    Confirm,
}

pub struct App {
    pub exit: bool,
    pub token: String,
    pub token_input: String,
    pub waiting_for_token: bool,
    pub mode: Mode,
    pub waiting_for_repos: bool,
    pub repo_list: RepoList,
}

impl App {
    pub fn new() -> App {
        let repo_list = RepoList {
            repos: None,
            state: ListState::default(),
        };

        App {
            exit: false,
            token: String::new(),
            token_input: String::new(),
            waiting_for_token: false,
            mode: Mode::Welcome,
            waiting_for_repos: false,
            repo_list,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn select_next(&mut self) {
        self.repo_list.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.repo_list.state.select_previous();
    }

    pub fn toggle_status(&mut self) {
        if let Some(repos) = self.repo_list.repos.as_mut() {
            if let Some(i) = self.repo_list.state.selected() {
                repos[i].status = match repos[i].status {
                    Status::Selected => Status::Unselected,
                    Status::Unselected => Status::Selected,
                };
            }
        }
    }
}
