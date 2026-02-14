use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame, Terminal,
};
use std::io;

use crate::euclidean::EuclideanGenerator;
use crate::monitor::SystemMonitor;

pub struct AppTui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl AppTui {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn draw(
        &mut self,
        monitor: &SystemMonitor,
        generators: &[EuclideanGenerator],
        current_step: usize,
        bpm: f32,
    ) -> Result<()> {
        self.terminal.draw(|f| {
            ui(f, monitor, generators, current_step, bpm);
        })?;
        Ok(())
    }

    pub fn should_quit(&self) -> Result<bool> {
        if event::poll(std::time::Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

impl Drop for AppTui {
    fn drop(&mut self) {
        disable_raw_mode().unwrap_or(());
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap_or(());
        self.terminal.show_cursor().unwrap_or(());
    }
}

fn ui(
    f: &mut Frame,
    monitor: &SystemMonitor,
    generators: &[EuclideanGenerator],
    current_step: usize,
    bpm: f32,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Tracks
            Constraint::Length(1), // Footer
        ])
        .split(f.size());

    // Header
    let global_cpu = monitor.get_global_cpu_usage();
    let memory = monitor.get_memory_usage();

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(chunks[0]);

    f.render_widget(
        Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Global CPU"))
            .gauge_style(Style::default().fg(Color::Red))
            .ratio(global_cpu.clamp(0.0, 1.0) as f64),
        header_chunks[0],
    );

    f.render_widget(
        Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Memory"))
            .gauge_style(Style::default().fg(Color::Blue))
            .ratio(memory.clamp(0.0, 1.0) as f64),
        header_chunks[1],
    );

    f.render_widget(
        Paragraph::new(format!("{:.1}", bpm))
            .block(Block::default().borders(Borders::ALL).title("BPM"))
            .style(Style::default().fg(Color::Yellow)),
        header_chunks[2],
    );

    // Tracks
    let constraints: Vec<Constraint> = generators.iter().map(|_| Constraint::Length(3)).collect();

    let track_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(chunks[1]);

    for (i, gen) in generators.iter().enumerate() {
        if i >= track_chunks.len() {
            break;
        }

        let track_name = match i {
            0 => "KICK (Core 0)",
            1 => "SNARE (Core 1)",
            2 => "HIHAT (Core 2)",
            3 => "PERC 1 (Core 3)",
            4 => "PERC 2 (Core 4)",
            _ => "PERC",
        };

        let track_color = match i {
            0 => Color::Red,
            1 => Color::Blue,
            2 => Color::Yellow,
            3 => Color::Green,
            _ => Color::White,
        };

        // Visualize pattern
        let len = gen.pattern.len();
        let mut spans = Vec::new();

        if len > 0 {
            for (idx, &beat) in gen.pattern.iter().enumerate() {
                let is_current = idx == (current_step % len);
                let char_display = if beat { "█ " } else { "░ " };

                let style = if is_current {
                    Style::default().bg(Color::White).fg(Color::Black)
                } else if beat {
                    Style::default().fg(track_color)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                spans.push(Span::styled(char_display, style));
            }
        }

        let line = Line::from(spans);

        f.render_widget(
            Paragraph::new(vec![
                Line::from(format!("Steps: {}, Pulses: {}", gen.steps, gen.pulses)),
                line,
            ])
            .block(
                Block::default()
                    .borders(Borders::LEFT)
                    .title(track_name)
                    .border_style(Style::default().fg(track_color)),
            ),
            track_chunks[i],
        );
    }

    f.render_widget(Paragraph::new("Press 'q' or 'ESC' to quit"), chunks[2]);
}
