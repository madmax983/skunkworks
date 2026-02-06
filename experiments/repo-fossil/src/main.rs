mod git;
mod entropy;
mod tui;
mod audio;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tui::{AppState, Mode, draw};
use git::{load_history, get_files_in_commit, get_file_content};
use entropy::fossilize;
use audio::Geiger;

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Audio
    let geiger = Geiger::new();

    // App State
    let commits = load_history()?;
    let mut app_state = AppState::new(commits);

    // Initial load of files for the first commit if available
    if let Some(first) = app_state.commits.first() {
        if let Ok(files) = get_files_in_commit(&first.hash) {
            app_state.files = files;
            if !app_state.files.is_empty() {
                app_state.file_list_state.select(Some(0));
            }
        }
    }

    let res = run_app(&mut terminal, &mut app_state, &geiger);

    // Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, state: &mut AppState, geiger: &Geiger) -> Result<()> {
    loop {
        terminal.draw(|f| draw(f, state))?;

        if let Event::Key(key) = event::read()? {
            match state.mode {
                Mode::CommitSelect => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Down | KeyCode::Char('j') => {
                            state.next_commit();
                            geiger.click();
                            // Refresh files
                            if let Some(idx) = state.commit_list_state.selected() {
                                if let Some(commit) = state.commits.get(idx) {
                                    if let Ok(files) = get_files_in_commit(&commit.hash) {
                                        state.files = files;
                                        state.file_list_state.select(Some(0));
                                        state.current_fossil = None; // Clear current view
                                    }
                                }
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            state.previous_commit();
                            geiger.click();
                            // Refresh files
                             if let Some(idx) = state.commit_list_state.selected() {
                                if let Some(commit) = state.commits.get(idx) {
                                    if let Ok(files) = get_files_in_commit(&commit.hash) {
                                        state.files = files;
                                        state.file_list_state.select(Some(0));
                                        state.current_fossil = None;
                                    }
                                }
                            }
                        }
                        KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                            state.mode = Mode::FileSelect;
                        }
                        _ => {}
                    }
                }
                Mode::FileSelect => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left | KeyCode::Char('h') => {
                            state.mode = Mode::CommitSelect;
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            state.next_file();
                            geiger.click();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            state.previous_file();
                            geiger.click();
                        }
                        KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                             // Load File Content
                             if let Some(commit_idx) = state.commit_list_state.selected() {
                                 if let Some(file_idx) = state.file_list_state.selected() {
                                     let commit = &state.commits[commit_idx];
                                     if let Some(path) = state.files.get(file_idx) {
                                         if let Ok(content) = get_file_content(&commit.hash, path) {
                                             use std::collections::hash_map::DefaultHasher;
                                             use std::hash::{Hash, Hasher};

                                             let mut hasher = DefaultHasher::new();
                                             commit.hash.hash(&mut hasher);
                                             let seed = hasher.finish();

                                             let age_factor = (commit_idx as f64) / 50.0;
                                             let fossil = fossilize(&content, age_factor, seed);
                                             state.current_fossil = Some(fossil);
                                             state.mode = Mode::Excavation;
                                             geiger.click();
                                         }
                                     }
                                 }
                             }
                        }
                        _ => {}
                    }
                }
                Mode::Excavation => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Esc | KeyCode::Left | KeyCode::Char('h') => {
                            state.mode = Mode::FileSelect;
                        }
                        // TODO: Add scrolling
                        _ => {}
                    }
                }
            }
        }
    }
}
