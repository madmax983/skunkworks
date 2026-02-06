pub mod audio;
pub mod game;
pub mod neuron;
pub mod physics;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use crate::audio::AudioEngine;
use crate::game::GameState;
use crate::physics::PacketKind;

struct App {
    game: GameState,
    audio: AudioEngine,
    running: bool,
}

impl App {
    fn new(width: f64, height: f64) -> Result<Self> {
        let game = GameState::new(width, height);
        let audio = AudioEngine::new(game.pins.len())?;

        Ok(Self {
            game,
            audio,
            running: true,
        })
    }

    fn on_tick(&mut self) {
        let dt = 0.016;
        self.game.tick(dt, &self.audio.hit_tx);

        while let Some(snap) = self.audio.snapshot_rx.pop() {
            self.game.update_voltages(snap);
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    let logical_width = 100.0;
    let logical_height = 100.0;

    // Attempt to init app. If audio fails, we still want to restore terminal
    let res = App::new(logical_width, logical_height);

    if let Err(e) = res {
        // Tui drop will restore terminal, but let's be safe
        drop(tui);
        eprintln!("Failed to initialize app (Audio device missing?): {}", e);
        return Err(e);
    }
    let mut app = res.unwrap();

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => app.running = false,
                            KeyCode::Char(' ') => app.game.spawn_packet(),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if !app.running {
            break;
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Synaptic Pachinko "),
        )
        .marker(ratatui::symbols::Marker::Braille)
        .x_bounds([0.0, app.game.width])
        .y_bounds([0.0, app.game.height])
        .paint(|ctx| {
            // Draw Neurons (Pins)
            for (i, pin) in app.game.pins.iter().enumerate() {
                let v = if i < app.game.neuron_voltages.len() {
                    app.game.neuron_voltages[i]
                } else {
                    -65.0
                };

                let color = if v < -60.0 {
                    Color::DarkGray
                } else if v < -40.0 {
                    Color::Cyan
                } else if v < 0.0 {
                    Color::Yellow
                } else {
                    Color::Red
                };

                let color = if v > 20.0 { Color::White } else { color };

                let render_y = app.game.height - pin.pos.y;
                ctx.print(
                    pin.pos.x,
                    render_y,
                    Span::styled("O", Style::default().fg(color)),
                );
            }

            // Draw Particles
            for p in &app.game.particles {
                let color = match p.kind {
                    PacketKind::Malware => Color::Red,
                    PacketKind::Ssh => Color::Blue,
                    PacketKind::Http => Color::Green,
                };

                let render_y = app.game.height - p.pos.y;
                ctx.draw(&Points {
                    coords: &[(p.pos.x, render_y)],
                    color,
                });
            }
        });

    f.render_widget(canvas, chunks[0]);

    let status = vec![Line::from(vec![
        Span::raw("Score: "),
        Span::styled(
            format!("{}", app.game.score),
            Style::default().fg(Color::Cyan).bold(),
        ),
        Span::raw(" | Mean Field: "),
        Span::styled(
            format!("{:.2}mV", app.game.mean_field),
            Style::default().fg(Color::Yellow),
        ),
        Span::raw(" | Space: Drop Packet | Q: Quit"),
    ])];

    f.render_widget(
        Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}
