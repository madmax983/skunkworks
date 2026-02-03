use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use neuro_terminal::nn::Network;
use rand::Rng;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    symbols::Marker,
    text::Span,
    widgets::{
        canvas::{Canvas, Line, Points},
        Block, Borders, Paragraph,
    },
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    let mut app = App::new();
    let res = run_app(&mut tui.terminal, &mut app);

    if let Err(err) = res {
        tui.exit()?;
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
            let dist = x.hypot(y);
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
            self.network.train(&self.inputs[idx], &self.targets[idx]);
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
                        app.network = Network::new(vec![2, 5, 4, 1], 0.1);
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
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Body
            Constraint::Length(1), // Footer
        ])
        .split(f.area());

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical_chunks[1]);

    draw_header(f, app, vertical_chunks[0]);
    draw_decision_boundary(f, app, body_chunks[0]);
    draw_network(f, app, body_chunks[1]);
    draw_footer(f, app, vertical_chunks[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let status_text = if app.paused {
        Span::styled(" PAUSED ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" RUNNING ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    };

    let title_text = Span::styled(" NEURO-TERMINAL ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let steps_text = Span::raw(format!(" Steps: {} ", app.steps));

    let line = ratatui::text::Line::from(vec![
        title_text,
        Span::raw(" | "),
        status_text,
        Span::raw(" | "),
        steps_text,
    ]);

    let paragraph = Paragraph::new(line)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn draw_footer(f: &mut Frame, _app: &App, area: Rect) {
    let keys = Span::styled(
        " [P]ause | [R]eset | [Q]uit ",
        Style::default().fg(Color::DarkGray),
    );
    let paragraph = Paragraph::new(keys)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn draw_decision_boundary(f: &mut Frame, app: &App, area: Rect) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Decision Boundary "),
        )
        .marker(Marker::Block)
        .x_bounds([-1.0, 1.0])
        .y_bounds([-1.0, 1.0])
        .paint(|ctx| {
            // Sample grid for decision boundary background
            // Draw background first so data points are on top
            for x_i in 0..60 {
                for y_i in 0..30 {
                    let x = -1.0 + x_i as f64 * 2.0 / 59.0;
                    let y = -1.0 + y_i as f64 * 2.0 / 29.0;
                    let out = app.network.predict(&[x, y]);

                    if out[0] > 0.5 {
                        ctx.draw(&Points {
                            coords: &[(x, y)],
                            color: Color::Rgb(20, 40, 40), // Dark Cyan background
                        });
                    } else {
                         ctx.draw(&Points {
                            coords: &[(x, y)],
                            color: Color::Rgb(40, 20, 40), // Dark Magenta background
                        });
                    }
                }
            }

            // Draw dataset points
            for (i, input) in app.inputs.iter().enumerate() {
                let color = if app.targets[i][0] > 0.5 {
                    Color::Cyan
                } else {
                    Color::Magenta
                };
                ctx.draw(&Points {
                    coords: &[(input[0], input[1])],
                    color,
                });
            }
        });
    f.render_widget(canvas, area);
}

fn draw_network(f: &mut Frame, app: &App, area: Rect) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Network Graph "),
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
                    ctx.draw(&Points {
                        coords: &[(x, y)],
                        color: Color::White,
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
                        let w = weights.get(i, j);

                        // Cyberpunk colors: Cyan (+) / Magenta (-)
                        let color = if w > 0.0 { Color::Cyan } else { Color::Magenta };

                        // Lower threshold to see more structure
                        if w.abs() > 0.2 {
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
