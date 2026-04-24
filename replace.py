content = """use crate::tui::state::{AppState, ViewMode};
use crate::vm::ChimeraVM;
use anyhow::Result;
use crossterm::event::KeyCode;

pub(crate) fn handle_navigation_input(
    key_code: KeyCode,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
    if app_state.view_mode.is_grid_navigable() {
        match key_code {
            KeyCode::Down => {
                if app_state.grid_cursor.1 < 15 {
                    app_state.grid_cursor.1 += 1;
                }
            }
            KeyCode::Up => {
                if app_state.grid_cursor.1 > 0 {
                    app_state.grid_cursor.1 -= 1;
                }
            }
            KeyCode::Right => {
                if app_state.grid_cursor.0 < 15 {
                    app_state.grid_cursor.0 += 1;
                }
            }
            KeyCode::Left => {
                if app_state.grid_cursor.0 > 0 {
                    app_state.grid_cursor.0 -= 1;
                }
            }
            _ => {}
        }
        return Ok(false);
    }

    match key_code {
        KeyCode::Down => handle_down_key(vm, app_state),
        KeyCode::Up => handle_up_key(vm, app_state),
        KeyCode::Right => handle_right_key(vm, app_state),
        KeyCode::Left => handle_left_key(vm, app_state),
        _ => {}
    }
    Ok(false)
}

fn handle_down_key(vm: &mut ChimeraVM, app_state: &mut AppState) {
    match app_state.view_mode {
        ViewMode::Genome => handle_genome_down(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Genesis => handle_genesis_down(app_state),
        ViewMode::Catalyst => handle_catalyst_down(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Pandemonium => handle_pandemonium_down(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Babel => handle_babel_down(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Graveyard => handle_graveyard_down(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Dream => handle_dream_down(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Alchemy => handle_alchemy_down(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Weaver | ViewMode::Laboratory | ViewMode::Prolouge => handle_weaver_lab_prolouge_down(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Grimoire => handle_grimoire_down(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Bestiary => handle_bestiary_down(vm, app_state),
        #[cfg(feature = "biophysics")]
        ViewMode::Cortex => handle_cortex_down(vm, app_state),
        _ => {}
    }
}

fn handle_genome_down(vm: &mut ChimeraVM, app_state: &mut AppState) {
    let s_len = vm.dna.helix.strands.len();
    if s_len > 0 {
        let g_len = vm.dna.helix.strands[app_state.selected_strand].genes.len();
        if app_state.selected_gene + 1 < g_len {
            app_state.selected_gene += 1;
        } else if app_state.selected_strand + 1 < s_len {
            app_state.selected_strand += 1;
            app_state.selected_gene = 0;
        }
    }
}

#[cfg(feature = "nova")]
fn handle_genesis_down(app_state: &mut AppState) {
    if app_state.genesis_focus == 2 && app_state.grid_cursor.1 < 15 {
        app_state.grid_cursor.1 += 1;
    }
}

fn handle_catalyst_down(vm: &mut ChimeraVM, app_state: &mut AppState) {
    if !vm.catalysts.is_empty() && app_state.catalyst_scroll + 1 < vm.catalysts.len() {
        app_state.catalyst_scroll += 1;
    }
}

#[cfg(feature = "nova")]
fn handle_pandemonium_down(app_state: &mut AppState) {
    app_state.pandemonium_cursor.1 += 1.0;
}

#[cfg(feature = "nova")]
fn handle_babel_down(app_state: &mut AppState) {
    app_state.babel_focus = (app_state.babel_focus + 1) % 2;
}

#[cfg(feature = "nova")]
fn handle_graveyard_down(vm: &mut ChimeraVM, app_state: &mut AppState) {
    if app_state.selected_graveyard_strand + 1 < vm.graveyard.len() {
        app_state.selected_graveyard_strand += 1;
    }
}

#[cfg(feature = "nova")]
fn handle_dream_down(vm: &mut ChimeraVM, app_state: &mut AppState) {
    if app_state.selected_dream_trace + 1 < vm.dream_traces.len() {
        app_state.selected_dream_trace += 1;
    }
}

#[cfg(feature = "nova")]
fn handle_alchemy_down(vm: &mut ChimeraVM, app_state: &mut AppState) {
    if app_state.alchemy_selection == 0 {
        if app_state.alchemy_shelf_idx < 7 {
            // 8 items
            app_state.alchemy_shelf_idx += 1;
        }
    } else if app_state.alchemy_strand_idx + 1 < vm.dna.helix.strands.len() {
        app_state.alchemy_strand_idx += 1;
    }
}

#[cfg(feature = "nova")]
fn handle_weaver_lab_prolouge_down(app_state: &mut AppState) {
    match app_state.selected_strand {
        // 0=A, 1=B, 2=Method/Pattern
        0 => {
            if app_state.lab_parent_a > 0 {
                app_state.lab_parent_a -= 1;
            }
        }
        1 => {
            if app_state.lab_parent_b > 0 {
                app_state.lab_parent_b -= 1;
            }
        }
        2 => {
            if let ViewMode::Laboratory = app_state.view_mode {
                if app_state.lab_method > 0 {
                    app_state.lab_method -= 1;
                }
            }
        }
        _ => {}
    }
}

#[cfg(feature = "nova")]
fn handle_grimoire_down(vm: &mut ChimeraVM, app_state: &mut AppState) {
    if app_state.selected_sigil_index + 1 < vm.sigil_registry.len() {
        app_state.selected_sigil_index += 1;
    }
}

#[cfg(feature = "nova")]
fn handle_bestiary_down(vm: &mut ChimeraVM, app_state: &mut AppState) {
    if !vm.organelles.is_empty()
        && app_state.selected_organelle_index + 1 < vm.organelles.len()
    {
        app_state.selected_organelle_index += 1;
    }
}

#[cfg(feature = "biophysics")]
fn handle_cortex_down(vm: &mut ChimeraVM, app_state: &mut AppState) {
    let mut neurons_sorted: Vec<_> = vm.neurons.keys().collect();
    neurons_sorted.sort();
    if let Some(current) = app_state.selected_neuron_coords {
        if let Some(pos) = neurons_sorted.iter().position(|&c| *c == current) {
            if pos + 1 < neurons_sorted.len() {
                app_state.selected_neuron_coords = Some(*neurons_sorted[pos + 1]);
                app_state.voltage_history.clear(); // Reset history on switch
            }
        }
    } else if !neurons_sorted.is_empty() {
        app_state.selected_neuron_coords = Some(*neurons_sorted[0]);
    }
}

fn handle_up_key(vm: &mut ChimeraVM, app_state: &mut AppState) {
    match app_state.view_mode {
        ViewMode::Genome => handle_genome_up(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Pandemonium => handle_pandemonium_up(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Grimoire => handle_grimoire_up(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Weaver | ViewMode::Laboratory | ViewMode::Prolouge => handle_weaver_lab_prolouge_up(vm, app_state),
        #[cfg(feature = "biophysics")]
        ViewMode::Cortex => handle_cortex_up(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Graveyard => handle_graveyard_up(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Dream => handle_dream_up(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Alchemy => handle_alchemy_up(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Bestiary => handle_bestiary_up(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Babel => handle_babel_up(app_state),
        ViewMode::Catalyst => handle_catalyst_up(app_state),
        _ => {}
    }
}

fn handle_genome_up(vm: &mut ChimeraVM, app_state: &mut AppState) {
    if app_state.selected_gene > 0 {
        app_state.selected_gene -= 1;
    } else if app_state.selected_strand > 0 {
        app_state.selected_strand -= 1;
        let g_len = vm.dna.helix.strands[app_state.selected_strand].genes.len();
        if g_len > 0 {
            app_state.selected_gene = g_len - 1;
        } else {
            app_state.selected_gene = 0;
        }
    }
}

#[cfg(feature = "nova")]
fn handle_pandemonium_up(app_state: &mut AppState) {
    app_state.pandemonium_cursor.1 -= 1.0;
}

#[cfg(feature = "nova")]
fn handle_grimoire_up(app_state: &mut AppState) {
    if app_state.selected_sigil_index > 0 {
        app_state.selected_sigil_index -= 1;
    }
}

#[cfg(feature = "nova")]
fn handle_weaver_lab_prolouge_up(vm: &mut ChimeraVM, app_state: &mut AppState) {
    let max_strand = vm.dna.helix.strands.len().saturating_sub(1);
    match app_state.selected_strand {
        // 0=A, 1=B, 2=Method
        0 => {
            if app_state.lab_parent_a < max_strand {
                app_state.lab_parent_a += 1;
            }
        }
        1 => {
            if app_state.lab_parent_b < max_strand {
                app_state.lab_parent_b += 1;
            }
        }
        2 => {
            if let ViewMode::Laboratory = app_state.view_mode {
                if app_state.lab_method < 3 {
                    app_state.lab_method += 1;
                }
            }
        }
        _ => {}
    }
}

#[cfg(feature = "biophysics")]
fn handle_cortex_up(vm: &mut ChimeraVM, app_state: &mut AppState) {
    let mut neurons_sorted: Vec<_> = vm.neurons.keys().collect();
    neurons_sorted.sort();
    if let Some(current) = app_state.selected_neuron_coords {
        if let Some(pos) = neurons_sorted.iter().position(|&c| *c == current) {
            if pos > 0 {
                app_state.selected_neuron_coords = Some(*neurons_sorted[pos - 1]);
                app_state.voltage_history.clear();
            }
        }
    } else if !neurons_sorted.is_empty() {
        app_state.selected_neuron_coords = Some(*neurons_sorted[0]);
    }
}

#[cfg(feature = "nova")]
fn handle_graveyard_up(app_state: &mut AppState) {
    if app_state.selected_graveyard_strand > 0 {
        app_state.selected_graveyard_strand -= 1;
    }
}

#[cfg(feature = "nova")]
fn handle_dream_up(app_state: &mut AppState) {
    if app_state.selected_dream_trace > 0 {
        app_state.selected_dream_trace -= 1;
    }
}

#[cfg(feature = "nova")]
fn handle_alchemy_up(app_state: &mut AppState) {
    if app_state.alchemy_selection == 0 {
        if app_state.alchemy_shelf_idx > 0 {
            app_state.alchemy_shelf_idx -= 1;
        }
    } else if app_state.alchemy_strand_idx > 0 {
        app_state.alchemy_strand_idx -= 1;
    }
}

#[cfg(feature = "nova")]
fn handle_bestiary_up(app_state: &mut AppState) {
    if app_state.selected_organelle_index > 0 {
        app_state.selected_organelle_index -= 1;
    }
}

#[cfg(feature = "nova")]
fn handle_babel_up(app_state: &mut AppState) {
    if app_state.babel_focus > 0 {
        app_state.babel_focus -= 1;
    } else {
        app_state.babel_focus = 1;
    }
}

fn handle_catalyst_up(app_state: &mut AppState) {
    if app_state.catalyst_scroll > 0 {
        app_state.catalyst_scroll -= 1;
    }
}

fn handle_right_key(vm: &mut ChimeraVM, app_state: &mut AppState) {
    match app_state.view_mode {
        #[cfg(feature = "nova")]
        ViewMode::Pandemonium => handle_pandemonium_right(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Weaver | ViewMode::Laboratory | ViewMode::Prolouge => handle_weaver_lab_prolouge_right(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Alchemy => handle_alchemy_right(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Quipu => handle_quipu_right(vm),
        _ => {}
    }
}

#[cfg(feature = "nova")]
fn handle_pandemonium_right(app_state: &mut AppState) {
    app_state.pandemonium_cursor.0 += 1.0;
}

#[cfg(feature = "nova")]
fn handle_weaver_lab_prolouge_right(app_state: &mut AppState) {
    if app_state.selected_strand < 2 {
        app_state.selected_strand += 1;
    } else {
        app_state.selected_strand = 0;
    }
}

#[cfg(feature = "nova")]
fn handle_alchemy_right(app_state: &mut AppState) {
    app_state.alchemy_selection = 1;
}

#[cfg(feature = "nova")]
fn handle_quipu_right(vm: &mut ChimeraVM) {
    if vm.quipu.active_cord + 1 < vm.quipu.cords.len() {
        vm.quipu.active_cord += 1;
    }
}

fn handle_left_key(vm: &mut ChimeraVM, app_state: &mut AppState) {
    match app_state.view_mode {
        #[cfg(feature = "nova")]
        ViewMode::Pandemonium => handle_pandemonium_left(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Weaver | ViewMode::Laboratory | ViewMode::Prolouge => handle_weaver_lab_prolouge_left(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Alchemy => handle_alchemy_left(app_state),
        #[cfg(feature = "nova")]
        ViewMode::Quipu => handle_quipu_left(vm),
        _ => {}
    }
}

#[cfg(feature = "nova")]
fn handle_pandemonium_left(app_state: &mut AppState) {
    app_state.pandemonium_cursor.0 -= 1.0;
}

#[cfg(feature = "nova")]
fn handle_weaver_lab_prolouge_left(app_state: &mut AppState) {
    if app_state.selected_strand > 0 {
        app_state.selected_strand -= 1;
    } else {
        app_state.selected_strand = 2;
    }
}

#[cfg(feature = "nova")]
fn handle_alchemy_left(app_state: &mut AppState) {
    app_state.alchemy_selection = 0;
}

#[cfg(feature = "nova")]
fn handle_quipu_left(vm: &mut ChimeraVM) {
    if vm.quipu.active_cord > 0 {
        vm.quipu.active_cord -= 1;
    }
}
"""

with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "w") as f:
    f.write(content)
