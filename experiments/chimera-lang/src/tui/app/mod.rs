pub(crate) mod handlers;

use crate::vm::ChimeraVM;
use anyhow::Result;
use crossterm::event::{self, Event};
use ratatui::Terminal;

use super::state::*;
use super::views::*;
use handlers::*;

pub(crate) fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<()>
where
    <B as ratatui::backend::Backend>::Error: Send + Sync + 'static,
{
    loop {
        // Hot Reload Check
        app_state.last_check_tick = app_state.last_check_tick.wrapping_add(1);
        if app_state.last_check_tick.is_multiple_of(10) {
            if let Some(path) = &app_state.source_path {
                if let Ok(metadata) = std::fs::metadata(path) {
                    if let Ok(modified) = metadata.modified() {
                        let should_reload = match app_state.last_modified {
                            Some(last) => modified > last,
                            None => true,
                        };

                        if should_reload {
                            let mut src = String::new();
                            if std::fs::File::open(path).and_then(|mut f| {
                                use std::io::Read;
                                f.take(10 * 1024 * 1024).read_to_string(&mut src)
                            }).is_ok() {
                                // Default to ChimeraScript for hot reload for now
                                // Ideally we check extension, but compile() handles imports
                                let parent = path.parent();
                                if let Ok(new_dna) = crate::compiler::compile(&src, parent) {
                                    vm.patch_dna(new_dna);
                                    app_state.last_modified = Some(modified);
                                    app_state.status_msg = "Hot Reloaded!".to_string();
                                    app_state.screen_shake = 5.0;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Process TuiEvents
        for event in vm.tui_events.drain(..) {
            match event {
                crate::vm::TuiEvent::Glitch(v) => vm.glitch_level = v,
                crate::vm::TuiEvent::Shake(v) => app_state.screen_shake = v,
                crate::vm::TuiEvent::Message(s) => app_state.status_msg = s,
            }
        }

        // Decay Shake
        if app_state.screen_shake > 0.0 {
            app_state.screen_shake *= 0.9;
            if app_state.screen_shake < 0.1 {
                app_state.screen_shake = 0.0;
            }
        }

        let size = terminal.size()?;
        app_state.matrix_rain.update(size.width, size.height);

        if let ViewMode::Evolution = app_state.view_mode {
            if app_state.evolution_state.auto_run {
                if let Some(engine) = &mut app_state.evolution_state.engine {
                    engine.step(vm);
                }
            }
        }

        if let ViewMode::Sequencer = app_state.view_mode {
            if app_state.sequencer_state.playing {
                app_state.sequencer_state.tick += 1;

                #[cfg(feature = "resonance")]
                {
                    if let Some(tx) = &vm.audio_tx {
                        let tick = app_state.sequencer_state.tick;
                        for strand in &vm.dna.helix.strands {
                            if tick < strand.genes.len() {
                                let gene = &strand.genes[tick];
                                let freq = match gene.op {
                                    crate::opcode::OpCode::Push => 110.0,
                                    crate::opcode::OpCode::Add => 220.0,
                                    crate::opcode::OpCode::Sub => 440.0,
                                    crate::opcode::OpCode::Jump => 55.0,
                                    _ => gene.op.to_string().len() as f32 * 50.0 + 200.0,
                                };

                                use resonance_audio::audio::AudioCommand;
                                let _ = tx.send(AudioCommand::Tone {
                                    x: 0,
                                    y: 0,
                                    frequency: freq,
                                    strength: 0.5,
                                    duration_ms: 100,
                                });
                            }
                        }
                    }
                }
            }
        }

        #[cfg(feature = "nova")]
        if let ViewMode::Fishing = app_state.view_mode {
            if app_state.fishing_cast {
                // Bobber float animation
                app_state.fishing_bobber_y += (rand::random::<f64>() - 0.5) * 0.5;

                // Random Hook
                if !app_state.fishing_hooked && rand::random::<f64>() < 0.01 {
                    app_state.fishing_hooked = true;
                    app_state.status_msg = "FISH HOOKED! REEL IT IN!".to_string();
                }

                if app_state.fishing_hooked {
                    // Fish fights back (Randomly pulls)
                    if rand::random::<f64>() < 0.2 {
                        app_state.fishing_tension += 0.02;
                    } else {
                        // Passive decay when not being pulled
                        app_state.fishing_tension -= 0.002;
                    }
                    app_state.fishing_fish_y =
                        app_state.fishing_bobber_y + (rand::random::<f64>() - 0.5) * 2.0;
                } else {
                    app_state.fishing_tension -= 0.01;
                }

                // Clamp tension
                app_state.fishing_tension = app_state.fishing_tension.clamp(0.0, 1.1); // Allow slight over for snap check

                if app_state.fishing_tension >= 1.0 {
                    app_state.status_msg = "SNAP! Line broke.".to_string();
                    app_state.fishing_cast = false;
                    app_state.fishing_hooked = false;
                    app_state.fishing_tension = 0.0;
                    app_state.screen_shake = 2.0;
                }
            }
        }

        #[cfg(feature = "biophysics")]
        if let Some(coord) = app_state.selected_neuron_coords {
            if let Some(neuron) = vm.neurons.get(&coord) {
                // Push voltage (mapped to u64 for Sparkline)
                // V is approx -100 to +50. Shift by +100.
                let v_norm = (neuron.v + 100.0).clamp(0.0, 200.0) as u64;
                if app_state.voltage_history.len() >= 100 {
                    app_state.voltage_history.remove(0);
                }
                app_state.voltage_history.push(v_norm);
            }
        }

        terminal.draw(|f| {
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

            match app_state.view_mode {
                ViewMode::Genome => render_genome(f, vm, app_state),
                ViewMode::Grid => render_grid(f, vm, app_state),
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
                _ => render_grid(f, vm, app_state), // Fallback
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
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if app_state.show_view_selector {
                    if handle_view_selector(key, app_state) {
                        continue;
                    }
                }

                if let InputMode::Editing = app_state.input_mode {
                    if handle_editing_input(key, vm, app_state)? {
                        continue;
                    }
                } else {
                    if handle_normal_input(key, vm, app_state)? {
                        continue;
                    }
                }
            }
        }
    }
}
