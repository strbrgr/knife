use app::ErrorState;
use components::list::Status;
use ratatui::{
    Terminal,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::{Backend, CrosstermBackend},
};
use reqwest::StatusCode;
use std::io;
use utils::{github::delete_repo, ui::ui};

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
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    KeyCode::Enter => {
                        app.submit_message();
                        app.token = app.token_input.clone();
                        app.token_input = String::new();
                        app.waiting_for_token = false;
                        app.waiting_for_repos = true;
                        let repositories = utils::github::get_data_from_github(&app.token).await?;
                        app.repositories = Some(repositories);
                        app.waiting_for_repos = false;
                        app.mode = Mode::Select;
                    }
                    KeyCode::Backspace => app.delete_char(),
                    KeyCode::Left => app.move_cursor_left(),
                    KeyCode::Right => app.move_cursor_right(),
                    KeyCode::Esc => {
                        app.mode = Mode::Welcome;
                        app.reset_cursor();
                        app.token = String::new();
                        app.token_input = String::new();
                    }
                    _ => {}
                },
                Mode::Select => match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.exit(),
                    KeyCode::Down | KeyCode::Char('j') => app.select_next(),
                    KeyCode::Up | KeyCode::Char('k') => app.select_previous(),
                    KeyCode::Char(' ') => {
                        app.toggle_status();
                    }
                    KeyCode::Enter => {
                        if let Some(repositories) = &app.repositories {
                            let at_least_one_selected = repositories
                                .repo_items
                                .repos
                                .iter()
                                .any(|repo| repo.status == Status::Selected);

                            if at_least_one_selected {
                                app.mode = Mode::Confirm
                            } else {
                                // TODO: error case change colors
                            }
                        }
                    }
                    _ => {}
                },
                Mode::Confirm => {
                    if key_event.code == KeyCode::Enter {
                        if let Some(repositories) = &mut app.repositories {
                            let selected_repos: Vec<String> = repositories
                                .repo_items
                                .repos
                                .iter()
                                .filter(|r| r.status == Status::Selected)
                                .map(|r| r.name.clone())
                                .collect();

                            for repo_name in &selected_repos {
                                let status_code =
                                    delete_repo(&repositories.owner, repo_name, &app.token).await?;

                                if status_code.is_client_error() {
                                    app.error_state = Some(ErrorState::DeleteClientError);
                                }

                                if status_code == StatusCode::NO_CONTENT {
                                    repositories.repo_items.repos = repositories
                                        .repo_items
                                        .repos
                                        .iter()
                                        .filter(|repo| &repo.name != repo_name)
                                        .cloned()
                                        .collect();
                                }
                            }

                            app.mode = Mode::Select;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
