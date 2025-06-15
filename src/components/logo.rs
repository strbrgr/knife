use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Style},
    widgets::Widget,
};

pub struct Logo<'a> {
    ascii: &'a str,
}

impl<'a> Logo<'a> {
    pub fn new() -> Self {
        Self {
            ascii: r#"
  ██ ▄█▀███▄    █ ██▓ █████▓█████ 
  ██▄█▒ ██ ▀█   █▓██▓██   ▒▓█   ▀ 
▓███▄░▓██  ▀█ ██▒██▒████ ░▒███   
▓██ █▄▓██▒  ▐▌██░██░▓█▒  ░▒▓█  ▄ 
▒██▒ █▒██░   ▓██░██░▒█░   ░▒████▒
▒ ▒▒ ▓░ ▒░   ▒ ▒░▓  ▒ ░   ░░ ▒░ ░
░ ░▒ ▒░ ░░   ░ ▒░▒ ░░      ░ ░  ░
░ ░░ ░   ░   ░ ░ ▒ ░░ ░      ░   
░  ░           ░ ░           ░  ░
"#,
        }
    }
}

impl<'a> Widget for Logo<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut y = area.y;
        let dark_red = Color::Rgb(100, 0, 0);
        let medium_dark_red = Color::Rgb(150, 0, 0);
        let medium_red = Color::Rgb(200, 0, 0);
        let bright_red = Color::Rgb(255, 50, 50);

        for line in self.ascii.lines() {
            if y >= area.y + area.height {
                break;
            }

            // Trim trailing whitespace to get actual line width
            let trimmed_line = line.trim_end();
            let line_width = trimmed_line.chars().count();

            // Calculate horizontal padding for centering
            let padding = if area.width > line_width as u16 {
                (area.width - line_width as u16) / 2
            } else {
                0
            };

            let mut x = area.x + padding;

            for ch in trimmed_line.chars() {
                if x >= area.x + area.width {
                    break;
                }

                let style = match ch {
                    '█' => Style::default().fg(bright_red),
                    '▀' => Style::default().fg(bright_red),
                    '▄' => Style::default().fg(bright_red),
                    '▌' => Style::default().fg(bright_red),
                    '▐' => Style::default().fg(bright_red),
                    '▓' => Style::default().fg(bright_red),
                    '▒' => Style::default().fg(bright_red),
                    '░' => Style::default().fg(bright_red),
                    _ => {
                        x += 1;
                        continue;
                    }
                };

                let position = Position::new(x, y);
                buf.cell_mut(position)
                    .expect("Could not get cell")
                    .set_char(ch)
                    .set_style(style);
                x += 1;
            }

            y += 1;
        }
    }
}
