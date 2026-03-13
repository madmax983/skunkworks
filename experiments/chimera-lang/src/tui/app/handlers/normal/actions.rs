use crate::vm::ChimeraVM;
use crate::tui::state::{AppState, InputMode, ViewMode};
use anyhow::Result;
use crossterm::event::KeyCode;

pub(crate) fn handle_action_input(key_code: KeyCode, vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    match key_code {
        KeyCode::Tab => {
                        if let ViewMode::Evolution = app_state.view_mode {
                            use crate::vm::evolution::Challenge;
                            app_state.evolution_state.challenge =
                                match app_state.evolution_state.challenge {
                                    Challenge::Target(_) => Challenge::Doubler,
                                    Challenge::Doubler => Challenge::Adder,
                                    Challenge::Adder => Challenge::Fibonacci,
                                    Challenge::Fibonacci => Challenge::Target(42),
                                    Challenge::Custom(_) => Challenge::Target(42), // Fallback/Cycle
                                };
                            if let Some(engine) = &mut app_state.evolution_state.engine {
                                engine.challenge = app_state.evolution_state.challenge.clone();
                            }
                            app_state.status_msg =
                                format!("Challenge set to {}", app_state.evolution_state.challenge);
                            return Ok(());
                        }

                        #[cfg(feature = "nova")]
                        if let ViewMode::Genesis = app_state.view_mode {
                            app_state.genesis_focus = (app_state.genesis_focus + 1) % 3;
                            return Ok(());
                        }

                        #[cfg(feature = "nova")]
                        if let ViewMode::Forge = app_state.view_mode {
                            app_state.forge_focus = (app_state.forge_focus + 1) % 3;
                            return Ok(());
                        }

                        app_state.view_mode = match app_state.view_mode {
                            ViewMode::Genome => ViewMode::Grid,
                            ViewMode::Grid => ViewMode::Microscope,
                            ViewMode::Microscope => {
                                #[cfg(feature = "biophysics")]
                                {
                                    ViewMode::Cortex
                                }
                                #[cfg(not(feature = "biophysics"))]
                                {
                                    #[cfg(feature = "resonance")]
                                    {
                                        ViewMode::Resonance
                                    }
                                    #[cfg(not(feature = "resonance"))]
                                    {
                                        #[cfg(feature = "nova")]
                                        {
                                            ViewMode::Grimoire
                                        }
                                        #[cfg(not(feature = "nova"))]
                                        {
                                            ViewMode::Heatmap
                                        }
                                    }
                                }
                            }
                            #[cfg(feature = "biophysics")]
                            ViewMode::Cortex => {
                                #[cfg(feature = "resonance")]
                                {
                                    ViewMode::Resonance
                                }
                                #[cfg(not(feature = "resonance"))]
                                {
                                    #[cfg(feature = "nova")]
                                    {
                                        ViewMode::Grimoire
                                    }
                                    #[cfg(not(feature = "nova"))]
                                    {
                                        ViewMode::Heatmap
                                    }
                                }
                            }
                            #[cfg(feature = "resonance")]
                            ViewMode::Resonance => {
                                #[cfg(feature = "nova")]
                                {
                                    ViewMode::Grimoire
                                }
                                #[cfg(not(feature = "nova"))]
                                {
                                    ViewMode::Heatmap
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Grimoire => ViewMode::Topology,
                            #[cfg(feature = "nova")]
                            ViewMode::Topology => ViewMode::Graveyard,
                            #[cfg(feature = "nova")]
                            ViewMode::Graveyard => ViewMode::PianoRoll,
                            #[cfg(feature = "nova")]
                            ViewMode::PianoRoll => ViewMode::Retina,
                            #[cfg(feature = "nova")]
                            ViewMode::Retina => ViewMode::Quantum,
                            #[cfg(feature = "nova")]
                            ViewMode::Quantum => ViewMode::Dream,
                            #[cfg(feature = "nova")]
                            ViewMode::Dream => ViewMode::Phylogeny,
                            #[cfg(feature = "nova")]
                            ViewMode::Phylogeny => ViewMode::Alchemy,
                            #[cfg(feature = "nova")]
                            ViewMode::Alchemy => ViewMode::Memetics,
                            #[cfg(feature = "nova")]
                            ViewMode::Memetics => ViewMode::Egregore,
                            #[cfg(feature = "nova")]
                            ViewMode::Egregore => ViewMode::Bestiary,
                            #[cfg(feature = "nova")]
                            ViewMode::Bestiary => ViewMode::Kaleidoscope,
                            #[cfg(feature = "nova")]
                            ViewMode::Kaleidoscope => ViewMode::Void,
                            #[cfg(feature = "nova")]
                            ViewMode::Void => ViewMode::Signals,
                            #[cfg(feature = "nova")]
                            ViewMode::Signals => ViewMode::Sovereignty,
                            #[cfg(feature = "nova")]
                            ViewMode::Sovereignty => ViewMode::Spectrogram,
                            #[cfg(feature = "nova")]
                            ViewMode::Spectrogram => ViewMode::Market,
                            #[cfg(feature = "nova")]
                            ViewMode::Market => ViewMode::Ballistics,
                            #[cfg(feature = "nova")]
                            ViewMode::Ballistics => ViewMode::Scent,
                            #[cfg(feature = "nova")]
                            ViewMode::Scent => ViewMode::Fishing,
                            #[cfg(feature = "nova")]
                            ViewMode::Fishing => ViewMode::Arena,
                            #[cfg(feature = "nova")]
                            ViewMode::Arena => ViewMode::Garden,
                            #[cfg(feature = "nova")]
                            ViewMode::Garden => ViewMode::Orca,
                            #[cfg(feature = "nova")]
                            ViewMode::Orca => ViewMode::Babel,
                            #[cfg(feature = "nova")]
                            ViewMode::Babel => ViewMode::Strings,
                            #[cfg(feature = "nova")]
                            ViewMode::Strings => ViewMode::Quipu,
                            #[cfg(feature = "nova")]
                            ViewMode::Quipu => ViewMode::Hydra,
                            #[cfg(feature = "nova")]
                            ViewMode::Hydra => ViewMode::Chronos,
                            #[cfg(feature = "nova")]
                            ViewMode::Chronos => ViewMode::Logos,
                            #[cfg(feature = "nova")]
                            ViewMode::Logos => ViewMode::Pandemonium,
                            #[cfg(feature = "nova")]
                            ViewMode::Pandemonium => ViewMode::BioticChaos,
                            ViewMode::BioticChaos => ViewMode::Catalyst,
                            ViewMode::Catalyst => ViewMode::Heatmap,
                            ViewMode::Heatmap => {
                                #[cfg(feature = "silicon")]
                                {
                                    ViewMode::Schematic
                                }
                                #[cfg(not(feature = "silicon"))]
                                {
                                    #[cfg(feature = "elektra")]
                                    {
                                        ViewMode::Elektra
                                    }
                                    #[cfg(not(feature = "elektra"))]
                                    {
                                        #[cfg(feature = "nova")]
                                        {
                                            ViewMode::Laboratory
                                        }
                                        #[cfg(not(feature = "nova"))]
                                        {
                                            ViewMode::Genome
                                        }
                                    }
                                }
                            }
                            #[cfg(feature = "silicon")]
                            ViewMode::Schematic => ViewMode::Foundry,
                            #[cfg(feature = "silicon")]
                            ViewMode::Foundry => {
                                #[cfg(feature = "elektra")]
                                {
                                    ViewMode::Elektra
                                }
                                #[cfg(not(feature = "elektra"))]
                                {
                                    #[cfg(feature = "nova")]
                                    {
                                        ViewMode::Laboratory
                                    }
                                    #[cfg(not(feature = "nova"))]
                                    {
                                        ViewMode::Genome
                                    }
                                }
                            }
                            #[cfg(feature = "elektra")]
                            ViewMode::Elektra => {
                                #[cfg(feature = "nova")]
                                {
                                    ViewMode::Laboratory
                                }
                                #[cfg(not(feature = "nova"))]
                                {
                                    ViewMode::Genome
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Laboratory => ViewMode::Genome,
                            #[cfg(feature = "nova")]
                            ViewMode::Hyperspace => ViewMode::Genome,
                            #[cfg(feature = "nova")]
                            ViewMode::Hologram => ViewMode::Genome,
                            #[cfg(feature = "nova")]
                            ViewMode::Weaver => ViewMode::Terminal,
                            #[cfg(feature = "nova")]
                            ViewMode::Terminal => ViewMode::Attractor,
                            #[cfg(feature = "nova")]
                            ViewMode::Attractor => ViewMode::Virology,
                            #[cfg(feature = "nova")]
                            ViewMode::Virology => ViewMode::BioMesh,
                            #[cfg(feature = "nova")]
                            ViewMode::BioMesh => ViewMode::Crispr,
                            #[cfg(feature = "nova")]
                            ViewMode::Crispr => ViewMode::Reactor,
                            #[cfg(feature = "nova")]
                            ViewMode::Reactor => ViewMode::Biolum,
                            #[cfg(feature = "nova")]
                            ViewMode::Biolum => ViewMode::Evolution,
                            ViewMode::Evolution => {
                                #[cfg(feature = "nova")]
                                {
                                    ViewMode::Ecology
                                }
                                #[cfg(not(feature = "nova"))]
                                {
                                    ViewMode::Genome
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Ecology => ViewMode::LifeCycle,
                            #[cfg(feature = "nova")]
                            ViewMode::Semiotics => ViewMode::Genome,
                            #[cfg(feature = "nova")]
                            ViewMode::Fractal => ViewMode::Genome,
                            #[cfg(feature = "nova")]
                            ViewMode::LifeCycle => ViewMode::Metazoa,
                            #[cfg(feature = "nova")]
                            ViewMode::Metazoa => ViewMode::Genesis,
                            #[cfg(feature = "nova")]
                            ViewMode::Genesis => ViewMode::Genome,
                            #[cfg(feature = "nova")]
                            ViewMode::Cambrian => ViewMode::Savant,
                            #[cfg(feature = "nova")]
                            ViewMode::Savant => ViewMode::Akashic,
                            #[cfg(feature = "nova")]
                            ViewMode::Akashic => ViewMode::Prologue,
                            #[cfg(feature = "nova")]
                            ViewMode::Prologue => ViewMode::Lexicon,
                            #[cfg(feature = "nova")]
                            ViewMode::Lexicon => ViewMode::Narrative,
                            #[cfg(feature = "nova")]
                            ViewMode::Narrative => ViewMode::Sequencer,
                            ViewMode::Sequencer => ViewMode::Mutagen,
                            #[cfg(feature = "nova")]
                            ViewMode::Mutagen => ViewMode::Forge,
                            ViewMode::Forge => {
                                #[cfg(feature = "nova")]
                                {
                                    ViewMode::Tesseract
                                }
                                #[cfg(not(feature = "nova"))]
                                {
                                    ViewMode::Genome
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Tesseract => ViewMode::Genome,
                            #[cfg(not(feature = "nova"))]
                            ViewMode::Tesseract => ViewMode::Choir,
                            #[cfg(feature = "nova")]
                            ViewMode::Choir => ViewMode::Paradox,
                            #[cfg(feature = "nova")]
                            ViewMode::Paradox => ViewMode::Codex,
                            #[cfg(feature = "nova")]
                            ViewMode::Codex => ViewMode::Verbum,
                            ViewMode::Verbum => ViewMode::Genome,
                        };
                    }
        _ => {}
    }
    Ok(false)
}
