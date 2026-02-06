mod etym;
mod ui;

use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use git2::Repository;
use std::{env, path::Path, time::Duration};
use tui_shared::Tui;
use crate::etym::EtymologyTracer;
use crate::ui::{AppState, AppMode, draw_ui};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Default to current dir if no args, but we need a file.
    if args.len() < 3 {
        eprintln!("Usage: git-etymology <repo_path> <file_rel_path>");
        eprintln!("Example: git-etymology . src/main.rs");
        return Ok(());
    }

    let repo_path = &args[1];
    let file_path = &args[2];

    // Initialize Tracer
    let tracer = EtymologyTracer::new(repo_path)?;

    // Read file content from HEAD
    let repo = Repository::discover(repo_path).context("Failed to open repository for reading HEAD")?;
    let head = repo.head().context("HEAD not found")?;
    let commit = head.peel_to_commit().context("HEAD is not a commit")?;
    let tree = commit.tree()?;
    let entry = tree.get_path(Path::new(file_path)).context("File not found in HEAD")?;
    let object = entry.to_object(&repo)?;
    let blob = object.as_blob().context("Not a blob")?;
    let content = std::str::from_utf8(blob.content()).context("File is not UTF-8")?.to_string();
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    // Init TUI
    let mut tui = Tui::init()?;
    let mut app = AppState::new(file_path.to_string(), lines);

    run_app(&mut tui, &mut app, &tracer)?;

    Ok(())
}

fn run_app(tui: &mut Tui, app: &mut AppState, tracer: &EtymologyTracer) -> Result<()> {
    loop {
        tui.terminal.draw(|f| draw_ui(f, app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Down | KeyCode::Char('j') => app.next_line(),
                        KeyCode::Up | KeyCode::Char('k') => app.prev_line(),
                        KeyCode::Char(' ') => {
                            // Toggle mode
                             if app.mode == AppMode::Browsing {
                                 app.mode = AppMode::Tracing;
                             } else {
                                 app.mode = AppMode::Browsing;
                             }
                        }
                        KeyCode::Enter => {
                            if app.mode == AppMode::Browsing {
                                if let Some(idx) = app.list_state.selected() {
                                    app.status_msg = format!("Tracing line {}...", idx + 1);
                                    tui.terminal.draw(|f| draw_ui(f, app))?; // Force redraw with status

                                    // Run trace (blocking for now)
                                    // Line numbers are 1-based in Tracer
                                    match tracer.trace_line(&app.file_path, idx + 1) {
                                        Ok(trace) => {
                                            app.trace = trace;
                                            app.trace_state.select(Some(0));
                                            app.mode = AppMode::Tracing; // Auto switch to trace view
                                            app.status_msg = "Trace complete.".to_string();
                                        }
                                        Err(e) => {
                                            app.status_msg = format!("Error: {}", e);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
