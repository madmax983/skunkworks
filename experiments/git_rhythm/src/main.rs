use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Gauge, Paragraph, Sparkline},
    Terminal,
};
use std::io::{self};
use std::time::{Duration, Instant};

use git_rhythm::harvester::harvest_repo;
use git_rhythm::synth::Synthesizer;
use git_rhythm::vis::VisualState;

fn main() -> Result<()> {
    // 1. Harvest
    let commits = harvest_repo(".")?;
    if commits.is_empty() {
        println!("No commits found!");
        return Ok(());
    }

    // 2. Setup Audio Writer
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut wav_writer = hound::WavWriter::create("skunkworks_symphony.wav", spec)?;

    // 3. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 4. State
    let mut synth = Synthesizer::new(44100);
    let mut vis = VisualState::new();

    // Playback state
    let mut current_commit_idx = 0;
    let samples_per_commit = 22050; // 0.5 seconds per commit
    let mut samples_played_for_commit = 0;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16); // ~60 FPS

    loop {
        // Handle Input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Logic
        let commit = &commits[current_commit_idx];

        // Generate audio for this frame (simulated buffer size)
        // Let's generate 1/60th of a second worth of samples
        let samples_per_frame = 44100 / 60;

        for _ in 0..samples_per_frame {
            let sample = synth.generate_next_sample(commit);

            // Convert f32 (-1.0 to 1.0) to i16
            let sample_i16 = (sample * i16::MAX as f32) as i16;
            wav_writer.write_sample(sample_i16)?;

            // Update vis state (maybe not every sample to save CPU, but for now it's fine)
            vis.update(commit.clone(), sample);

            samples_played_for_commit += 1;
            if samples_played_for_commit >= samples_per_commit {
                samples_played_for_commit = 0;
                current_commit_idx = (current_commit_idx + 1) % commits.len();
                // If we wrapped around, we just keep going
            }
        }

        // Draw
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(f.area());

            let info_text = format!(
                "Commit: {}\nAuthor: {}\nTime: {}\nPress 'q' to quit",
                commit.hash, commit.author, commit.timestamp
            );
            let info = Paragraph::new(info_text)
                .block(Block::default().title("Meta").borders(Borders::ALL));
            f.render_widget(info, chunks[0]);

            // Waveform (Sparkline needs u64, so we map f32 to u64)
            // Sparkline expects a slice of u64
            let data: Vec<u64> = vis
                .waveform_buffer
                .iter()
                .map(|&s| ((s + 1.0) * 50.0) as u64) // Map -1..1 to 0..100
                .collect();

            let sparkline = Sparkline::default()
                .block(Block::default().title("Waveform").borders(Borders::ALL))
                .data(&data)
                .max(100); // 100 height
            f.render_widget(sparkline, chunks[1]);

            // Churn Gauge
            // Normalize churn to percentage (cap at 2000 lines)
            let churn_ratio = (commit.churn as f64 / 2000.0).min(1.0);
            let gauge = Gauge::default()
                .block(Block::default().title("Flux (Churn)").borders(Borders::ALL))
                .gauge_style(ratatui::style::Style::default().fg(ratatui::style::Color::Red))
                .ratio(churn_ratio)
                .label(format!("{} lines", commit.churn));
            f.render_widget(gauge, chunks[2]);
        })?;

        // Limit FPS
        if last_tick.elapsed() < tick_rate {
            std::thread::sleep(tick_rate - last_tick.elapsed());
        }
        last_tick = Instant::now();
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    println!("Done! Written to skunkworks_symphony.wav");

    Ok(())
}
