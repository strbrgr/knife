use crate::components::list::{Repositories, Status};

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
    pub github_data: Option<Repositories>,
}

impl App {
    pub fn new() -> App {
        App {
            exit: false,
            token: String::new(),
            token_input: String::new(),
            waiting_for_token: false,
            mode: Mode::Welcome,
            waiting_for_repos: false,
            github_data: None,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn select_next(&mut self) {
        if let Some(github_data) = self.github_data.as_mut() {
            github_data.repo_items.list_state.select_next();
        }
    }

    pub fn select_previous(&mut self) {
        if let Some(github_data) = self.github_data.as_mut() {
            github_data.repo_items.list_state.select_previous();
        }
    }

    pub fn toggle_status(&mut self) {
        if let Some(github_data) = self.github_data.as_mut() {
            if let Some(i) = github_data.repo_items.list_state.selected() {
                github_data.repo_items.repos[i].status =
                    match github_data.repo_items.repos[i].status {
                        Status::Selected => Status::Unselected,
                        Status::Unselected => Status::Selected,
                    };
            }
        }
    }
}
