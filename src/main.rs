use crate::app::App;
use ratatui::DefaultTerminal;

mod app;
mod github;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terminal = ratatui::init();
    run(terminal).await?;
    ratatui::restore();

    Ok(())
}

async fn run(terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    App::new().run(terminal).await?;

    Ok(())
}
