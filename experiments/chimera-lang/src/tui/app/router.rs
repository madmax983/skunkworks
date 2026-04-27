use crate::tui::state::{AppState, ViewMode};
use crate::vm::ChimeraVM;
use ratatui::Frame;

#[cfg(not(feature = "nova"))]
use crate::tui::views::audio::*;
#[cfg(not(feature = "nova"))]
use crate::tui::views::bio::*;
#[cfg(not(feature = "nova"))]
use crate::tui::views::core::*;
#[cfg(not(feature = "nova"))]
use crate::tui::views::misc::*;
#[cfg(not(feature = "nova"))]
use crate::tui::views::physics::*;
#[cfg(feature = "nova")]
use crate::tui::views::*;

pub(crate) fn route_view(f: &mut Frame, vm: &mut ChimeraVM, app_state: &mut AppState) {
    #[cfg(feature = "nova")]
    if matches!(app_state.view_mode, ViewMode::Terminal | ViewMode::Void) {
        let area = app_state.get_render_area(f.area());
        app_state.matrix_rain.render(f.buffer_mut(), area);
    }

    let bypasses_glitch = match app_state.view_mode {
        ViewMode::Microscope => {
            render_microscope(f, vm, app_state);
            true
        }
        #[cfg(feature = "biophysics")]
        ViewMode::Cortex => {
            render_cortex(f, vm, app_state);
            true
        }
        #[cfg(feature = "resonance")]
        ViewMode::Resonance => {
            render_resonance(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Grimoire => {
            render_grimoire(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Topology => {
            render_topology(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Laboratory => {
            render_laboratory(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Graveyard => {
            render_graveyard(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Retina => {
            render_retina(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Quantum => {
            render_quantum(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Dream => {
            render_dream(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Phylogeny => {
            render_phylogeny(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Alchemy => {
            render_alchemy(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::PianoRoll => {
            render_piano_roll(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Memetics => {
            render_memetics(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Egregore => {
            render_egregore(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Bestiary => {
            render_bestiary(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Kaleidoscope => {
            render_kaleidoscope(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Void => {
            render_void(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Signals => {
            render_signals(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Sovereignty => {
            render_sovereignty(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Spectrogram => {
            render_spectrogram(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Market => {
            render_market(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Ballistics => {
            render_ballistics(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Scent => {
            render_scent(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Fishing => {
            render_fishing(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Arena => {
            render_arena(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Garden => {
            render_garden(f, vm, app_state);
            true
        }
        #[cfg(feature = "elektra")]
        ViewMode::Elektra => {
            render_elektra(f, vm, app_state);
            true
        }
        #[cfg(feature = "silicon")]
        ViewMode::Schematic => {
            render_schematic(f, vm, app_state);
            true
        }
        #[cfg(feature = "silicon")]
        ViewMode::Foundry => {
            render_foundry(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Orca => {
            render_orca(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Babel => {
            render_babel(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Strings => {
            render_strings(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Quipu => {
            render_quipu(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Hydra => {
            render_hydra(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Chronos => {
            render_chronos(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Logos => {
            render_logos(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Pandemonium => {
            render_pandemonium(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::BioticChaos => {
            render_biotic_chaos(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Catalyst => {
            render_catalyst(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Hyperspace => {
            render_hyperspace(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Hologram => {
            render_hologram(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Weaver => {
            render_weaver(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Attractor => {
            render_attractor(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Virology => {
            render_virology(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::BioMesh => {
            render_biomesh(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Reactor => {
            render_reactor(f, vm, app_state);
            true
        }
        ViewMode::Evolution => {
            render_evolution(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Ecology => {
            render_ecology(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Savant => {
            render_savant(f, vm, app_state);
            true
        }
        #[cfg(feature = "nova")]
        ViewMode::Prolouge => {
            crate::tui::views::magic::render_prolouge(f, vm, app_state);
            true
        }

        // These ones fell through in the original code
        ViewMode::Genome | ViewMode::Grid => {
            crate::tui::views::core::render_genome_and_grid(f, vm, app_state);
            false
        }
        ViewMode::Sequencer => {
            render_sequencer(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Heatmap => {
            render_heatmap(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Paradox => {
            render_paradox(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Codex => {
            render_codex(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Choir => {
            render_choir(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Genesis => {
            render_genesis(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Terminal => {
            render_terminal(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Forge => {
            render_forge(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Prologue => {
            render_prologue(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Verbum => {
            render_verbum(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Crispr => {
            render_crispr(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Lexicon => {
            render_lexicon(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Narrative => {
            render_narrative(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Akashic => {
            render_akashic(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Cambrian => {
            render_cambrian(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Biolum => {
            render_biolum(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::LifeCycle => {
            render_lifecycle(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Tesseract => {
            render_tesseract(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Metazoa => {
            render_metazoa(f, vm, app_state);
            false
        }
        ViewMode::Mutagen => {
            render_mutagen(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Fractal => {
            render_fractal(f, vm, app_state);
            false
        }
        #[cfg(feature = "nova")]
        ViewMode::Semiotics => {
            render_semiotics(f, vm, app_state);
            false
        }
    };

    if bypasses_glitch {
        return;
    }

    // Apply glitch effect to entire screen if level > 0
    if vm.glitch_level > 0.0 {
        let intensity = (vm.glitch_level / 100.0).clamp(0.0, 1.0);
        crate::tui::apply_glitch_fx(f.buffer_mut(), intensity);
        vm.glitch_level *= 0.9;
        if vm.glitch_level < 0.1 {
            vm.glitch_level = 0.0;
        }
    }
}
