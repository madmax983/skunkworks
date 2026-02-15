use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

use digital_decay::{
    decay::bit_rot,
    scanner::scan_dir,
    ui::{draw_ui, AppState},
};
use rand::Rng;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initial scan
    // Scan the experiments directory
    let root = std::path::Path::new("experiments");
    let files = scan_dir(root);

    // App State
    let mut app = AppState {
        files,
        selected_index: 0,
        entropy: 0.0,
        inspector_view: false,
        decayed_content: None,
        scroll_offset: 0,
    };

    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> Result<()>
where
    <B as ratatui::backend::Backend>::Error: Send + Sync + 'static,
{
    loop {
        // Update simulation
        if app.entropy > 0.0 {
            let mut rng = rand::thread_rng();
            for file in &mut app.files {
                // Simulate health decay based on entropy and random noise
                let noise = rng.gen_range(0.8..1.2);
                file.health = (1.0 - (app.entropy * noise)).clamp(0.0, 1.0);
            }
        } else {
            for file in &mut app.files {
                file.health = 1.0;
            }
        }

        terminal.draw(|f| draw_ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Esc => {
                            if app.inspector_view {
                                app.inspector_view = false;
                                app.decayed_content = None;
                            } else {
                                return Ok(());
                            }
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            app.entropy = (app.entropy + 0.05).min(1.0);
                            if app.inspector_view {
                                update_inspector_content(app);
                            }
                        }
                        KeyCode::Char('-') | KeyCode::Char('_') => {
                            app.entropy = (app.entropy - 0.05).max(0.0);
                            if app.inspector_view {
                                update_inspector_content(app);
                            }
                        }
                        KeyCode::Char('r') => {
                            // "Repair" just re-reads the file without corruption
                            if app.inspector_view {
                                app.decayed_content = load_file_content(app);
                            }
                        }
                        KeyCode::Enter => {
                            if !app.inspector_view && !app.files.is_empty() {
                                app.inspector_view = true;
                                update_inspector_content(app);
                            }
                        }

                        // Navigation
                        KeyCode::Down | KeyCode::Char('j') => {
                            if app.inspector_view {
                                app.scroll_offset = app.scroll_offset.saturating_add(1);
                            } else {
                                // TODO: Improve grid navigation (move by row)
                                // For now, just move linear
                                if app.selected_index + 1 < app.files.len() {
                                    app.selected_index += 1;
                                }
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if app.inspector_view {
                                app.scroll_offset = app.scroll_offset.saturating_sub(1);
                            } else {
                                if app.selected_index > 0 {
                                    app.selected_index -= 1;
                                }
                            }
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                             if !app.inspector_view && app.selected_index > 0 {
                                app.selected_index -= 1;
                            }
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                             if !app.inspector_view && app.selected_index + 1 < app.files.len() {
                                app.selected_index += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn load_file_content(app: &AppState) -> Option<String> {
    if let Some(file) = app.files.get(app.selected_index) {
        // Read first 2KB for preview
        use std::io::Read;
        if let Ok(mut f) = std::fs::File::open(&file.path) {
            let mut buffer = vec![0; 2048];
            if let Ok(n) = f.read(&mut buffer) {
                buffer.truncate(n);
                // Convert to string (lossy)
                return Some(String::from_utf8_lossy(&buffer).to_string());
            }
        }
    }
    None
}

fn update_inspector_content(app: &mut AppState) {
    if let Some(file) = app.files.get(app.selected_index) {
        use std::io::Read;
        if let Ok(mut f) = std::fs::File::open(&file.path) {
            let mut buffer = vec![0; 2048];
            if let Ok(n) = f.read(&mut buffer) {
                buffer.truncate(n);

                // Apply Decay
                if app.entropy > 0.0 {
                    bit_rot(&mut buffer, app.entropy * 0.1); // Scale entropy for visual effect
                }

                // Convert to string (lossy)
                app.decayed_content = Some(String::from_utf8_lossy(&buffer).to_string());
            }
        }
    }
}
