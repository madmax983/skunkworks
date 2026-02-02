use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color},
    widgets::{canvas::{Canvas, Points}, Block, Borders, Paragraph},
};
use std::{
    collections::VecDeque,
    time::Duration,
};
use tui_shared::Tui;

pub mod audio;
pub mod network;
pub mod neuron;

use network::Network;
use neuron::Neuron;
#[cfg(feature = "audio")]
use audio::AudioEvent;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    #[cfg(feature = "audio")]
    let _audio_handle = match audio::engine::init() {
        Ok(h) => Some(h),
        Err(_e) => {
            // In a real app we might show this error in the UI
            None
        }
    };

    #[cfg(feature = "audio")]
    let sender = if let Some(ref h) = _audio_handle {
        Some(h.sender.clone())
    } else {
        None
    };

    // Setup Network
    let mut net = Network::new();

    // Create 4 neurons
    for _ in 0..4 {
        net.add_neuron(Neuron::regular_spiking());
    }

    // Connect in a ring: 0->1->2->3->0
    net.add_synapse(0, 1, 50.0); // Strong synapses to ensure propagation
    net.add_synapse(1, 2, 50.0);
    net.add_synapse(2, 3, 50.0);
    net.add_synapse(3, 0, 50.0);

    let mut spike_history: VecDeque<(f64, f64)> = VecDeque::new();
    let max_history = 1000;
    let mut time = 0.0;

    loop {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    break;
                }
            }
        }

        // Run simulation
        // 10ms per frame to speed things up? Or just 1ms?
        // Let's run 5 steps per frame (approx 5ms simulation per 16ms real time)
        // This makes it slower than real time but visible.
        // Actually, if we want audio to sound rhythmic, we need faster simulation.
        // If we run 1 step per frame (16ms), 1 second sim = 16 seconds real. Too slow.
        // We want 1ms sim = 1ms real? Then we need to run ~16 steps per frame.

        for _ in 0..10 {
            // Input to N0 to act as pacemaker
            let currents = vec![15.0, 0.0, 0.0, 0.0];

            let spikes = net.update(&currents, 1.0);

            for (i, &spiked) in spikes.iter().enumerate() {
                if spiked {
                    spike_history.push_back((time, i as f64));
                    #[cfg(feature = "audio")]
                    if let Some(s) = &sender {
                        let _ = s.send(AudioEvent::Spike(i));
                    }
                }
            }
            time += 1.0;
        }

        while spike_history.len() > max_history {
            spike_history.pop_front();
        }

        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(f.area());

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Spike Raster (Genesis Neuro-Beat)"))
                .x_bounds([time - 200.0, time]) // Show last 200ms
                .y_bounds([-0.5, 3.5]) // 4 neurons
                .marker(ratatui::symbols::Marker::Block)
                .paint(|ctx| {
                    // Convert deque to vec for drawing
                    // Points expects slice of tuples
                    let points: Vec<(f64, f64)> = spike_history.iter().copied().collect();
                    ctx.draw(&Points {
                        coords: &points,
                        color: Color::Green,
                    });
                });

            f.render_widget(canvas, chunks[0]);

            let info = Paragraph::new("Press ESC to Quit. Run with --features audio to hear it.")
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(info, chunks[1]);
        })?;
    }

    Ok(())
}
