mod analysis;
mod git;
mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::ListState;
use std::time::Duration;
use tui_shared::Tui;

fn main() -> Result<()> {
    // 1. Scan first (simplest approach)
    println!("👻 Summoning spirits from git history... (Scanning last 1000 commits)");
    let commits = git::get_git_log(1000)?;
    let files = analysis::analyze_repository(&commits);

    if files.is_empty() {
        println!("No ghosts found. Is this a git repository?");
        return Ok(());
    }

    // 2. Initialize TUI
    let mut tui = Tui::init()?;

    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        tui.terminal.draw(|f| {
            ui::draw(f, &files, &mut list_state);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Down | KeyCode::Char('j') => {
                            let i = match list_state.selected() {
                                Some(i) => {
                                    if i >= files.len() - 1 {
                                        0
                                    } else {
                                        i + 1
                                    }
                                }
                                None => 0,
                            };
                            list_state.select(Some(i));
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            let i = match list_state.selected() {
                                Some(i) => {
                                    if i == 0 {
                                        files.len() - 1
                                    } else {
                                        i - 1
                                    }
                                }
                                None => 0,
                            };
                            list_state.select(Some(i));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
