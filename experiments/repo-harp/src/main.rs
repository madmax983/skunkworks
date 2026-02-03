mod dsp;
mod audio;
mod model;

use crate::audio::{AudioSystem, AudioEngine};
use crate::model::scan_directory;
use tui_shared::Tui;
use ratatui::{
    widgets::{Block, Borders, Sparkline, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // 1. Setup TUI
    let mut tui = Tui::init()?;

    // 2. Scan Files
    let root = std::env::current_dir()?;
    // If we are in the crate dir, maybe go up to repo root?
    // Let's just use current dir for now.
    let repo_strings = scan_directory(&root);

    // 3. Init Audio
    let audio_engine = AudioEngine::new()?;

    let dsp_strings = repo_strings.iter().map(|s| s.to_dsp(44100.0)).collect();
    audio_engine.set_strings(dsp_strings);

    let mut selected_index = 0;
    if repo_strings.is_empty() {
        // Handle empty case gracefully?
        // Just let it be 0.
    }

    // 4. Main Loop
    loop {
        // Update physics (if needed for dummy)
        audio_engine.update();

        // Get Visual State
        let visual_state = audio_engine.get_visual_state();

        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(f.area());

            // Render Vibration
            // We visualize the amplitude envelope of each string
            let data: Vec<u64> = visual_state.iter()
                .map(|&v| (v * 1000.0).clamp(0.0, 100.0) as u64)
                .collect();

            let sparkline = Sparkline::default()
                .block(Block::default().title("Repo Strings (Vibration)").borders(Borders::ALL))
                .data(&data)
                .style(Style::default().fg(Color::Cyan));

            f.render_widget(sparkline, chunks[0]);

            // Render Info
            let status = if !repo_strings.is_empty() {
                let s = &repo_strings[selected_index];
                format!(
                    "Selected: {} | Size: {} bytes | Age: {}s | Freq: {:.1}Hz | [Arrows] Select, [Space] Pluck, [Q] Quit",
                    s.path.file_name().unwrap_or_default().to_string_lossy(),
                    s.file_size,
                    s.age_seconds,
                    s.frequency
                )
            } else {
                "No files found in current directory.".to_string()
            };

            f.render_widget(Paragraph::new(status).block(Block::default().borders(Borders::ALL)), chunks[1]);
        })?;

        // Input Handling
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Left => {
                        if selected_index > 0 { selected_index -= 1; }
                    }
                    KeyCode::Right => {
                        if selected_index + 1 < repo_strings.len() { selected_index += 1; }
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if !repo_strings.is_empty() {
                            audio_engine.pluck(selected_index);
                        }
                    }
                    KeyCode::Char('r') => {
                        if !repo_strings.is_empty() {
                            // Strum random
                            let idx = rand::random::<usize>() % repo_strings.len();
                            audio_engine.pluck(idx);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
