mod audio;
mod engine;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{
        canvas::{Canvas, Circle, Context},
        Block, Borders, Gauge, Paragraph,
    },
    Frame,
};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tui_shared::Tui;

use crate::audio::{AudioEngine, AudioEvent};
use crate::engine::{Engine, DrummerState};

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // Initialize Audio
    let audio = AudioEngine::new()?;

    // Initialize Engine
    let mut engine = Engine::new(&audio);

    // Spawn Drummers (Polymeters)
    // 4/4 Base (Pulse)
    engine.spawn_drummer("Base (4/4)", Duration::from_millis(500), AudioEvent::Kick, Color::Blue);

    // 5 against 4: Period = 500 * (4/5) = 400ms
    engine.spawn_drummer("Poly (5:4)", Duration::from_millis(400), AudioEvent::Snare, Color::Red);

    // 3 against 4: Period = 500 * (4/3) = 666ms
    engine.spawn_drummer("Poly (3:4)", Duration::from_millis(666), AudioEvent::HiHat, Color::Green);

    // Fast Pulse
    engine.spawn_drummer("Pulse", Duration::from_millis(125), AudioEvent::Pulse(440.0), Color::Yellow);

    let mut drift_amount = 0.0;
    let mut show_help = true;

    loop {
        tui.terminal.draw(|f| {
            ui(f, &engine.states, drift_amount, show_help);
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('d') => {
                        drift_amount += 0.01;
                        engine.set_global_drift(drift_amount);
                    }
                    KeyCode::Char('a') => {
                        drift_amount -= 0.01;
                        if drift_amount < 0.0 { drift_amount = 0.0; }
                        engine.set_global_drift(drift_amount);
                    }
                    KeyCode::Char('r') => {
                        drift_amount = 0.0;
                        engine.reset_drift();
                    }
                    KeyCode::Char('h') => {
                        show_help = !show_help;
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, states: &Vec<Arc<Mutex<DrummerState>>>, drift: f32, show_help: bool) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Main Content
            Constraint::Length(3), // Status
        ])
        .split(f.area());

    let title = Paragraph::new("⚛️ THREAD-PHASE: Polymetric Synchronization Sonification")
        .style(Style::default().fg(Color::Cyan).bold())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main Content: Split into Linear View and Phase View
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Left: Linear Gauges
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            states.iter().map(|_| Constraint::Length(3)).collect::<Vec<_>>()
        )
        .split(main_chunks[0]);

    for (i, state_arc) in states.iter().enumerate() {
        if i >= left_chunks.len() { break; }

        let state = state_arc.lock().unwrap();
        let elapsed = state.last_hit.elapsed();
        let period_secs = state.period.as_secs_f32() * state.drift;
        let ratio = (elapsed.as_secs_f32() / period_secs).min(1.0);

        let label = format!("{} [{:.2}x]", state.name, state.drift);
        let gauge = Gauge::default()
            .block(Block::default().title(label).borders(Borders::ALL))
            .gauge_style(Style::default().fg(state.color))
            .ratio(ratio as f64);

        f.render_widget(gauge, left_chunks[i]);
    }

    // Right: Phase Lissajous (Drummer 0 vs Drummer 1)
    if states.len() >= 2 {
        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Phase Interference (0 vs 1)"))
            .x_bounds([0.0, 1.0])
            .y_bounds([0.0, 1.0])
            .paint(|ctx: &mut Context| {
                let s0 = states[0].lock().unwrap();
                let s1 = states[1].lock().unwrap();

                let p0 = (s0.last_hit.elapsed().as_secs_f32() / (s0.period.as_secs_f32() * s0.drift)) % 1.0;
                let p1 = (s1.last_hit.elapsed().as_secs_f32() / (s1.period.as_secs_f32() * s1.drift)) % 1.0;

                // Draw current point
                ctx.draw(&Circle {
                    x: p0 as f64,
                    y: p1 as f64,
                    radius: 0.05,
                    color: Color::White,
                });
            });
        f.render_widget(canvas, main_chunks[1]);
    }

    // Status / Help
    let help_text = if show_help {
        "Q: Quit | D: +Drift | A: -Drift | R: Reset Drift | H: Toggle Help"
    } else {
        "Press H for Help"
    };

    let status = Paragraph::new(format!("Drift: {:.2} | {}", drift, help_text))
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}
