use std::io;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};

use crate::shared::GrooveState;

pub struct AppTui {
    state: Arc<GrooveState>,
}

impl AppTui {
    pub fn new(state: Arc<GrooveState>) -> Self {
        Self { state }
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run_app(&mut terminal);

        disable_raw_mode()?;
        io::stdout().execute(LeaveAlternateScreen)?;

        if let Err(err) = res {
            println!("{:?}", err);
        }

        Ok(())
    }

    fn run_app(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Min(0),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());

                let kick_latency = self.state.get_kick_latency().as_millis() as u64;
                let snare_latency = self.state.get_snare_latency().as_millis() as u64;
                let hat_latency = self.state.get_hat_latency().as_millis() as u64;
                let total_beats = self.state.get_total_beats();

                // Title
                let title = Paragraph::new(format!("NET-GROOVE: Network Latency Rhythms | Beats: {}", total_beats))
                    .block(Block::default().borders(Borders::ALL).title("Status"));
                f.render_widget(title, chunks[0]);

                // Kick Gauge
                let kick_gauge = Gauge::default()
                    .block(Block::default().title("Kick (8.8.8.8) - Swing").borders(Borders::ALL))
                    .gauge_style(Style::default().fg(Color::Red))
                    .percent(((kick_latency as f64 / 500.0) * 100.0).clamp(0.0, 100.0) as u16)
                    .label(format!("{} ms", kick_latency));
                f.render_widget(kick_gauge, chunks[1]);

                // Snare Gauge
                let snare_gauge = Gauge::default()
                    .block(Block::default().title("Snare (1.1.1.1) - Swing").borders(Borders::ALL))
                    .gauge_style(Style::default().fg(Color::Green))
                    .percent(((snare_latency as f64 / 500.0) * 100.0).clamp(0.0, 100.0) as u16)
                    .label(format!("{} ms", snare_latency));
                f.render_widget(snare_gauge, chunks[2]);

                // Hat Gauge
                let hat_gauge = Gauge::default()
                    .block(Block::default().title("Hat (example.com) - Swing").borders(Borders::ALL))
                    .gauge_style(Style::default().fg(Color::Blue))
                    .percent(((hat_latency as f64 / 500.0) * 100.0).clamp(0.0, 100.0) as u16)
                    .label(format!("{} ms", hat_latency));
                f.render_widget(hat_gauge, chunks[3]);

                // Info
                let info = Paragraph::new("Press 'q' to quit.\nLatency determines the 'Swing' offset.\nHigher latency = More drag.\nTimeout (500ms) = Dropped/Ghost note feel.")
                    .block(Block::default().borders(Borders::ALL).title("Info"));
                f.render_widget(info, chunks[4]);

            })?;

            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                }
            }
        }
    }
}
