use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
};
use std::{collections::VecDeque, time::Duration};
use tui_shared::Tui;

use crossbeam_channel::unbounded;
use ndarray::Array1;

use crate::audio::{AudioEngine, AudioEvent};
use crate::network::Reservoir;

pub mod audio;
pub mod network;
pub mod neuron;

fn main() -> Result<()> {
    // Initialize TUI
    let mut tui = Tui::init()?;

    // Setup Channels
    let (audio_tx, audio_rx) = unbounded();

    // Initialize Audio
    let _audio = AudioEngine::new(audio_rx)?;

    // Initialize Network
    let size = 1000;
    // 10% connectivity density
    let mut net = Reservoir::new(size, 0.1);

    // Input buffer
    let mut input_current = Array1::zeros(size);

    let mut spike_history: VecDeque<(f64, f64)> = VecDeque::new();
    let max_history_len = 5000;
    let mut sim_time = 0.0;

    // Simulation loop
    loop {
        // Handle Input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    break;
                }
            }
        }

        // Run multiple steps per frame for speed (and audio density)
        // 60 FPS -> 16ms. If we simulate 10 steps of 0.5ms = 5ms sim per 16ms real. Slow motion.
        // To be real-time, we need 32 steps per frame (16ms / 0.5ms = 32).
        for _ in 0..30 {
            // Random thalamic input (noise)
            use rand::Rng;
            let mut rng = rand::thread_rng();

            // Apply noise to random neurons to keep activity going
            // Or a rhythmic pulse? Let's just do noise.
            for x in input_current.iter_mut() {
                if rng.gen::<f32>() < 0.005 {
                    *x = 20.0; // Strong kick
                } else {
                    *x = 0.0;
                }
            }

            let spikes = net.step(0.5, &input_current);
            sim_time += 0.5;

            for (i, &s) in spikes.iter().enumerate() {
                if s {
                    spike_history.push_back((sim_time, i as f64));
                    // Send to audio (fire and forget)
                    let _ = audio_tx.send(AudioEvent::Spike(i));
                }
            }
        }

        // Prune history
        while spike_history.len() > max_history_len {
            spike_history.pop_front();
        }
        // Prune old time
        while let Some(&(t, _)) = spike_history.front() {
            if t < sim_time - 200.0 {
                spike_history.pop_front();
            } else {
                break;
            }
        }

        // Render
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Beat Cortex Raster"),
                )
                .x_bounds([sim_time - 200.0, sim_time])
                .y_bounds([0.0, size as f64])
                .marker(ratatui::symbols::Marker::Block)
                .paint(|ctx| {
                    let points: Vec<(f64, f64)> = spike_history.iter().copied().collect();
                    ctx.draw(&Points {
                        coords: &points,
                        color: Color::Cyan,
                    });
                });

            f.render_widget(canvas, chunks[0]);

            let status_text = if cfg!(feature = "audio") {
                "ESC: Quit | Audio: ON"
            } else {
                "ESC: Quit | Audio: OFF"
            };

            f.render_widget(
                Paragraph::new(status_text).block(Block::default().borders(Borders::ALL)),
                chunks[1],
            );
        })?;

        // Aim for ~60 FPS
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
