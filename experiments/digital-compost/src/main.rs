mod compost;
mod decay;
mod ui;

use crate::compost::CompostBin;
use crate::decay::apply_decay;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::ListState;
use std::fs;
use std::time::Duration;
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // Scan current directory
    let root = std::env::current_dir()?;
    let bin = CompostBin::scan(&root)?;

    let mut list_state = ListState::default();
    if !bin.files.is_empty() {
        list_state.select(Some(0));
    }

    let mut scroll = 0;
    let mut content_cache: Option<(usize, String)> = None; // (file_index, decayed_content)

    loop {
        // Update content if selection changed or cache invalid
        if let Some(selected) = list_state.selected() {
            if selected < bin.files.len() {
                let file = &bin.files[selected];

                // Check cache
                let should_load = if let Some((idx, _)) = content_cache {
                    idx != selected
                } else {
                    true
                };

                if should_load {
                    // Read file
                    if let Ok(raw_content) = fs::read_to_string(&file.path) {
                        let decayed = apply_decay(&raw_content, file.decay_level);
                        content_cache = Some((selected, decayed));
                    } else {
                        content_cache = Some((selected, "Binary or unreadable file.".to_string()));
                    }
                    scroll = 0;
                }
            }
        } else {
            content_cache = None;
        }

        tui.terminal.draw(|f| {
            let content = content_cache.as_ref().map(|(_, c)| c.clone());
            ui::draw(f, &bin, &mut list_state, &content, scroll);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Down | KeyCode::Char('j') => {
                            if bin.files.is_empty() {
                                continue;
                            }
                            let i = match list_state.selected() {
                                Some(i) => {
                                    if i >= bin.files.len() - 1 {
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
                            if bin.files.is_empty() {
                                continue;
                            }
                            let i = match list_state.selected() {
                                Some(i) => {
                                    if i == 0 {
                                        bin.files.len() - 1
                                    } else {
                                        i - 1
                                    }
                                }
                                None => 0,
                            };
                            list_state.select(Some(i));
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            // Scroll down content
                            scroll = scroll.saturating_add(1);
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            // Scroll up content
                            scroll = scroll.saturating_sub(1);
                        }
                        KeyCode::PageDown => {
                            scroll = scroll.saturating_add(10);
                        }
                        KeyCode::PageUp => {
                            scroll = scroll.saturating_sub(10);
                        }
                        KeyCode::Char('r') => {
                            // Refresh decay (regenerate random noise)
                            // By clearing cache, next loop will reload
                            content_cache = None;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
