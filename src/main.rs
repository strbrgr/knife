use components::list::RepoList;
use ratatui::{
    Terminal,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::{Backend, CrosstermBackend},
    widgets::ListState,
};
use std::io;
use utils::ui::ui;

use crate::app::{App, Mode};

mod app;
mod components;
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
                        app.waiting_for_repos = true;
                        let repos = utils::github::get_repos_from_github(&app.token).await?;
                        app.repo_list = RepoList {
                            repos: Some(repos),
                            state: ListState::default(),
                        };
                        app.waiting_for_repos = false;
                        app.mode = Mode::Select;
                    }
                    _ => {}
                },
                Mode::Select => match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.exit(),
                    KeyCode::Char('j') | KeyCode::Down => app.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => app.select_previous(),
                    KeyCode::Char('g') | KeyCode::Home => app.select_first(),
                    KeyCode::Char('G') | KeyCode::End => app.select_last(),
                    KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                        app.toggle_status();
                    }
                    _ => {}
                },
            }
        }
    }

    Ok(())
}
