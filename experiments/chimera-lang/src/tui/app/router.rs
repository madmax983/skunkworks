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

    if let ViewMode::Microscope = app_state.view_mode {
        render_microscope(f, vm, app_state);
        return;
    }

    #[cfg(feature = "biophysics")]
    if let ViewMode::Cortex = app_state.view_mode {
        render_cortex(f, vm, app_state);
        return;
    }

    #[cfg(feature = "resonance")]
    if let ViewMode::Resonance = app_state.view_mode {
        render_resonance(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Grimoire = app_state.view_mode {
        render_grimoire(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Topology = app_state.view_mode {
        render_topology(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Laboratory = app_state.view_mode {
        render_laboratory(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Graveyard = app_state.view_mode {
        render_graveyard(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Retina = app_state.view_mode {
        render_retina(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Quantum = app_state.view_mode {
        render_quantum(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Dream = app_state.view_mode {
        render_dream(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Phylogeny = app_state.view_mode {
        render_phylogeny(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Alchemy = app_state.view_mode {
        render_alchemy(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::PianoRoll = app_state.view_mode {
        render_piano_roll(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Memetics = app_state.view_mode {
        render_memetics(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Egregore = app_state.view_mode {
        render_egregore(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Bestiary = app_state.view_mode {
        render_bestiary(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Kaleidoscope = app_state.view_mode {
        render_kaleidoscope(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Void = app_state.view_mode {
        render_void(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Signals = app_state.view_mode {
        render_signals(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Sovereignty = app_state.view_mode {
        render_sovereignty(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Spectrogram = app_state.view_mode {
        render_spectrogram(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Market = app_state.view_mode {
        render_market(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Ballistics = app_state.view_mode {
        render_ballistics(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Scent = app_state.view_mode {
        render_scent(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Fishing = app_state.view_mode {
        render_fishing(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Arena = app_state.view_mode {
        render_arena(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Garden = app_state.view_mode {
        render_garden(f, vm, app_state);
        return;
    }

    #[cfg(feature = "elektra")]
    if let ViewMode::Elektra = app_state.view_mode {
        render_elektra(f, vm, app_state);
        return;
    }

    #[cfg(feature = "silicon")]
    if let ViewMode::Schematic = app_state.view_mode {
        render_schematic(f, vm, app_state);
        return;
    }

    #[cfg(feature = "silicon")]
    if let ViewMode::Foundry = app_state.view_mode {
        render_foundry(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Orca = app_state.view_mode {
        render_orca(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Babel = app_state.view_mode {
        render_babel(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Strings = app_state.view_mode {
        render_strings(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Quipu = app_state.view_mode {
        render_quipu(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Hydra = app_state.view_mode {
        render_hydra(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Chronos = app_state.view_mode {
        render_chronos(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Logos = app_state.view_mode {
        render_logos(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Pandemonium = app_state.view_mode {
        render_pandemonium(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::BioticChaos = app_state.view_mode {
        render_biotic_chaos(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Catalyst = app_state.view_mode {
        render_catalyst(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Hyperspace = app_state.view_mode {
        render_hyperspace(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Hologram = app_state.view_mode {
        render_hologram(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Weaver = app_state.view_mode {
        render_weaver(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Attractor = app_state.view_mode {
        render_attractor(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Virology = app_state.view_mode {
        render_virology(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::BioMesh = app_state.view_mode {
        render_biomesh(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Reactor = app_state.view_mode {
        render_reactor(f, vm, app_state);
        return;
    }

    if let ViewMode::Evolution = app_state.view_mode {
        render_evolution(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Ecology = app_state.view_mode {
        render_ecology(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Savant = app_state.view_mode {
        render_savant(f, vm, app_state);
        return;
    }

    #[cfg(feature = "nova")]
    if let ViewMode::Prolouge = app_state.view_mode {
        crate::tui::views::magic::render_prolouge(f, vm, app_state);
        return;
    }

    match app_state.view_mode {
        ViewMode::Genome => crate::tui::views::core::render_genome_and_grid(f, vm, app_state),
        ViewMode::Grid => crate::tui::views::core::render_genome_and_grid(f, vm, app_state),
        ViewMode::Sequencer => render_sequencer(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Heatmap => render_heatmap(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Paradox => render_paradox(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Codex => render_codex(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Choir => render_choir(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Genesis => render_genesis(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Terminal => render_terminal(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Forge => render_forge(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Prologue => render_prologue(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Verbum => render_verbum(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Crispr => render_crispr(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Lexicon => render_lexicon(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Narrative => render_narrative(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Akashic => render_akashic(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Cambrian => render_cambrian(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Biolum => render_biolum(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::LifeCycle => render_lifecycle(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Tesseract => render_tesseract(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Metazoa => render_metazoa(f, vm, app_state),
        ViewMode::Mutagen => render_mutagen(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Fractal => render_fractal(f, vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Semiotics => render_semiotics(f, vm, app_state),
        _ => crate::tui::views::core::render_genome_and_grid(f, vm, app_state), // Fallback
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
