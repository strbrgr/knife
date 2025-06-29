use ratatui::DefaultTerminal;

use crate::app::App;

mod app;
mod components;
mod utils;

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
