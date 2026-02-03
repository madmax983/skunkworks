use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::{
    sync::Arc,
    time::Duration,
};
use parking_lot::Mutex;
use tui_shared::Tui;

mod audio;
mod engine;
use crate::engine::{Engine, RenderState, TrackState};
use crate::audio::init_audio;

fn main() -> Result<()> {
    // Load Files
    let v1 = include_str!("../v1.rs");
    let v2 = include_str!("../v2.rs");

    // Init Engine
    let engine = Arc::new(Mutex::new(Engine::new(v1, v2)));

    // Init Audio
    let _audio = init_audio(engine.clone())?;

    // Init TUI
    let mut tui = Tui::init()?;

    let mut drift = 0.001;
    engine.lock().set_drift(drift);

    loop {
        // Snapshot State
        let state = {
            let engine = engine.lock();
            engine.get_render_state()
        };

        // Render
        tui.terminal.draw(|f| {
            ui(f, &state);
        })?;

        // Input
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right => {
                        drift += 0.0005;
                        engine.lock().set_drift(drift);
                    }
                    KeyCode::Left => {
                        drift -= 0.0005;
                        if drift < 0.0 { drift = 0.0; }
                        engine.lock().set_drift(drift);
                    }
                    KeyCode::Char(' ') => {
                        drift = 0.0;
                        engine.lock().set_drift(0.0);
                    }
                     _ => {}
                }
            }
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, state: &RenderState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("⚛️ GIT PHASING: Polymetric Diff Sonification")
        .style(Style::default().fg(Color::Cyan).bold())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Status
    let status = Paragraph::new(format!(
        "Drift: {:.4} (Use Arrows to Adjust, Space to Reset) | BPM: {:.1} | Voice 1: {} | Voice 2: {}",
        state.drift, state.bpm, state.track1.name, state.track2.name
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);

    // Main Content
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    draw_track(f, &state.track1, main_chunks[0], Color::Blue);
    draw_track(f, &state.track2, main_chunks[1], Color::Green);
}

fn draw_track(f: &mut Frame, track: &TrackState, area: ratatui::layout::Rect, highlight_color: Color) {
    let mut spans = Vec::new();
    let mut current_line = Vec::new();
    let mut current_width = 0;
    let max_width = (area.width as usize).saturating_sub(2);

    let mut active_line_index = 0;
    let mut current_line_index = 0;

    for (i, token) in track.tokens.iter().enumerate() {
        let is_active = i == track.current_index;

        let style = if is_active {
            active_line_index = current_line_index;
            Style::default().fg(Color::Black).bg(highlight_color)
        } else if token.velocity > 0.8 {
            Style::default().fg(Color::Yellow)
        } else if token.velocity < 0.3 {
            Style::default().fg(Color::Gray)
        } else {
            Style::default().fg(Color::White)
        };

        let content = format!("{} ", token.text);
        let len = content.len();

        // Simple wrapping logic
        if current_width + len > max_width {
            spans.push(Line::from(current_line.clone()));
            current_line.clear();
            current_width = 0;
            current_line_index += 1;
            if is_active {
                active_line_index = current_line_index;
            }
        }

        current_line.push(Span::styled(content, style));
        current_width += len;
    }
    if !current_line.is_empty() {
        spans.push(Line::from(current_line));
    }

    let view_height = (area.height as usize).saturating_sub(2);
    let scroll = if active_line_index > view_height / 2 {
        (active_line_index - view_height / 2) as u16
    } else {
        0
    };

    let p = Paragraph::new(spans)
        .block(Block::default().title(track.name.as_str()).borders(Borders::ALL))
        .scroll((scroll, 0));

    f.render_widget(p, area);
}
