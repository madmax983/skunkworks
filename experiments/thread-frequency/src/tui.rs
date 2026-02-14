use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use std::sync::{Arc, atomic::Ordering};
use crate::musician::Beacon;
use crate::audio::Voice;

pub struct TuiState {
    pub musician_names: Vec<String>,
    pub musician_periods: Vec<u64>,
    pub musician_voices: Vec<Voice>,
    pub beacons: Vec<Arc<Beacon>>,
    pub current_time: Arc<std::sync::atomic::AtomicU64>,
    pub sample_rate: u32,
}

pub fn draw_ui(f: &mut Frame, state: &TuiState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Musicians
                Constraint::Length(3), // Footer
            ]
            .as_ref(),
        )
        .split(f.area());

    let now = state.current_time.load(Ordering::Relaxed);
    let time_sec = now as f64 / state.sample_rate as f64;

    let header = Paragraph::new(format!("THREAD FREQUENCY :: Time: {:.2}s :: Sample: {}", time_sec, now))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let musician_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            state.musician_names.iter().map(|_| Constraint::Length(3)).collect::<Vec<_>>()
        )
        .split(chunks[1]);

    for (i, name) in state.musician_names.iter().enumerate() {
        if i >= musician_chunks.len() { break; }

        let beacon = &state.beacons[i];
        let last_beat = beacon.last_beat.load(Ordering::Relaxed);
        let is_clash = beacon.is_clash.load(Ordering::Relaxed);

        // Calculate "freshness" of the beat
        let time_diff = if now >= last_beat { now - last_beat } else { last_beat - now }; // Absolute diff

        let time_diff_sec = time_diff as f64 / state.sample_rate as f64;

        // Flash window: 0.1s
        let intensity = if time_diff_sec < 0.1 {
            1.0 - (time_diff_sec * 10.0)
        } else {
            0.0
        };

        let color = if is_clash {
            Color::Red
        } else {
            Color::Green
        };

        let style = if intensity > 0.0 {
            Style::default().fg(color).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };

        let label = format!("{} ({}ms) - {:?}", name, state.musician_periods[i], state.musician_voices[i]);
        let gauge_value = (intensity * 100.0) as u16;
        let gauge = Gauge::default()
            .block(Block::default().title(label).borders(Borders::ALL))
            .gauge_style(style)
            .percent(gauge_value);

        f.render_widget(gauge, musician_chunks[i]);
    }

    let footer = Paragraph::new("Press 'q' to quit. M/Space to mutate? (Not implemented)")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::TOP));
    f.render_widget(footer, chunks[2]);
}
