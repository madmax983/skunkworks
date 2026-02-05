mod audio;
mod paint;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};
use std::{
    io,
    time::{Duration, Instant},
};

use audio::{SignalGenerator, SpectralAnalyzer};
use paint::Canvas;

fn main() -> Result<()> {
    // Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App State
    let width = 80;
    let height = 40;
    let mut canvas = Canvas::new(width, height);
    let sample_rate = 44100;
    let fft_size = 1024; // Resolution of freq bands
    let mut generator = SignalGenerator::new(sample_rate);
    let mut analyzer = SpectralAnalyzer::new(fft_size);

    // Buffers for Zero-Allocation
    let mut audio_buffer = vec![0.0; 1024];
    let mut spectrum_buffer = vec![0.0; 512]; // Nyquist

    // Loop
    let tick_rate = Duration::from_millis(33); // ~30 FPS
    let mut last_tick = Instant::now();
    let mut frame_count = 0;

    let res = run_app(
        &mut terminal,
        &mut canvas,
        &mut generator,
        &mut analyzer,
        tick_rate,
        &mut last_tick,
        &mut frame_count,
        &mut audio_buffer,
        &mut spectrum_buffer,
    );

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    canvas: &mut Canvas,
    generator: &mut SignalGenerator,
    analyzer: &mut SpectralAnalyzer,
    tick_rate: Duration,
    last_tick: &mut Instant,
    frame_count: &mut u64,
    audio_buffer: &mut [f32],
    spectrum_buffer: &mut [f32],
) -> Result<()> {
    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // Input
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }

        // Update
        if last_tick.elapsed() >= tick_rate {
            *last_tick = Instant::now();
            *frame_count += 1;

            // 1. Generate Audio & Analyze
            generator.generate_chunk(audio_buffer);
            analyzer.compute_spectrum(audio_buffer, spectrum_buffer);

            // 2. Map Spectrum to Canvas
            // Spectrum size = 512 (Nyquist).
            // We map this to Canvas Height (40).
            // Low frequencies (idx 0) -> Bottom (y = height - 1)
            // High frequencies (idx 512) -> Top (y = 0)

            let buckets = canvas.height;
            let bins_per_bucket = spectrum_buffer.len() / buckets;

            for y_bucket in 0..buckets {
                let start_bin = y_bucket * bins_per_bucket;
                let end_bin = start_bin + bins_per_bucket;

                // Calculate average magnitude in this band
                let mut sum = 0.0;
                for i in start_bin..end_bin {
                    if i < spectrum_buffer.len() {
                        sum += spectrum_buffer[i];
                    }
                }
                let avg = sum / bins_per_bucket as f32;

                // Amplify for visual effect
                let intensity = avg * 0.1;

                // Threshold to paint
                if intensity > 0.01 {
                    // Map bucket 0 (Low Freq) to Bottom (height - 1)
                    // Map bucket N (High Freq) to Top (0)
                    // But usually FFT[0] is DC/Low.
                    // So we want:
                    // bucket 0 -> canvas.height - 1
                    // bucket N -> 0
                    let canvas_y = canvas.height - 1 - y_bucket;

                    let color = map_frequency_to_color(y_bucket, buckets);
                    canvas.apply_brush(canvas_y, intensity, color);
                }
            }

            // 3. Physics
            canvas.tick_physics();

            // 4. Draw
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(0), Constraint::Length(1)])
                    .split(f.area());

                // Render Canvas
                // We use a Paragraph with styled spaces because direct Buffer access is verbose,
                // but actually direct buffer access via `f.buffer_mut()` isn't exposed easily in closure.
                // We construct Lines.

                let mut lines = Vec::new();
                for y in 0..canvas.height {
                    let mut spans = Vec::new();
                    for x in 0..canvas.width {
                        let p = canvas.get(x, y);
                        // Using "  " (double space) for aspect ratio correction?
                        // Or just " "
                        spans.push(ratatui::text::Span::styled(
                            " ",
                            Style::default().bg(Color::Rgb(p.r, p.g, p.b)),
                        ));
                    }
                    lines.push(Line::from(spans));
                }

                let canvas_widget = Paragraph::new(lines)
                    .block(Block::default().borders(Borders::ALL).title("Spectral Canvas"));

                f.render_widget(canvas_widget, chunks[0]);

                let info = Paragraph::new(format!("Frame: {} | 44.1kHz | 1024 FFT", frame_count));
                f.render_widget(info, chunks[1]);
            })?;
        }
    }
}

fn map_frequency_to_color(bucket: usize, total_buckets: usize) -> (u8, u8, u8) {
    let t = bucket as f32 / total_buckets as f32;
    // t=0 (Low Freq) -> Deep Purple/Blue
    // t=0.5 (Mid) -> Red/Orange
    // t=1.0 (High) -> Yellow/White

    if t < 0.3 {
        // Deep Blue to Blue
        (20, 0, (100.0 + t * 500.0).min(255.0) as u8)
    } else if t < 0.6 {
        // Blue to Red
        let norm = (t - 0.3) / 0.3;
        ((norm * 255.0) as u8, 0, ((1.0 - norm) * 255.0) as u8)
    } else {
        // Red to Yellow/White
        let norm = (t - 0.6) / 0.4;
        (255, (norm * 255.0) as u8, (norm * 200.0) as u8)
    }
}
