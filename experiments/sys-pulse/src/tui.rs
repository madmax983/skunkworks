use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment},
    widgets::{Block, Borders, Gauge, Sparkline, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::sync::{Arc, RwLock};
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use crate::monitor::SystemStats;
use crate::rhythm::Euclidean;

pub struct App {
    stats: Arc<RwLock<SystemStats>>,
    cpu_history: VecDeque<u64>,
}

impl App {
    pub fn new(stats: Arc<RwLock<SystemStats>>) -> Self {
        Self {
            stats,
            cpu_history: VecDeque::from(vec![0; 100]),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        // Setup Terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run_app(&mut terminal);

        // Restore Terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        res
    }

    fn run_app(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> anyhow::Result<()> {
        let tick_rate = Duration::from_millis(100);
        let mut last_tick = Instant::now();

        loop {
            // Update History
            {
                // Use try_read to prevent freezing UI if monitor locks
                if let Ok(lock) = self.stats.try_read() {
                    if self.cpu_history.len() >= 100 {
                        self.cpu_history.pop_front();
                    }
                    self.cpu_history.push_back(lock.cpu_usage as u64);
                }
            }

            terminal.draw(|f| {
                // Layout
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(30),
                            Constraint::Percentage(30),
                            Constraint::Percentage(40),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                // Sparkline
                let data: Vec<u64> = self.cpu_history.iter().copied().collect();
                let sparkline = Sparkline::default()
                    .block(Block::default().title("CPU Heartbeat").borders(Borders::ALL))
                    .data(&data)
                    .style(ratatui::style::Style::default().fg(ratatui::style::Color::Red));
                f.render_widget(sparkline, chunks[0]);

                // Gauge (Memory)
                let mem = {
                    // We can block here briefly or use cached value.
                    // Let's use read() as we are the main thread.
                    if let Ok(lock) = self.stats.read() {
                        (lock.memory_usage * 100.0).min(100.0) as u16
                    } else {
                        0
                    }
                };
                let gauge = Gauge::default()
                    .block(Block::default().title("Memory Pressure").borders(Borders::ALL))
                    .gauge_style(ratatui::style::Style::default().fg(ratatui::style::Color::Blue))
                    .percent(mem);
                f.render_widget(gauge, chunks[1]);

                // Rhythm Pattern Visualizer
                let cpu = self.cpu_history.back().copied().unwrap_or(0) as f32;
                let k = ((cpu / 100.0) * 16.0).round().max(1.0) as usize;
                let k = k.min(16);
                let pattern = Euclidean::generate(k, 16);
                let pat_str: String = pattern.iter().map(|&b| if b { "█ " } else { "· " }).collect();

                let p = Paragraph::new(format!("Pattern E({}, 16):\n\n{}", k, pat_str))
                     .block(Block::default().title("Euclidean Matrix").borders(Borders::ALL))
                     .alignment(Alignment::Center);
                f.render_widget(p, chunks[2]);

            })?;

            // Events
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }
}
