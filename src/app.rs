pub enum Screen {
    Welcome,
    Auth,
}

pub struct App {
    pub token: String,
    pub token_input: String,
    pub waiting_for_token: bool,
    pub current_screen: Screen,
}

impl App {
    pub fn new() -> App {
        App {
            token: String::new(),
            token_input: String::new(),
            waiting_for_token: false,
            current_screen: Screen::Welcome,
        }
    }

    pub fn insert_token(&mut self) {
        self.token = self.token_input.clone();
        self.token_input = String::new();
    }
}
