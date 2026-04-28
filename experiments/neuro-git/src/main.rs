//! # 🧬 Splice: `neuro-git`
//!
//! **Lineage:** `crates/neuro-sim` × `crates/git-associates`
//!
//! **Concept:** Neural Git History. The history of a codebase acts as the stimulus to a Spiking Neural Network (SNN).
//!
//! **Novel Trait:** Commits inject current into a neural network. Insertions excite excitatory neurons,
//! deletions excite inhibitory neurons. The SNN's firing rate visualizes the "stress" and "cognitive load"
//! of the repository's development history.

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
    env, io,
    time::{Duration, Instant},
};

use git_associates::GitModel;
use neuro_sim::Network;
use rand::Rng;

const GRID_W: usize = 120;
const GRID_H: usize = 50;

fn main() -> Result<()> {
    // 1. Initialize Git History
    let current_dir = env::current_dir().unwrap_or_else(|_| ".".into());
    let repo_path = std::env::args()
        .nth(1)
        .map(std::path::PathBuf::from)
        .unwrap_or(current_dir);

    let git_model = GitModel::open(&repo_path)?;
    let mut history = git_model.history_with_diffs(200)?;
    history.reverse(); // Play chronologically

    // 2. Initialize SNN
    let mut network = Network::new();
    let mut neuron_positions = Vec::new();
    let mut rng = rand::thread_rng();
    let num_neurons = 60;

    for _ in 0..num_neurons {
        let n_id = network.add_neuron();
        let x = rng.gen_range(5..GRID_W - 5);
        let y = rng.gen_range(5..GRID_H - 5);
        neuron_positions.push((n_id, x, y));
    }

    // Connect them with mostly excitatory, some inhibitory
    for i in 0..num_neurons {
        for j in 0..num_neurons {
            if i != j && rng.gen_bool(0.1) {
                let weight = if rng.gen_bool(0.7) { 12.0 } else { -8.0 };
                network.add_synapse(neuron_positions[i].0, neuron_positions[j].0, weight);
            }
        }
    }

    // 3. Setup TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut last_update = Instant::now();
    let tick_rate = Duration::from_millis(30);

    let mut commit_idx = 0;
    let mut last_commit_time = Instant::now();
    let commit_interval = Duration::from_millis(800); // Trigger commit every 800ms

    let mut ext_inputs = vec![0.0; num_neurons];

    loop {
        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if last_update.elapsed() >= tick_rate {
            last_update = Instant::now();

            // SNN Physics step
            network.step(&ext_inputs);

            // Apply gradual decay to ext inputs to simulate trailing stimuli
            for input in ext_inputs.iter_mut() {
                *input *= 0.8;
                if *input < 0.1 {
                    *input = 0.0;
                }
            }

            // Inject Git metadata into the network periodically
            if last_commit_time.elapsed() >= commit_interval {
                last_commit_time = Instant::now();

                if commit_idx < history.len() {
                    let commit = &history[commit_idx];

                    if let Some(stats) = &commit.stats {
                        let total_changes = stats.insertions + stats.deletions;
                        if total_changes > 0 {
                            let mut rng = rand::thread_rng();

                            // Insertions excite network (positive current)
                            let insertion_current =
                                (stats.insertions as f32).sqrt().clamp(0.0, 50.0);
                            for input in ext_inputs.iter_mut() {
                                if rng.gen_bool(0.3) {
                                    *input += insertion_current;
                                }
                            }

                            // Deletions inhibit network (negative current)
                            let deletion_current = (stats.deletions as f32).sqrt().clamp(0.0, 50.0);
                            for input in ext_inputs.iter_mut() {
                                if rng.gen_bool(0.3) {
                                    *input -= deletion_current;
                                }
                            }
                        }
                    }
                    commit_idx += 1;
                }
            }

            terminal.draw(|f| {
                let size = f.area();

                let title = if commit_idx < history.len() {
                    let c = &history[commit_idx];
                    format!(
                        "Neuro Git - Commit: {} | {} (Press 'q' to quit)",
                        c.short_hash, c.message
                    )
                } else {
                    "Neuro Git - End of History (Press 'q' to quit)".to_string()
                };

                let canvas = ratatui::widgets::canvas::Canvas::default()
                    .block(Block::default().borders(Borders::ALL).title(title))
                    .x_bounds([0.0, GRID_W as f64])
                    .y_bounds([0.0, GRID_H as f64])
                    .paint(|ctx| {
                        for &(id, x, y) in &neuron_positions {
                            let color = if network.is_spiking(id) {
                                Color::Cyan
                            } else {
                                Color::DarkGray
                            };
                            ctx.print(
                                x as f64,
                                (GRID_H - y) as f64,
                                Span::styled("O", Style::default().fg(color)),
                            );
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
