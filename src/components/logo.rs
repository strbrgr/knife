use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget},
};

pub struct Logo {
    content: String,
}

impl Widget for Logo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(self.content)
            .alignment(ratatui::layout::Alignment::Center)
            .style(Style::default().fg(Color::LightRed))
            .block(Block::new());

        paragraph.render(area, buf);
    }
}

impl Logo {
    pub fn new() -> Self {
        Logo {
            content: get_ascii_logo(),
        }
    }
}

fn get_ascii_logo() -> String {
    let ascii_art = r#"
██ ▄█▀ ███▄    █  ██▓  █████▒▓█████ 
██▄█  ██ ▀█   █ ▓██▒▓██   ▒ ▓█   ▀ 
▓███▄░ ▓██  ▀█ ██▒▒██▒▒████ ░ ▒███   
▓██ █▄ ▓██▒  ▐▌██▒░██░░▓█▒  ░ ▒▓█  ▄ 
▒██▒ █▄▒██░   ▓██░░██░░▒█░    ░▒████▒
▒ ▒▒ ▓▒░ ▒░   ▒ ▒ ░▓   ▒ ░    ░░ ▒░ ░
░ ░▒ ▒░░ ░░   ░ ▒░ ▒ ░ ░       ░ ░  ░
░ ░░ ░    ░   ░ ░  ▒ ░ ░ ░       ░   
░  ░            ░  ░             ░  ░
"#;

    ascii_art.to_string()
}
