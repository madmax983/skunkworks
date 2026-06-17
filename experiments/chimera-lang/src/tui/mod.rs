//! Terminal User Interface (TUI) frontend for the Chimera Engine.
//!
//! Provides the primary visualization and interactive environment for running
//! Biological scripts, esoteric PrologueEsolang circuits, and observing cellular automata.

use crate::vm::ChimeraVM;
use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders},
    Terminal,
};
use std::io;

pub(crate) const GRIMOIRE_TEXT: &str = include_str!("../../GRIMOIRE.md");

/// The `state` module.
pub(crate) mod state;
pub use state::*;
pub(crate) mod app;
pub(crate) mod views;
pub(crate) fn panel_block<'a>(title: &'a str, active: bool) -> Block<'a> {
    let border_style = if active {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Cyan)
    };

    Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            title,
            if active {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            },
        ))
        .border_style(border_style)
}

/// Performs the `run_tui` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of run_tui
/// ```
pub fn run_tui(
    mut vm: ChimeraVM,
    initial_view: Option<ViewMode>,
    source_path: Option<std::path::PathBuf>,
) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::new(initial_view, source_path);

    // Initialize Evolution Engine if config is present
    if let Some(config) = &vm.dna.evolution_config {
        if let Some(strand) = vm.dna.helix.strands.first() {
            app_state.evolution_state.engine = Some(
                crate::vm::evolution::EvolutionEngine::from_config(strand.clone(), config.clone()),
            );
            app_state.evolution_state.challenge =
                crate::vm::evolution::Challenge::Custom(config.clone());
            app_state.view_mode = ViewMode::Evolution; // Auto-switch to view
        }
    }

    let res = app::run_app(&mut terminal, &mut vm, &mut app_state);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

pub(crate) fn parse_grid_value(s: &str) -> crate::vm::Value {
    if s.is_empty() {
        return crate::vm::Value::Int(0);
    }
    if let Ok(i) = s.parse::<i64>() {
        crate::vm::Value::Int(i)
    } else {
        crate::vm::Value::Str(s.to_string())
    }
}

pub(crate) fn apply_glitch_fx(buffer: &mut ratatui::buffer::Buffer, intensity: f32) {
    let area = *buffer.area();
    let mut rng = rand::thread_rng();
    use rand::Rng;

    for y in area.y..area.height {
        for x in area.x..area.width {
            if rng.gen::<f32>() < intensity {
                let cell = buffer.cell_mut((x, y)).unwrap();
                match rng.gen_range(0..4) {
                    0 => {
                        let chars = ['@', '#', '$', '%', '&', '!', '?', 'X', '.', ':', ';', '~'];
                        cell.set_char(chars[rng.gen_range(0..chars.len())]);
                    }
                    1 => {
                        std::mem::swap(&mut cell.fg, &mut cell.bg);
                    }
                    2 => {
                        cell.fg = Color::DarkGray;
                    }
                    3 => {
                        cell.set_char('?');
                        cell.set_style(Style::default().fg(Color::Red).bg(Color::Black));
                    }
                    _ => {}
                }
            }
        }
    }
}

