use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{collections::VecDeque, time::Duration};

mod cochlea;
mod dsp;
mod neuron;
mod signal;
mod tui;

use signal::SAMPLE_RATE;

fn main() -> Result<()> {
    // TUI Setup
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let mut gen = signal::SignalGenerator::new();
    let num_channels = 32;
    let mut cochlea = cochlea::Cochlea::new(num_channels);

    // Buffers
    let max_history_duration = 2.0; // Seconds to keep in memory
    let max_history_samples = (max_history_duration * SAMPLE_RATE) as usize;

    let mut waveform_history: VecDeque<f32> = VecDeque::with_capacity(max_history_samples);
    let mut spike_history: VecDeque<(f64, usize)> = VecDeque::new();
    let mut potentials_history: Vec<f32> = vec![0.0; num_channels];

    let mut sim_time = 0.0;
    // Simulate ~60 FPS
    let chunk_duration = 0.016;
    let chunk_size = (chunk_duration * SAMPLE_RATE) as usize;

    // View Window
    let window_duration = 0.1; // Show last 100ms for fast scrolling

    loop {
        // Input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    return Ok(());
                }
            }
        }

        // Generate & Process
        let mut chunk = Vec::with_capacity(chunk_size);
        for _ in 0..chunk_size {
            if let Some(s) = gen.next() {
                chunk.push(s);
            }
        }

        let (spikes, potentials) = cochlea.process_chunk(&chunk);

        // Store history
        for &s in chunk.iter() {
            waveform_history.push_back(s);
            if waveform_history.len() > max_history_samples {
                waveform_history.pop_front();
            }
        }

        for (t_offset, ch) in spikes {
            let spike_time = sim_time + (t_offset as f64 / SAMPLE_RATE as f64);
            spike_history.push_back((spike_time, ch));
        }

        // Prune spikes
        while let Some(&(t, _)) = spike_history.front() {
            if t < sim_time - max_history_duration as f64 {
                spike_history.pop_front();
            } else {
                break;
            }
        }

        potentials_history = potentials;

        sim_time += chunk.len() as f64 / SAMPLE_RATE as f64;

        // Render
        terminal.draw(|f| {
            let end = sim_time;
            let start = end - window_duration;

            let samples_needed = (window_duration * SAMPLE_RATE as f64) as usize;
            let available = waveform_history.len();
            let start_idx = available.saturating_sub(samples_needed);

            let visible_waveform: Vec<f32> =
                waveform_history.iter().skip(start_idx).cloned().collect();

            let visible_spikes: Vec<(f64, usize)> = spike_history
                .iter()
                .filter(|&&(t, _)| t >= start && t <= end)
                .cloned()
                .collect();

            tui::draw(
                f,
                &visible_waveform,
                &visible_spikes,
                &potentials_history,
                num_channels,
                start,
                end,
            );
        })?;

        // Throttle
        std::thread::sleep(Duration::from_millis(10));
    }
}
