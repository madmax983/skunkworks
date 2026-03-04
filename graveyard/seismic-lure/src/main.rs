mod audio;
mod graph;
mod scanner;
mod sim;

use std::{
    error::Error,
    io,
    path::PathBuf,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, MouseEventKind};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use tui_shared::Tui;

use audio::ScannedSynth;
use graph::Graph;
use sim::WaveTank;

struct App {
    tank: WaveTank,
    graph: Graph,
    synth: ScannedSynth,
    running: bool,
    freq: f32,
    scale: f64,
    center_x: f64,
    center_y: f64,
}

impl App {
    fn new(width: usize, height: usize, path: PathBuf) -> Self {
        let scans = scanner::scan_workspace(path);
        let graph = Graph::new(scans);

        Self {
            tank: WaveTank::new(width, height),
            graph,
            synth: ScannedSynth::new(),
            running: true,
            freq: 110.0,
            scale: 2.0, // Zoom factor
            center_x: 0.0,
            center_y: 0.0,
        }
    }

    fn on_tick(&mut self) {
        self.graph.step();
        self.tank.step();

        // Coupling: Stress nodes disturb the water
        let w = self.tank.width as f64;
        let h = self.tank.height as f64;

        let cx = w / 2.0;
        let cy = h / 2.0;

        for node in &self.graph.nodes {
            if node.stress > 0.0 {
                // Map graph coords (centered at 0,0) to tank coords (0..w, 0..h)
                // Apply scale and offset
                let x = (node.pos.x - self.center_x) * self.scale + cx;
                let y = (node.pos.y - self.center_y) * self.scale + cy;

                let ix = x as usize;
                let iy = y as usize;

                if ix > 0 && ix < self.tank.width - 1 && iy > 0 && iy < self.tank.height - 1 {
                    // Disturbance amount proportional to stress
                    // But prevent explosion, scale down
                    let amount = (node.stress * 0.05).min(2.0);
                    // Use sin wave to create pulsing effect? Or just constant pressure?
                    // Constant pressure creates a static hill. We want waves.
                    // Random noise or oscillation?
                    // Let's use oscillation based on stress level?
                    // Simple: Random jitter
                    let jitter = rand::random::<f32>() * 2.0 - 1.0;
                    if jitter > 0.0 {
                        self.tank.disturb(ix, iy, amount as f32 * jitter);
                    }
                }
            }
        }

        // Audio Generation
        let mid = self.tank.height / 2;
        let row = self.tank.get_row(mid);
        self.synth.push_frame(row, 1.0 / 60.0, self.freq);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        std::env::current_dir()?
    };

    let mut tui = Tui::init()?;

    let size = tui.terminal.size()?;
    let width = size.width as usize;
    let height = (size.height.saturating_sub(2)) as usize;

    let mut app = App::new(width, height, path);

    let res = run_app(&mut tui.terminal, &mut app);

    if let Err(e) = app.synth.save("seismic_session.wav") {
        eprintln!("Failed to save audio: {}", e);
    }

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(0), Constraint::Length(1)])
                    .split(f.area());

                let mut lines = Vec::new();
                let chars = [' ', ' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

                // Render WaveTank + Overlay Nodes
                // We construct a grid of characters first, then convert to Lines.
                // But we can't easily overlay efficiently with just `Paragraph`.
                // We have to build the string line by line.

                let w = app.tank.width;
                let h = app.tank.height;

                let cx = w as f64 / 2.0;
                let cy = h as f64 / 2.0;

                // Pre-calculate node positions on screen
                let mut node_map = vec![vec![None; w]; h];
                for node in &app.graph.nodes {
                    let x = (node.pos.x - app.center_x) * app.scale + cx;
                    let y = (node.pos.y - app.center_y) * app.scale + cy;
                    let ix = x as usize;
                    let iy = y as usize;
                    if ix < w && iy < h {
                        node_map[iy][ix] = Some(node);
                    }
                }

                for y in 0..h {
                    let mut spans = Vec::new();
                    for x in 0..w {
                        if let Some(node) = node_map[y][x] {
                            // Draw Node
                            let symbol = if node.stress > 0.0 { "●" } else { "○" };
                            let color = if node.stress > 10.0 {
                                Color::Red
                            } else if node.stress > 0.0 {
                                Color::Yellow
                            } else {
                                Color::Green
                            };
                            spans.push(Span::styled(symbol, Style::default().fg(color)));
                        } else {
                            // Draw Wave
                            let height_val = app.tank.get_height(x, y);
                            let idx = ((height_val + 0.5).clamp(0.0, 1.0)
                                * (chars.len() - 1) as f32)
                                as usize;
                            let c = chars[idx];
                            // Color based on height
                            let color = if height_val > 0.2 {
                                Color::Cyan
                            } else if height_val < -0.2 {
                                Color::Blue
                            } else {
                                Color::DarkGray
                            };
                            spans.push(Span::styled(c.to_string(), Style::default().fg(color)));
                        }
                    }
                    lines.push(Line::from(spans));
                }

                let tank_widget = Paragraph::new(lines);
                f.render_widget(tank_widget, chunks[0]);

                let status = format!(
                    "Freq: {:.1}Hz | Files: {} | Stressors: {} | WASD Pan | +/- Zoom | 'q' Quit",
                    app.freq,
                    app.graph.nodes.len(),
                    app.graph.nodes.iter().filter(|n| n.stress > 0.0).count()
                );
                f.render_widget(
                    Paragraph::new(status).block(Block::default().borders(Borders::TOP)),
                    chunks[1],
                );
            })
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.running = false,
                    KeyCode::Char('w') => app.center_y -= 1.0 / app.scale,
                    KeyCode::Char('s') => app.center_y += 1.0 / app.scale,
                    KeyCode::Char('a') => app.center_x -= 1.0 / app.scale,
                    KeyCode::Char('d') => app.center_x += 1.0 / app.scale,
                    KeyCode::Char('=') | KeyCode::Char('+') => app.scale *= 1.1,
                    KeyCode::Char('-') => app.scale *= 0.9,
                    KeyCode::Up => app.freq += 10.0,
                    KeyCode::Down => app.freq -= 10.0,
                    _ => {}
                },
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