#[cfg(feature = "nova")]
#[cfg(feature = "oracle")]
#[allow(dead_code)]
pub(crate) fn get_all_views() -> Vec<(ViewMode, &'static str, &'static str)> {
    let mut views = vec![
        (ViewMode::Genome, "Genome", "Tab"),
        (ViewMode::Grid, "Grid", "Tab"),
        (ViewMode::Microscope, "Microscope", "Tab"),
        (ViewMode::Heatmap, "Heatmap", "h"),
    ];

    #[cfg(feature = "biophysics")]
    views.push((ViewMode::Cortex, "Cortex", "b"));

    #[cfg(feature = "resonance")]
    views.push((ViewMode::Resonance, "Resonance", "Tab"));

    #[cfg(feature = "elektra")]
    views.push((ViewMode::Elektra, "Elektra", "E"));

    #[cfg(feature = "silicon")]
    {
        views.push((ViewMode::Schematic, "Schematic", "s"));
        views.push((ViewMode::Foundry, "Foundry", "F"));
    }

    #[cfg(feature = "nova")]
    {
        views.push((ViewMode::Grimoire, "Grimoire", "Tab"));
        views.push((ViewMode::Laboratory, "Laboratory", "Tab"));
        views.push((ViewMode::PrologueEsolang, "PrologueEsolang", "Tab"));
        views.push((ViewMode::Topology, "Topology", "Tab"));
        views.push((ViewMode::Graveyard, "Graveyard", "Tab"));
        views.push((ViewMode::PianoRoll, "Piano Roll", "p"));
        views.push((ViewMode::Retina, "Retina", "Tab"));
        views.push((ViewMode::Quantum, "Quantum", "Tab"));
        views.push((ViewMode::Dream, "Dream Catcher", "Tab"));
        views.push((ViewMode::Phylogeny, "Phylogeny", "Tab"));
        views.push((ViewMode::Alchemy, "Alchemy", "Tab"));
        views.push((ViewMode::Memetics, "Memetics", "Tab"));
        views.push((ViewMode::Egregore, "Egregore", "Tab"));
        views.push((ViewMode::Bestiary, "Bestiary", "z"));
        views.push((ViewMode::Kaleidoscope, "Kaleidoscope", "k"));
        views.push((ViewMode::Void, "Void", "Tab"));
        views.push((ViewMode::Signals, "Signals", "Tab"));
        views.push((ViewMode::Sovereignty, "Sovereignty", "Tab"));
        views.push((ViewMode::Spectrogram, "Spectrogram", "Tab"));
        views.push((ViewMode::Market, "Market", "$"));
        views.push((ViewMode::Ballistics, "Ballistics", "!"));
        views.push((ViewMode::Scent, "Scent", "~"));
        views.push((ViewMode::Fishing, "Fishing", "f"));
        views.push((ViewMode::Arena, "Arena", "V"));
        views.push((ViewMode::Garden, "Garden", "G"));
        views.push((ViewMode::Orca, "Orca", "O"));
        views.push((ViewMode::Babel, "Babel", "L"));
        views.push((ViewMode::Strings, "Strings", "="));
        views.push((ViewMode::Quipu, "Quipu", "Tab"));
        views.push((ViewMode::Hydra, "Hydra", "Y"));
        views.push((ViewMode::Chronos, "Chronos", "T"));
        views.push((ViewMode::Logos, "Logos", "U"));
        views.push((ViewMode::Pandemonium, "Pandemonium", "P"));
        views.push((ViewMode::BioticChaos, "Biotic Chaos", "Tab"));
        views.push((ViewMode::Catalyst, "Catalyst Chamber", "Tab"));
        views.push((ViewMode::Hyperspace, "Hyperspace", "H"));
        views.push((ViewMode::Hologram, "Hologram", "I"));
        views.push((ViewMode::Weaver, "The Weaver", "W"));
        views.push((ViewMode::Terminal, "Terminal", "`"));
        views.push((ViewMode::Attractor, "Attractor", "A"));
        views.push((ViewMode::Virology, "Virology", "v"));
        views.push((ViewMode::BioMesh, "BioMesh", "N"));
        views.push((ViewMode::Reactor, "Reactor", "X"));
        views.push((ViewMode::Evolution, "Evolution", "E"));
        views.push((ViewMode::Ecology, "Genetic Ecology", "Shift+E"));
        views.push((ViewMode::Savant, "Savant", "Shift+\\"));
    }
    views
}

#[cfg(feature = "nova")]
pub(crate) fn layout_tree_node(
    node_id: usize,
    depth: f64,
    current_y: &mut f64,
    positions: &mut std::collections::HashMap<usize, (f64, f64)>,
    vm: &ChimeraVM,
    max_depth: &mut f64,
) -> f64 {
    if depth > *max_depth {
        *max_depth = depth;
    }

    let mut my_y = *current_y;

    if let Some(node) = vm.cladistics.nodes.get(&node_id) {
        if node.children.is_empty() {
            *current_y += 1.0;
        } else {
            let mut sum_y = 0.0;
            let count = node.children.len() as f64;
            for child_id in &node.children {
                sum_y +=
                    layout_tree_node(*child_id, depth + 1.0, current_y, positions, vm, max_depth);
            }
            my_y = sum_y / count;
        }
        positions.insert(node_id, (depth, my_y));
    }
    my_y
}
