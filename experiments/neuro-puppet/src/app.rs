use crate::body::Creature;
use crate::network::{construct_cpg, Network};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::{Block, Borders, Gauge, Paragraph, Sparkline},
};
use std::io;
use std::time::{Duration, Instant};
use tui_shared::Tui;

pub struct App {
    network: Network,
    creature: Creature,
    drive: f64,
    history_v1: Vec<u64>,
    history_v2: Vec<u64>,
    steps: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            network: construct_cpg(),
            creature: Creature::new(),
            drive: 12.0, // Good starting value for Izhikevich to burst/spike
            history_v1: vec![0; 200],
            history_v2: vec![0; 200],
            steps: 0,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut tui = Tui::init()?;
        let tick_rate = Duration::from_millis(30);
        let mut last_tick = Instant::now();

        loop {
            tui.terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Up | KeyCode::Char('w') => self.drive += 1.0,
                        KeyCode::Down | KeyCode::Char('s') => self.drive -= 1.0,
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
        tui.exit()?;
        Ok(())
    }

    fn on_tick(&mut self) {
        // 5 simulation steps per frame
        for _ in 0..5 {
            let inputs = vec![self.drive, self.drive];
            let spikes = self.network.step(&inputs);
            self.creature.update(&spikes);

            // Map voltage (-80..40) to 0..120
            let v1 = (self.network.neurons[0].v + 80.0).max(0.0) as u64;
            let v2 = (self.network.neurons[1].v + 80.0).max(0.0) as u64;

            self.history_v1.push(v1);
            self.history_v2.push(v2);
            if self.history_v1.len() > 200 {
                self.history_v1.remove(0);
            }
            if self.history_v2.len() > 200 {
                self.history_v2.remove(0);
            }

            self.steps += 1;
        }
    }

    fn ui(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(frame.area());

        // Header
        let title = Paragraph::new(format!(
            "Neuro-Puppet (Genesis Moonshot) | Drive Current: {:.1} | Press 'w'/'s' to adjust, 'q' to quit",
            self.drive
        ))
        .block(Block::default().borders(Borders::ALL).title("Control Panel"));
        frame.render_widget(title, chunks[0]);

        // Creature Visualization
        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let muscle_l_val = (self.creature.muscle_l / 10.0).clamp(0.0, 1.0);
        let muscle_r_val = (self.creature.muscle_r / 10.0).clamp(0.0, 1.0);

        let gauge_l = Gauge::default()
            .block(Block::default().title("Left Muscle").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Red))
            .ratio(muscle_l_val);
        frame.render_widget(gauge_l, body_chunks[0]);

        let gauge_r = Gauge::default()
            .block(Block::default().title("Right Muscle").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Blue))
            .ratio(muscle_r_val);
        frame.render_widget(gauge_r, body_chunks[1]);

        // Neural Activity
        let neural_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[2]);

        let sparkline_1 = Sparkline::default()
            .block(
                Block::default()
                    .title("Neuron 1 (Left Drive)")
                    .borders(Borders::ALL),
            )
            .data(&self.history_v1)
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(sparkline_1, neural_chunks[0]);

        let sparkline_2 = Sparkline::default()
            .block(
                Block::default()
                    .title("Neuron 2 (Right Drive)")
                    .borders(Borders::ALL),
            )
            .data(&self.history_v2)
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(sparkline_2, neural_chunks[1]);
    }
}
