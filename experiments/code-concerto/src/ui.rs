use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{BarChart, Block, Borders, Gauge, Paragraph},
    Frame, Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crate::mapping::MusicalEvent;

pub struct App {
    pub code_lines: Vec<String>,
    pub events: Vec<MusicalEvent>,
    pub start_time: Instant,
    pub is_playing: bool,
    pub current_event_index: usize,
    pub total_duration: Duration,
}

impl App {
    pub fn new(code: String, events: Vec<MusicalEvent>) -> Self {
        let total_secs: f32 = events
            .iter()
            .map(|e| match e {
                MusicalEvent::Note { duration, .. } => *duration,
                MusicalEvent::Chord { duration, .. } => *duration,
                MusicalEvent::Rest { duration } => *duration,
            })
            .sum();

        Self {
            code_lines: code.lines().map(|s| s.to_string()).collect(),
            events,
            start_time: Instant::now(), // Will be reset on play
            is_playing: false,
            current_event_index: 0,
            total_duration: Duration::from_secs_f32(total_secs),
        }
    }

    pub fn update(&mut self) {
        if !self.is_playing {
            return;
        }

        let elapsed = self.start_time.elapsed().as_secs_f32();

        // Find current event based on elapsed time
        let mut time_cursor = 0.0;
        let mut found = false;

        for (i, event) in self.events.iter().enumerate() {
            let duration = match event {
                MusicalEvent::Note { duration, .. } => *duration,
                MusicalEvent::Chord { duration, .. } => *duration,
                MusicalEvent::Rest { duration } => *duration,
            };

            if elapsed >= time_cursor && elapsed < time_cursor + duration {
                self.current_event_index = i;
                found = true;
                break;
            }
            time_cursor += duration;
        }

        if !found && elapsed >= time_cursor {
            // Finished
            self.is_playing = false;
            self.current_event_index = self.events.len().saturating_sub(1);
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    app.start_time = Instant::now();
    app.is_playing = true;

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        app.update();
        if !app.is_playing && app.start_time.elapsed() > app.total_duration + Duration::from_secs(1)
        {
            // Auto exit or loop? Let's just stop updating but keep running until 'q'
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // Left: Source Code
    // In a real implementation, we'd map events back to line numbers.
    // For now, we just scroll or show the code.
    // TODO: Add line tracking to MusicalEvent in mapping.rs

    let code_text: Text = app.code_lines.join("\n").into();
    let code_block = Paragraph::new(code_text)
        .block(Block::default().borders(Borders::ALL).title("Source Code"));
    f.render_widget(code_block, chunks[0]);

    // Right: Visualizer
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let event = &app.events[app.current_event_index];
    let (desc, color) = match event {
        MusicalEvent::Note { freq, .. } => (format!("Note: {:.1} Hz", freq), Color::Cyan),
        MusicalEvent::Chord { freqs, .. } => {
            (format!("Chord: {} tones", freqs.len()), Color::Magenta)
        }
        MusicalEvent::Rest { .. } => ("Rest".to_string(), Color::Gray),
    };

    let info_block = Paragraph::new(desc)
        .style(Style::default().fg(color))
        .block(Block::default().borders(Borders::ALL).title("Now Playing"));
    f.render_widget(info_block, right_chunks[0]);

    // Progress
    let elapsed = if app.is_playing {
        app.start_time.elapsed()
    } else {
        app.total_duration
    }; // Simplified
    let progress = (elapsed.as_secs_f32() / app.total_duration.as_secs_f32()).min(1.0);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::Yellow))
        .ratio(progress as f64);
    f.render_widget(gauge, right_chunks[1]);
}
