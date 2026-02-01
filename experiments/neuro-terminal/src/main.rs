use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use neuro_terminal::nn::Network;
use rand::Rng;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Line, Points},
        Block, Borders,
    },
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();

    // Run app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

struct App {
    network: Network,
    inputs: Vec<Vec<f64>>,
    targets: Vec<Vec<f64>>,
    steps: usize,
    paused: bool,
}

impl App {
    fn new() -> Self {
        // Circle dataset
        let mut inputs = vec![];
        let mut targets = vec![];
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let x: f64 = rng.gen_range(-1.0..1.0);
            let y: f64 = rng.gen_range(-1.0..1.0);
            inputs.push(vec![x, y]);

            // Circle radius 0.6
            let dist = (x.powi(2) + y.powi(2)).sqrt();
            if dist < 0.6 {
                targets.push(vec![1.0]);
            } else {
                targets.push(vec![0.0]);
            }
        }

        Self {
            network: Network::new(vec![2, 5, 4, 1], 0.1),
            inputs,
            targets,
            steps: 0,
            paused: false,
        }
    }

    fn update(&mut self) {
        if self.paused {
            return;
        }

        let mut rng = rand::thread_rng();
        // Train on random batch
        for _ in 0..10 {
            let idx = rng.gen_range(0..self.inputs.len());
            self.network
                .train(self.inputs[idx].clone(), self.targets[idx].clone());
            self.steps += 1;
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('p') => app.paused = !app.paused,
                    KeyCode::Char('r') => {
                        let new_app = App::new();
                        app.network = new_app.network;
                        app.steps = 0;
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    draw_decision_boundary(f, app, chunks[0]);
    draw_network(f, app, chunks[1]);
}

fn draw_decision_boundary(f: &mut Frame, app: &App, area: Rect) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Decision Boundary"),
        )
        .marker(Marker::Block)
        .x_bounds([-1.0, 1.0])
        .y_bounds([-1.0, 1.0])
        .paint(|ctx| {
            // Draw dataset points
            for (i, input) in app.inputs.iter().enumerate() {
                let color = if app.targets[i][0] > 0.5 {
                    Color::Green
                } else {
                    Color::Red
                };
                ctx.draw(&Points {
                    coords: &[(input[0], input[1])],
                    color,
                });
            }

            // Sample grid for decision boundary background
            for x_i in 0..40 {
                for y_i in 0..40 {
                    let x = -1.0 + (x_i as f64 / 20.0);
                    let y = -1.0 + (y_i as f64 / 20.0);
                    let out = app.network.predict(vec![x, y]);

                    if out[0] > 0.5 {
                        ctx.draw(&Points {
                            coords: &[(x, y)],
                            color: Color::Rgb(50, 50, 50), // Faint grey/white
                        });
                    }
                }
            }
        });
    f.render_widget(canvas, area);
}

fn draw_network(f: &mut Frame, app: &App, area: Rect) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Network (Steps: {})", app.steps)),
        )
        .marker(Marker::Braille)
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(|ctx| {
            // Draw layers
            let layer_count = app.network.layers.len();
            let x_step = 100.0 / (layer_count as f64 + 1.0);

            let mut node_positions = vec![];

            for (l, &neuron_count) in app.network.layers.iter().enumerate() {
                let x = x_step * (l as f64 + 1.0);
                let y_step = 100.0 / (neuron_count as f64 + 1.0);

                let mut layer_nodes = vec![];
                for n in 0..neuron_count {
                    let y = y_step * (n as f64 + 1.0);
                    layer_nodes.push((x, y));

                    // Draw node
                    // We can use a small circle or point
                    ctx.draw(&Points {
                        coords: &[(x, y)],
                        color: Color::Yellow,
                    });
                }
                node_positions.push(layer_nodes);
            }

            // Draw weights (connections)
            for l in 0..layer_count - 1 {
                let weights = &app.network.weights[l];
                let current_layer_nodes = &node_positions[l];
                let next_layer_nodes = &node_positions[l + 1];

                for (i, to_pos) in next_layer_nodes.iter().enumerate() {
                    for (j, from_pos) in current_layer_nodes.iter().enumerate() {
                        // Weight connects from_pos (j) to to_pos (i)
                        // weights matrix is [next_layer_size, current_layer_size]
                        // value is weights[i][j] (row i, col j)

                        let idx = i * weights.cols + j;
                        let w = weights.data[idx];

                        // Color based on weight sign
                        let color = if w > 0.0 { Color::Green } else { Color::Red };

                        // Thickness/brightness based on magnitude?
                        // Canvas Line doesn't support thickness.
                        // We can threshold drawing.
                        if w.abs() > 0.5 {
                            ctx.draw(&Line {
                                x1: from_pos.0,
                                y1: from_pos.1,
                                x2: to_pos.0,
                                y2: to_pos.1,
                                color,
                            });
                        }
                    }
                }
            }
        });
    f.render_widget(canvas, area);
}
