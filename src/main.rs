use ratatui::{
    Terminal,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::{Backend, CrosstermBackend},
};
use std::io;
use ui::ui;
use utils::{get_repos_with, get_user_with};

use crate::app::{App, Mode};

mod app;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    run_app(&mut terminal, &mut app).await?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    while !app.exit {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key_event) = event::read()? {
            // Exit the app
            if key_event.modifiers.contains(KeyModifiers::CONTROL)
                && key_event.code == KeyCode::Char('c')
            {
                app.exit();
            }
            if key_event.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.mode {
                Mode::Welcome => {
                    if key_event.code == KeyCode::Enter {
                        let path = "https://github.com/settings/tokens/new?scopes=delete_repo,repo&description=Repo%20Remover%20Token";

                        app.waiting_for_token = true;
                        app.mode = Mode::Auth;

                        if let Err(e) = open::that(path) {
                            eprintln!("Failed to open browser: {e}");
                        }
                    }
                }
                Mode::Auth => match key_event.code {
                    KeyCode::Char(value) => {
                        app.token_input.push(value);
                    }
                    KeyCode::Enter => {
                        app.token = app.token_input.clone();
                        app.token_input = String::new();
                        app.waiting_for_token = false;
                        let user = get_user_with(&app.token).await?;
                        let repos = get_repos_with(user.as_str(), &app.token).await?;
                        app.repos = Some(repos);
                        app.mode = Mode::Select;
                    }
                    _ => {}
                },
                Mode::Select => {}
            }
        }
    }

    Ok(())
}
