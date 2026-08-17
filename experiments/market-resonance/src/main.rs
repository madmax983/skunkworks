//! # market-resonance
//!
//! A hybrid experiment crossing `market-sim` (Continuous Double Auction particle system) and `resonance-audio` (FDTD wave simulation).
//!
//! ## Concept: Acoustic Market Volatility
//! In standard `market-sim`, bid and ask particles interact to form trades. In this hybrid, every transaction acts as an acoustic impulse that strikes a continuous finite difference time domain (FDTD) wave grid provided by `resonance-audio`. The market acts as a reactor where high trading volume in specific price bands physically excites the acoustic space, creating standing waves and ripples of liquidity and resistance.
//!
//! ## Lineage
//! - **Parent A (market-sim)**: Provides the discrete bid/ask particle dynamics, collision logic, and price discovery.
//! - **Parent B (resonance-audio)**: Provides the continuous 2D FDTD simulation and audio processing pipeline.
//!
//! ## Novel Trait
//! Mapping discrete market trades directly to acoustic wave propagation.
//!
//! ## Setup & Running
//! ```bash
//! cargo run -p market-resonance
//! cargo run -p market-resonance -- --headless
//! ```
//!
use std::env;
use std::io::{self};
use std::thread;
use std::time::{Duration, Instant};

use crossbeam_channel::{bounded, Receiver, Sender};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use market_sim::{Grid, Particle};
use rand::Rng;
use resonance_audio::{AudioCommand, AudioModel, AudioSnapshot};

const TICK_RATE: Duration = Duration::from_millis(50);
const MARKET_WIDTH: usize = 60;
const MARKET_HEIGHT: usize = 30;

struct App {
    market: Grid,
    cmd_tx: Sender<AudioCommand>,
    snap_rx: Receiver<AudioSnapshot>,
    latest_snapshot: Option<AudioSnapshot>,
    running: bool,
    ticks: u64,
}

impl App {
    fn new(cmd_tx: Sender<AudioCommand>, snap_rx: Receiver<AudioSnapshot>) -> Self {
        Self {
            market: Grid::new(MARKET_WIDTH, MARKET_HEIGHT),
            cmd_tx,
            snap_rx,
            latest_snapshot: None,
            running: true,
            ticks: 0,
        }
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // Inject new orders occasionally
        if rng.gen_bool(0.4) {
            let x = rng.gen_range(0..MARKET_WIDTH);
            self.market.set(x, MARKET_HEIGHT - 1, Particle::Bid(1)); // Buyer at bottom
        }
        if rng.gen_bool(0.4) {
            let x = rng.gen_range(0..MARKET_WIDTH);
            self.market.set(x, 0, Particle::Ask(2)); // Seller at top
        }

        // Run market simulation
        let _trades = self.market.update();

        // Find trade coordinates and trigger acoustic plucks
        for y in 0..MARKET_HEIGHT {
            for x in 0..MARKET_WIDTH {
                if let Particle::Trade { .. } = self.market.get(x, y) {
                    // Send an acoustic pluck at this coordinate
                    let _ = self.cmd_tx.send(AudioCommand::Pluck {
                        x,
                        y,
                        strength: 0.8,
                    });
                }
            }
        }

        // Receive the latest snapshot
        while let Ok(snap) = self.snap_rx.try_recv() {
            self.latest_snapshot = Some(snap);
        }

        // Ask for a snapshot periodically
        if self.ticks.is_multiple_of(2) {
            // No RequestSnapshot in resonance-audio. Snapshots are sent automatically by the audio thread based on its sample counter (every 735 samples).
        }

        self.ticks += 1;
    }

    fn render(&self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default()
                .title(" Acoustic Market Volatility 📉🔊 ")
                .borders(Borders::ALL);

            let inner_area = block.inner(size);
            f.render_widget(block, size);

            let mut lines = Vec::new();

            let pressure_map = self.latest_snapshot.as_ref().map(|s| &s.pressure);

            for y in 0..MARKET_HEIGHT.min(inner_area.height as usize) {
                let mut spans = Vec::new();
                for x in 0..MARKET_WIDTH.min(inner_area.width as usize) {
                    let particle = self.market.get(x, y);

                    let mut p = 0.0;
                    if let Some(map) = pressure_map {
                        if y < MARKET_HEIGHT && x < MARKET_WIDTH {
                            p = map[y * MARKET_WIDTH + x];
                        }
                    }

                    // Visualize wave pressure in the background
                    let bg_color = if p > 0.5 {
                        Color::LightBlue
                    } else if p > 0.1 {
                        Color::Blue
                    } else if p < -0.5 {
                        Color::Red
                    } else if p < -0.1 {
                        Color::LightRed
                    } else {
                        Color::Reset
                    };

                    let (ch, fg_color) = match particle {
                        Particle::Bid(_) => ("B", Color::Green),
                        Particle::Ask(_) => ("A", Color::Yellow),
                        Particle::Trade { .. } => ("X", Color::White),
                        Particle::Empty => (" ", Color::Reset),
                        Particle::Wall => ("#", Color::DarkGray),
                    };

                    spans.push(Span::styled(ch, Style::default().fg(fg_color).bg(bg_color)));
                }
                lines.push(Line::from(spans));
            }

            let paragraph = Paragraph::new(lines);
            f.render_widget(paragraph, inner_area);
        })?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(1);

    // Create and spawn AudioModel in a background thread
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

    let model = AudioModel::new(MARKET_WIDTH, MARKET_HEIGHT, cmd_rx, snap_tx, None);

    let host = cpal::default_host();
    let _stream = if let Some(device) = host.default_output_device() {
        if let Ok(config) = device.default_output_config() {
            let model_lock = std::sync::Arc::new(std::sync::Mutex::new(model));
            let model_clone = model_lock.clone();

            let stream = match config.sample_format() {
                cpal::SampleFormat::F32 => device.build_output_stream(
                    &config.into(),
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        if let Ok(mut model) = model_clone.lock() {
                            model.process(data);
                        }
                    },
                    |err| eprintln!("an error occurred on stream: {}", err),
                    None,
                ),
                _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
            };

            if let Ok(s) = stream {
                if s.play().is_ok() {
                    Some((s, model_lock))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // In headless or if stream failed, fallback to a dummy thread to drain the commands
    let _fallback_thread = if _stream.is_none() {
        let mut model = AudioModel::new(
            MARKET_WIDTH,
            MARKET_HEIGHT,
            bounded(1).1,
            bounded(1).0,
            None,
        );
        Some(thread::spawn(move || {
            let mut buffer = vec![0.0; 256];
            loop {
                model.process(&mut buffer);
                thread::sleep(Duration::from_millis(5));
            }
        }))
    } else {
        None
    };

    if args.contains(&"--headless".to_string()) {
        let mut app = App::new(cmd_tx, snap_rx);
        for _ in 0..10 {
            app.update();
        }
        println!("Headless execution completed successfully.");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(cmd_tx, snap_rx);
    let mut last_tick = Instant::now();

    while app.running {
        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    app.running = false;
                }
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            app.update();
            last_tick = Instant::now();
        }

        app.render(&mut terminal)?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
