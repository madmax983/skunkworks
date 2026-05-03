use std::io::Read;
pub(crate) mod handlers;
pub(crate) mod router;

use crate::vm::ChimeraVM;
use anyhow::Result;

fn check_hot_reload(vm: &mut ChimeraVM, app_state: &mut AppState) {
    let path = match &app_state.source_path {
        Some(p) => p,
        None => return,
    };

    let metadata = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return,
    };

    let modified = match metadata.modified() {
        Ok(m) => m,
        Err(_) => return,
    };

    let should_reload = match app_state.last_modified {
        Some(last) => modified > last,
        None => true,
    };

    if !should_reload {
        return;
    }

    let file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return,
    };

    let mut src = String::new();
    let limit = 1024 * 1024; // 1MB limit
    if let Ok(bytes) =
        file.take(limit + 1).read_to_string(&mut src)
    {
        if bytes as u64 <= limit {
            let parent = path.parent();
            if let Ok(new_dna) = crate::compiler::compile(&src, parent) {
                vm.patch_dna(new_dna);
                app_state.last_modified = Some(modified);
                app_state.status_msg = "Hot Reloaded!".to_string();
                app_state.screen_shake = 5.0;
            }
        } else {
            app_state.status_msg = "File too large to hot reload!".to_string();
        }
    }
}
use crossterm::event::{self, Event};
use ratatui::Terminal;

use super::state::*;

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
            check_hot_reload(vm, app_state);
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
            crate::tui::app::router::route_view(f, vm, app_state);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if app_state.show_view_selector && handle_view_selector(key, app_state) {
                    continue;
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
