use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

pub mod game;
pub mod physics;

use crate::game::GameState;
use crate::physics::PacketKind;

struct App {
    game: GameState,
    running: bool,
}

impl App {
    fn new(width: f64, height: f64) -> Result<Self> {
        let game = GameState::new(width, height);
        Ok(Self {
            game,
            running: true,
        })
    }

    fn on_tick(&mut self) {
        let dt = 0.016;
        self.game.tick(dt);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    let logical_width = 100.0;
    let logical_height = 100.0;

    let mut app = App::new(logical_width, logical_height)?;

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
                .title(" Chimera Pachinko 🧬 "),
        )
        .marker(ratatui::symbols::Marker::Braille)
        .x_bounds([0.0, app.game.width])
        .y_bounds([0.0, app.game.height])
        .paint(|ctx| {
            // Draw Pins (Chimera Agents)
            for pin in &app.game.pins {
                let energy = pin.vm.energy;

                let color = if energy < 20 {
                    Color::DarkGray
                } else if energy < 50 {
                    Color::Cyan
                } else if energy < 100 {
                    Color::Green
                } else {
                    Color::Yellow
                };

                let symbol = if pin.generation > 0 { "M" } else { "O" };

                let render_y = app.game.height - pin.pos.y;
                ctx.print(
                    pin.pos.x,
                    render_y,
                    Span::styled(symbol, Style::default().fg(color)),
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
        Span::raw(" | Max Gen: "),
        Span::styled(
            format!(
                "{}",
                app.game
                    .pins
                    .iter()
                    .map(|p| p.generation)
                    .max()
                    .unwrap_or(0)
            ),
            Style::default().fg(Color::Yellow),
        ),
        Span::raw(" | Space: Drop Packet | Q: Quit"),
    ])];

    f.render_widget(
        Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}
