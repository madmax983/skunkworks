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

use neuro_sim::Network;
use rand::Rng;
use resonance_audio::{AudioCommand, AudioModel, AudioSnapshot};

const TICK_RATE: Duration = Duration::from_millis(50);
const GRID_WIDTH: usize = 60;
const GRID_HEIGHT: usize = 30;
const NUM_NEURONS: usize = 50;

struct App {
    network: Network,
    neuron_positions: Vec<(usize, usize)>,
    cmd_tx: Sender<AudioCommand>,
    snap_rx: Receiver<AudioSnapshot>,
    latest_snapshot: Option<AudioSnapshot>,
    running: bool,
    ticks: u64,
}

impl App {
    fn new(cmd_tx: Sender<AudioCommand>, snap_rx: Receiver<AudioSnapshot>) -> Self {
        let mut network = Network::new();
        let mut neuron_positions = Vec::new();
        let mut rng = rand::thread_rng();

        // Create neurons and assign them random topological positions on the 2D audio grid
        for _ in 0..NUM_NEURONS {
            let _n = network.add_neuron();
            let x = rng.gen_range(5..GRID_WIDTH - 5);
            let y = rng.gen_range(5..GRID_HEIGHT - 5);
            neuron_positions.push((x, y));
        }

        // Create synapses to form a connected SNN
        for i in 0..NUM_NEURONS {
            // Connect to 3 random neighbors
            for _ in 0..3 {
                let target = rng.gen_range(0..NUM_NEURONS);
                if target != i {
                    // Mix of excitatory and inhibitory weights
                    let weight = if rng.gen_bool(0.2) { -15.0 } else { 20.0 };
                    let delay = rng.gen_range(1..5);
                    network.add_synapse_with_delay(i, target, weight, delay);
                }
            }
        }

        Self {
            network,
            neuron_positions,
            cmd_tx,
            snap_rx,
            latest_snapshot: None,
            running: true,
            ticks: 0,
        }
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();
        let mut inputs = vec![0.0; NUM_NEURONS];

        // Randomly inject current into a few neurons to keep the network active
        for input in inputs.iter_mut() {
            if rng.gen_bool(0.05) {
                *input = 50.0;
            }
        }

        self.network.step(&inputs);

        // Map neuron spikes to audio impulses
        for (i, &(x, y)) in self.neuron_positions.iter().enumerate() {
            if self.network.is_spiking(i) {
                // Send an acoustic pluck at this coordinate
                let _ = self.cmd_tx.send(AudioCommand::Pluck {
                    x,
                    y,
                    strength: 0.9,
                });
            }
        }

        // Receive the latest snapshot
        while let Ok(snap) = self.snap_rx.try_recv() {
            self.latest_snapshot = Some(snap);
        }

        self.ticks += 1;
    }

    fn render(&self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default()
                .title(" Sonification of Neural Dynamics 🧠🔊 ")
                .borders(Borders::ALL);

            let inner_area = block.inner(size);
            f.render_widget(block, size);

            let mut lines = Vec::new();

            let pressure_map = self.latest_snapshot.as_ref().map(|s| &s.pressure);

            for y in 0..GRID_HEIGHT.min(inner_area.height as usize) {
                let mut spans = Vec::new();
                for x in 0..GRID_WIDTH.min(inner_area.width as usize) {
                    let mut p = 0.0;
                    if let Some(map) = pressure_map {
                        if y < GRID_HEIGHT && x < GRID_WIDTH {
                            p = map[y * GRID_WIDTH + x];
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

                    // Check if a neuron is here
                    let mut ch = " ";
                    let mut fg_color = Color::Reset;

                    for (i, &(nx, ny)) in self.neuron_positions.iter().enumerate() {
                        if nx == x && ny == y {
                            if self.network.is_spiking(i) {
                                ch = "X";
                                fg_color = Color::White;
                            } else {
                                ch = "O";
                                fg_color = Color::DarkGray;
                            }
                            break;
                        }
                    }

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

    let host = cpal::default_host();
    let _stream = if let Some(device) = host.default_output_device() {
        if let Ok(config) = device.default_output_config() {
            let model = AudioModel::new(
                GRID_WIDTH,
                GRID_HEIGHT,
                cmd_rx.clone(),
                snap_tx.clone(),
                None,
            );
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
        let mut fallback_model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);
        Some(thread::spawn(move || {
            let mut buffer = vec![0.0; 256];
            loop {
                fallback_model.process(&mut buffer);
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
