use indoc::indoc;
use ratatui::{buffer::Buffer, layout::Rect, text::Text, widgets::Widget};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Logo {
    size: Size,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Size {
    #[default]
    Medium,
}

impl Logo {
    pub const fn new(size: Size) -> Self {
        Self { size }
    }
}

impl Widget for Logo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let logo = self.size.as_str();
        Text::raw(logo).render(area, buf);
    }
}

impl Size {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Medium => Self::medium(),
        }
    }

    const fn medium() -> &'static str {
        indoc! {"
            ██ ▄█▀ ███▄    █  ██▓  █████▒▓█████ 
            ██▄█▒  ██ ▀█   █ ▓██▒▓██   ▒ ▓█   ▀ 
            ▓███▄░ ▓██  ▀█ ██▒▒██▒▒████ ░ ▒███   
            ▓██ █▄ ▓██▒  ▐▌██▒░██░░▓█▒  ░ ▒▓█  ▄ 
            ▒██▒ █▄▒██░   ▓██░░██░░▒█░    ░▒████▒
            ▒ ▒▒ ▓▒░ ▒░   ▒ ▒ ░▓   ▒ ░    ░░ ▒░ ░
            ░ ░▒ ▒░░ ░░   ░ ▒░ ▒ ░ ░       ░ ░  ░
            ░ ░░ ░    ░   ░ ░  ▒ ░ ░ ░       ░   
            ░  ░            ░  ░             ░  ░
        "}
    }
}
