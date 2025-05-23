pub enum Mode {
    Welcome,
    Auth,
    Select,
}

pub struct App {
    pub exit: bool,
    pub token: String,
    pub token_input: String,
    pub waiting_for_token: bool,
    pub mode: Mode,
}

impl App {
    pub fn new() -> App {
        App {
            exit: false,
            token: String::new(),
            token_input: String::new(),
            waiting_for_token: false,
            mode: Mode::Welcome,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}
