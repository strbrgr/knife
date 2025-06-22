use crate::components::list::{Repositories, Status};

pub enum Mode {
    Welcome,
    Auth,
    Select,
    Confirm,
}

pub struct App {
    pub exit: bool,
    /// Position of cursor in the editor area.
    pub character_index: usize,
    pub token: String,
    /// Current value of the input box
    pub token_input: String,
    /// Are we waiting for the token
    pub waiting_for_token: bool,
    /// Current mode of the app
    pub mode: Mode,
    pub waiting_for_repos: bool,
    /// Data that is being fetched from github
    pub repositories: Option<Repositories>,
}

impl App {
    pub fn new() -> App {
        App {
            exit: false,
            character_index: 0,
            token: String::new(),
            token_input: String::new(),
            waiting_for_token: false,
            mode: Mode::Welcome,
            waiting_for_repos: false,
            repositories: None,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
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
        let index = self.byte_index();
        self.token_input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
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
        if let Some(repositories) = self.repositories.as_mut() {
            repositories.repo_items.list_state.select_next();
        }
    }

    pub fn select_previous(&mut self) {
        if let Some(repositories) = self.repositories.as_mut() {
            repositories.repo_items.list_state.select_previous();
        }
    }

    pub fn toggle_status(&mut self) {
        if let Some(repositories) = self.repositories.as_mut() {
            if let Some(i) = repositories.repo_items.list_state.selected() {
                repositories.repo_items.repos[i].status =
                    match repositories.repo_items.repos[i].status {
                        Status::Selected => Status::Unselected,
                        Status::Unselected => Status::Selected,
                    };
            }
        }
    }
}
