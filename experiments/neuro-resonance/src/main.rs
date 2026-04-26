use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

use neuro_sim::Network;
use resonance_audio::physics::{PhysicsGrid, Material};
use rand::Rng;

const GRID_W: usize = 100;
const GRID_H: usize = 50;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut grid = PhysicsGrid::new(GRID_W, GRID_H);
    for y in 0..GRID_H {
        for x in 0..GRID_W {
            grid.set_material(x, y, Material::Air);
        }
    }

    let mut network = Network::new();
    let mut neuron_positions = Vec::new();

    let mut rng = rand::thread_rng();
    let num_neurons = 30;

    for _ in 0..num_neurons {
        let n_id = network.add_neuron();
        let x = rng.gen_range(5..GRID_W - 5);
        let y = rng.gen_range(5..GRID_H - 5);
        neuron_positions.push((n_id, x, y));
    }

    for i in 0..num_neurons {
        for j in 0..num_neurons {
            if i != j && rng.gen_bool(0.2) {
                let weight = if rng.gen_bool(0.8) { 15.0 } else { -10.0 };
                network.add_synapse(neuron_positions[i].0, neuron_positions[j].0, weight);
            }
        }
    }

    let mut last_update = Instant::now();
    let tick_rate = Duration::from_millis(30);

    let mut ext_inputs = vec![0.0; num_neurons];

    loop {
        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                } else if key.code == KeyCode::Char('p') {
                    let mut rng = rand::thread_rng();
                    for i in 0..num_neurons {
                        if rng.gen_bool(0.3) {
                            ext_inputs[i] = 50.0;
                        }
                    }
                }
            }
        }

        if last_update.elapsed() >= tick_rate {
            last_update = Instant::now();

            network.step(&ext_inputs);
            ext_inputs.fill(0.0);

            for &(id, x, y) in &neuron_positions {
                if network.is_spiking(id) {
                    grid.pluck(x, y, 1.0);
                }
            }

            grid.step();

            terminal.draw(|f| {
                let size = f.area();
                let canvas = ratatui::widgets::canvas::Canvas::default()
                    .block(Block::default().borders(Borders::ALL).title("Neuro Resonance (Press 'p' to pluck, 'q' to quit)"))
                    .x_bounds([0.0, GRID_W as f64])
                    .y_bounds([0.0, GRID_H as f64])
                    .paint(|ctx| {
                        for y in 0..GRID_H {
                            for x in 0..GRID_W {
                                let pressure = grid.get(x, y);
                                if pressure.abs() > 0.1 {
                                    let color = if pressure > 0.0 { Color::Cyan } else { Color::Magenta };
                                    ctx.print(x as f64, (GRID_H - y) as f64, Span::styled("·", Style::default().fg(color)));
                                }
                            }
                        }

                        for &(id, x, y) in &neuron_positions {
                            let color = if network.is_spiking(id) { Color::White } else { Color::Yellow };
                            ctx.print(x as f64, (GRID_H - y) as f64, Span::styled("O", Style::default().fg(color)));
                        }
                    });

                f.render_widget(canvas, size);
            })?;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
