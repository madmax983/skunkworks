use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use neuro_terminal::nn::Network;
use rand::Rng;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
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
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(area);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let title = Paragraph::new(Span::styled(
        " NEURO-TERMINAL 🧠 ",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
    .block(title_block)
    .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let stats_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
    let stats = Paragraph::new(Span::styled(
        format!("Steps: {}", app.steps),
        Style::default().fg(Color::Yellow),
    ))
    .block(stats_block)
    .alignment(Alignment::Center);
    f.render_widget(stats, chunks[1]);

    let status_style = if app.paused {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    };
    let status_block = Block::default()
        .borders(Borders::ALL)
        .border_style(status_style);
    let status_text = if app.paused {
        "PAUSED ⏸"
    } else {
        "RUNNING ▶"
    };
    let status = Paragraph::new(Span::styled(status_text, status_style))
        .block(status_block)
        .alignment(Alignment::Center);
    f.render_widget(status, chunks[2]);
}

fn draw_footer(f: &mut Frame, _app: &App, area: Rect) {
    let keys = vec![
        Span::raw(" "),
        Span::styled("Q", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" Quit | "),
        Span::styled("P", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" Pause/Resume | "),
        Span::styled("R", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" Reset "),
    ];
    let line = ratatui::text::Line::from(keys);
    let paragraph = Paragraph::new(line)
        .alignment(Alignment::Center)
        .style(Style::default().bg(Color::Rgb(20, 20, 20)).fg(Color::White));
    f.render_widget(paragraph, area);
}

fn draw_decision_boundary(f: &mut Frame, app: &App, area: Rect) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Decision Boundary 📊 "),
        )
        .marker(Marker::Block)
        .x_bounds([-1.0, 1.0])
        .y_bounds([-1.0, 1.0])
        .paint(|ctx| {
            // Sample grid for decision boundary background
            for x_i in 0..60 {
                for y_i in 0..30 {
                    let x = -1.0 + x_i as f64 * 2.0 / 59.0;
                    let y = -1.0 + y_i as f64 * 2.0 / 29.0;
                    let out = app.network.predict(&[x, y]);

                    if out[0] > 0.5 {
                        ctx.draw(&Points {
                            coords: &[(x, y)],
                            color: Color::Rgb(0, 40, 40), // Darker Cyan background
                        });
                    } else {
                        ctx.draw(&Points {
                            coords: &[(x, y)],
                            color: Color::Rgb(40, 0, 40), // Darker Magenta background
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
                .title(" Network Graph 🕸️ "),
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

                    // Color neuron based on activation if available
                    let color = if let Some(layer_data) = app.network.data.get(l) {
                        let activation = layer_data.get(n, 0);
                        let a = activation.clamp(0.0, 1.0);
                        // Gradient: Magenta (0.0) -> Cyan (1.0)
                        // Magenta: 255, 0, 255
                        // Cyan: 0, 255, 255
                        let r = (255.0 * (1.0 - a)) as u8;
                        let g = (255.0 * a) as u8;
                        Color::Rgb(r, g, 255)
                    } else {
                        Color::White
                    };

                    ctx.draw(&Points {
                        coords: &[(x, y)],
                        color,
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

                        let color = if w > 0.0 { Color::Cyan } else { Color::Magenta };

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
