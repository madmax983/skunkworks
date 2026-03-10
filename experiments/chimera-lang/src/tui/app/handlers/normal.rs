use crate::ast::Gene;
use crate::vm::ChimeraVM;
use crate::{ChimeraParser, Rule};
use super::super::state::{AppState, InputMode, ViewMode};
use super::super::get_all_views;
use super::super::parse_grid_value;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use pest::Parser;

pub(crate) fn handle_normal_input(
    key: KeyEvent,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
                #[cfg(feature = "nova")]
                if let KeyCode::Char(c) = key.code {
                    if app_state.view_mode == ViewMode::Orca
                        || app_state.view_mode == ViewMode::Prologue
                    {
                        if c == ' ' {
                            // Let Space fall through
                        } else if c.is_ascii_graphic() {
                            let (x, y) = app_state.grid_cursor;
                            vm.grid[y][x] = crate::vm::Value::Str(c.to_string());
                            return Ok(true);
                        }
                    }

                    if c != 'q'
                        && c != ' '
                        && c != 'm'
                        && c != 'c'
                        && c != 'C'
                        && c != 'K'
                        && vm.handle_input(c)
                    {
                        return Ok(true);
                    }
                }

                match key.code {
                    KeyCode::Char('K') => {
                        #[cfg(feature = "nova")]
                        {
                            if let ViewMode::Ecology = app_state.view_mode {
                                vm.organelles.clear();
                                app_state.status_msg = "Extinction Event.".to_string();
                            } else {
                                app_state.view_mode = ViewMode::Choir;
                                app_state.status_msg = "Switched to Choir View".to_string();
                            }
                        }
                    }
                    KeyCode::Char('C') => {
                        app_state.chaos_mode = !app_state.chaos_mode;
                        app_state.status_msg = format!("Chaos Mode: {}", app_state.chaos_mode);
                    }
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
                    #[cfg(feature = "nova")]
                    KeyCode::Char('^') => app_state.view_mode = ViewMode::Cambrian,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('&') => app_state.view_mode = ViewMode::Semiotics,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('*') => app_state.view_mode = ViewMode::Fractal,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('y') => app_state.view_mode = ViewMode::LifeCycle,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('|') => app_state.view_mode = ViewMode::Savant,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('#') => app_state.view_mode = ViewMode::Akashic,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('\\') => app_state.view_mode = ViewMode::Prologue,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('6') => app_state.view_mode = ViewMode::Lexicon,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('N') => app_state.view_mode = ViewMode::Narrative,
                    KeyCode::Char('h') => app_state.view_mode = ViewMode::Heatmap,
                    #[cfg(feature = "silicon")]
                    KeyCode::Char('F') => app_state.view_mode = ViewMode::Foundry,
                    #[cfg(feature = "elektra")]
                    KeyCode::Char('E') => app_state.view_mode = ViewMode::Elektra,
                    KeyCode::Char('p') => {
                        if let ViewMode::Grid = app_state.view_mode {
                            app_state.palette_open = !app_state.palette_open;
                        } else {
                            #[cfg(feature = "nova")]
                            {
                                app_state.view_mode = ViewMode::PianoRoll;
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('5') => {
                        if let ViewMode::Pandemonium = app_state.view_mode {
                            app_state.pandemonium_selected_tool = 4;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('z') => app_state.view_mode = ViewMode::Bestiary,
                    KeyCode::Char('i') => {
                        app_state.input_mode = InputMode::Injection;
                        app_state.input_buffer.clear();
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('r') => {
                        if let ViewMode::Graveyard = app_state.view_mode {
                            match vm.resurrect_from_graveyard(app_state.selected_graveyard_strand) {
                                Ok(idx) => {
                                    app_state.status_msg = format!("Resurrected strand {}!", idx);
                                    if app_state.selected_graveyard_strand >= vm.graveyard.len()
                                        && !vm.graveyard.is_empty()
                                    {
                                        app_state.selected_graveyard_strand =
                                            vm.graveyard.len() - 1;
                                    }
                                }
                                Err(e) => app_state.status_msg = format!("Error: {}", e),
                            }
                        }
                    }
                    #[cfg(feature = "biophysics")]
                    KeyCode::Char('b') => app_state.view_mode = ViewMode::Cortex,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('a') => {
                        if let ViewMode::Evolution = app_state.view_mode {
                            app_state.evolution_state.auto_run =
                                !app_state.evolution_state.auto_run;
                        } else if let ViewMode::Alchemy = app_state.view_mode {
                            // Add to Crucible
                            match app_state.alchemy_selection {
                                0 => {
                                    // Shelf
                                    let elements = [
                                        "Fire", "Water", "Earth", "Air", "Life", "Death", "Lead",
                                        "Energy",
                                    ];
                                    if app_state.alchemy_shelf_idx < elements.len() {
                                        vm.crucible.add(crate::vm::Value::Str(
                                            elements[app_state.alchemy_shelf_idx].to_string(),
                                        ));
                                    }
                                }
                                1 => {
                                    // Strands
                                    if app_state.alchemy_strand_idx < vm.dna.helix.strands.len() {
                                        vm.crucible.add(crate::vm::Value::Int(
                                            app_state.alchemy_strand_idx as i64,
                                        ));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('x') => {
                        if let ViewMode::Graveyard = app_state.view_mode {
                            if app_state.selected_graveyard_strand < vm.graveyard.len() {
                                vm.graveyard.remove(app_state.selected_graveyard_strand);
                                app_state.status_msg = "Exterminated strand.".to_string();
                                if app_state.selected_graveyard_strand >= vm.graveyard.len()
                                    && !vm.graveyard.is_empty()
                                {
                                    app_state.selected_graveyard_strand = vm.graveyard.len() - 1;
                                }
                            }
                        } else if let ViewMode::Alchemy = app_state.view_mode {
                            vm.crucible.clear();
                            app_state.status_msg = "Crucible emptied.".to_string();
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('t') => {
                        if let ViewMode::Alchemy = app_state.view_mode {
                            crate::vm::alchemy::transmute_crucible(vm);
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('k') => app_state.view_mode = ViewMode::Kaleidoscope,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('$') => app_state.view_mode = ViewMode::Market,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('!') => app_state.view_mode = ViewMode::Ballistics,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('~') => app_state.view_mode = ViewMode::Scent,
                    KeyCode::Char('e') => {
                        if let ViewMode::Genome = app_state.view_mode {
                            if let Some(strand) =
                                vm.dna.helix.strands.get(app_state.selected_strand)
                            {
                                let engine = crate::vm::evolution::EvolutionEngine::new(
                                    strand.clone(),
                                    20, // Population
                                    app_state.evolution_state.challenge.clone(),
                                );
                                app_state.evolution_state.engine = Some(engine);
                                app_state.view_mode = ViewMode::Evolution;
                                app_state.status_msg = "Evolution Initialized".to_string();
                            }
                        }
                    }
                    KeyCode::Char('f') => {
                        #[cfg(feature = "nova")]
                        if let ViewMode::Ecology = app_state.view_mode {
                            crate::vm::nova_ecology::spawn_food(vm);
                            app_state.status_msg = "Food spawned.".to_string();
                            return Ok(true);
                        }

                        #[cfg(feature = "silicon")]
                        if let ViewMode::Foundry = app_state.view_mode {
                            // Fabricate current strand
                            let (x, y) = app_state.grid_cursor;
                            let s_idx = app_state.selected_strand;
                            vm.stack.push(crate::vm::Value::Int(s_idx as i64));
                            vm.stack.push(crate::vm::Value::Int(y as i64));
                            vm.stack.push(crate::vm::Value::Int(x as i64));
                            crate::vm::silicon::exec_silicon_op(
                                vm,
                                crate::opcode::OpCode::Fabricate,
                                &[],
                            );
                            app_state.status_msg =
                                format!("Fabricated strand {} at {},{}", s_idx, x, y);
                            return Ok(true);
                        }

                        #[cfg(feature = "nova")]
                        {
                            app_state.view_mode = ViewMode::Fishing;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('V') => app_state.view_mode = ViewMode::Arena,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('G') => {
                        if let ViewMode::Babel = app_state.view_mode {
                            if let Some(ast) = &app_state.babel_ast {
                                let s = crate::vm::babel::generate_string(ast);
                                app_state.babel_result = s;
                            } else {
                                app_state.status_msg = "No Grammar to Generate from".to_string();
                            }
                        } else {
                            app_state.view_mode = ViewMode::Garden;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('l') => app_state.view_mode = ViewMode::Biolum,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('L') => app_state.view_mode = ViewMode::Babel,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('=') => app_state.view_mode = ViewMode::Strings,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('Y') => app_state.view_mode = ViewMode::Hydra,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('T') => {
                        if let ViewMode::Babel = app_state.view_mode {
                            if let Some(ast) = &app_state.babel_ast {
                                vm.stack.push(ast.clone());
                                vm.stack
                                    .push(crate::vm::Value::Str(app_state.babel_input.clone()));
                                crate::vm::babel::exec_babel_op(
                                    vm,
                                    crate::opcode::OpCode::Tongue,
                                    &[],
                                );
                                if let Some(res) = vm.stack.pop() {
                                    app_state.babel_result = format!("{}", res);
                                }
                            }
                        } else {
                            app_state.view_mode = ViewMode::Chronos;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('U') => app_state.view_mode = ViewMode::Logos,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('P') => app_state.view_mode = ViewMode::Pandemonium,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('H') => app_state.view_mode = ViewMode::Hyperspace,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('W') => app_state.view_mode = ViewMode::Weaver,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('`') => app_state.view_mode = ViewMode::Terminal,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('A') => app_state.view_mode = ViewMode::Attractor,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('v') => app_state.view_mode = ViewMode::Virology,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('B') => app_state.view_mode = ViewMode::BioMesh,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('X') => app_state.view_mode = ViewMode::Reactor,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('I') => {
                        if let ViewMode::Hologram = app_state.view_mode {
                            // Interfere (DNA -> Hologram)
                            let idx = app_state.selected_strand;
                            vm.stack.push(crate::vm::Value::Int(idx as i64));
                            crate::vm::nova_hologram::exec_interfere(
                                vm,
                                crate::opcode::OpCode::Interfere,
                                &[],
                            );
                            app_state.status_msg = format!("Interfered strand {}", idx);
                        } else if let ViewMode::Ecology = app_state.view_mode {
                            app_state.input_mode = InputMode::Editing;
                            app_state.input_buffer.clear();
                            app_state.status_msg = "Injecting Gene... (Type & Enter)".to_string();
                        } else {
                            app_state.view_mode = ViewMode::Hologram;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('O') => {
                        if let ViewMode::Hologram = app_state.view_mode {
                            // Refract (Hologram -> DNA)
                            crate::vm::nova_hologram::exec_refract(
                                vm,
                                crate::opcode::OpCode::Refract,
                                &[],
                            );
                        } else {
                            app_state.view_mode = ViewMode::Orca;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('+') => {
                        if let ViewMode::Hologram = app_state.view_mode {
                            let (x, y) = app_state.grid_cursor;
                            vm.hologram_grid[y][x].0 += 0.1;
                            vm.hologram_grid[y][x].1 += 0.1;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('-') => {
                        if let ViewMode::Hologram = app_state.view_mode {
                            let (x, y) = app_state.grid_cursor;
                            vm.hologram_grid[y][x].0 -= 0.1;
                            vm.hologram_grid[y][x].1 -= 0.1;
                        }
                    }
                    #[cfg(all(feature = "oracle", feature = "nova"))]
                    KeyCode::Char('/') => {
                        if let ViewMode::Grimoire = app_state.view_mode {
                            app_state.query_mode = true;
                            app_state.query_input.clear();
                            app_state.query_results.clear();
                        }
                    }
                    KeyCode::Char('?') => {
                        app_state.show_view_selector = !app_state.show_view_selector;
                        // Reset index when opening
                        if app_state.show_view_selector {
                            app_state.view_selector_state.borrow_mut().select(Some(0));
                        }
                    }
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => {
                        if let ViewMode::Evolution = app_state.view_mode {
                            if let Some(engine) = &mut app_state.evolution_state.engine {
                                engine.step(vm);
                            }
                            return Ok(true);
                        }

                        #[cfg(feature = "nova")]
                        if let ViewMode::Babel = app_state.view_mode {
                            // Run Parse
                            vm.stack
                                .push(crate::vm::Value::Str(app_state.babel_pattern.clone()));
                            let _ = crate::vm::babel::exec_babel_op(
                                vm,
                                crate::opcode::OpCode::ParserRegex,
                                &[],
                            );
                            vm.stack
                                .push(crate::vm::Value::Str(app_state.babel_input.clone()));
                            let _ = crate::vm::babel::exec_babel_op(
                                vm,
                                crate::opcode::OpCode::Parse,
                                &[],
                            );

                            if let Some(res) = vm.stack.pop() {
                                app_state.babel_result = format!("{}", res);
                            } else {
                                app_state.babel_result = "Stack Empty/Error".to_string();
                            }
                            return Ok(true);
                        }

                        if let ViewMode::Grid = app_state.view_mode {
                            if let Some(c) = app_state.palette_char {
                                let (x, y) = app_state.grid_cursor;
                                vm.grid[y][x] = crate::vm::Value::Str(c.to_string());
                            } else {
                                vm.step();
                            }
                        } else {
                            #[cfg(feature = "nova")]
                            if let ViewMode::Arena = app_state.view_mode {
                                if let Some(arena) = &mut vm.arena {
                                    arena.tick();
                                }
                            } else if let ViewMode::Pandemonium = app_state.view_mode {
                                let cx = app_state.pandemonium_cursor.0;
                                let cy = app_state.pandemonium_cursor.1;

                                let mut best_dist = 1.0;
                                let mut target = None;

                                let mut linear_idx = 0;
                                for (s_idx, strand) in vm.dna.helix.strands.iter().enumerate() {
                                    for (g_idx, _) in strand.genes.iter().enumerate() {
                                        let t = (linear_idx as f64) * 0.1;
                                        let gr = t * 0.5;
                                        let gx = gr * t.cos();
                                        let gy = gr * t.sin();

                                        let dist = ((gx - cx).powi(2) + (gy - cy).powi(2)).sqrt();
                                        if dist < best_dist {
                                            best_dist = dist;
                                            target = Some((s_idx, g_idx));
                                        }
                                        linear_idx += 1;
                                    }
                                }

                                if let Some((s, g)) = target {
                                    match app_state.pandemonium_selected_tool {
                                        0 => crate::vm::pandemonium::apply_mutation(vm, s, g),
                                        1 => crate::vm::pandemonium::apply_scramble(
                                            vm,
                                            s,
                                            g,
                                            app_state.pandemonium_radius,
                                        ),
                                        2 => crate::vm::pandemonium::apply_purge(
                                            vm,
                                            s,
                                            g,
                                            app_state.pandemonium_radius,
                                        ),
                                        3 => crate::vm::pandemonium::apply_duplicate(vm, s, g),
                                        4 => crate::vm::pandemonium::apply_storm(
                                            vm,
                                            s,
                                            g,
                                            app_state.pandemonium_radius,
                                        ),
                                        _ => {}
                                    }
                                    app_state.status_msg =
                                        format!("Pandemonium applied at {},{}", s, g);
                                }
                            } else if let ViewMode::Fishing = app_state.view_mode {
                                if app_state.fishing_cast {
                                    // Reel
                                    if app_state.fishing_hooked {
                                        app_state.fishing_bobber_y += 4.0;
                                        app_state.fishing_tension += 0.05; // Reeling increases tension

                                        if app_state.fishing_bobber_y > 90.0 {
                                            // Caught!
                                            app_state.status_msg = "CAUGHT A FISH!".to_string();
                                            app_state.fishing_cast = false;
                                            app_state.fishing_hooked = false;
                                            app_state.fishing_tension = 0.0;
                                            // Maybe give energy?
                                            vm.energy += 10;
                                        }
                                    } else {
                                        // Just pull empty line
                                        app_state.fishing_cast = false;
                                        app_state.status_msg = "Reeled in empty.".to_string();
                                    }
                                } else {
                                    // Cast
                                    app_state.fishing_cast = true;
                                    app_state.fishing_bobber_y = 50.0;
                                    app_state.fishing_tension = 0.0;
                                    app_state.status_msg = "Casted line...".to_string();
                                }
                            } else if let ViewMode::Kaleidoscope = app_state.view_mode {
                                // Paint
                                let (x, y) = app_state.grid_cursor;
                                let r = match app_state.kaleidoscope_hue_idx {
                                    0 => 255,
                                    1 => 255,
                                    2 => 0,
                                    3 => 0,
                                    4 => 0,
                                    5 => 255,
                                    _ => 255,
                                };
                                let g = match app_state.kaleidoscope_hue_idx {
                                    0 => 0,
                                    1 => 255,
                                    2 => 255,
                                    3 => 255,
                                    4 => 0,
                                    5 => 0,
                                    _ => 255,
                                };
                                let b = match app_state.kaleidoscope_hue_idx {
                                    0 => 0,
                                    1 => 0,
                                    2 => 0,
                                    3 => 255,
                                    4 => 255,
                                    5 => 255,
                                    _ => 255,
                                };

                                // Adjust for lightness (Light=0, Normal=1, Dark=2)
                                let (r, g, b) = match app_state.kaleidoscope_light_idx {
                                    0 => (r + (255 - r) / 2, g + (255 - g) / 2, b + (255 - b) / 2), // Light
                                    2 => (r / 2, g / 2, b / 2), // Dark
                                    _ => (r, g, b),             // Normal
                                };

                                vm.chroma_grid[y][x].fg = Some((r as u8, g as u8, b as u8));
                            } else {
                                vm.step();
                            }
                            #[cfg(not(feature = "nova"))]
                            vm.step();
                        }
                    }
                    KeyCode::Char('s') => {
                        #[cfg(feature = "nova")]
                        if let ViewMode::Ecology = app_state.view_mode {
                            crate::vm::nova_ecology::spawn_random_ecology(vm, 10);
                            app_state.status_msg = "Spawned 10 organisms.".to_string();
                            return Ok(true);
                        } else if let ViewMode::Kaleidoscope = app_state.view_mode {
                            // Step Piet
                            if vm.piet_state.is_none() {
                                vm.piet_state = Some(crate::vm::piet::init_piet(vm));
                            }
                            if let Some(mut state) = vm.piet_state.take() {
                                crate::vm::piet::step_piet_once(vm, &mut state);
                                vm.piet_state = Some(state);
                            }
                            return Ok(true);
                        }

                        #[cfg(feature = "silicon")]
                        {
                            app_state.view_mode = ViewMode::Schematic;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('R') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            vm.piet_state = None;
                            app_state.status_msg = "Piet State Reset".to_string();
                        } else if let ViewMode::Arena = app_state.view_mode {
                            if let Some(arena) = &mut vm.arena {
                                arena.reset();
                                app_state.status_msg = "Arena Reset".to_string();
                            }
                        } else if let ViewMode::Babel = app_state.view_mode {
                            // Seed
                            app_state.babel_ast = Some(crate::vm::Value::Junction(
                                crate::ast::JunctionType::Any,
                                vec![
                                    crate::vm::Value::Str("Seq".to_string()),
                                    crate::vm::Value::Junction(
                                        crate::ast::JunctionType::Any,
                                        vec![
                                            crate::vm::Value::Str("Match".to_string()),
                                            crate::vm::Value::Str("Hello".to_string()),
                                        ],
                                    ),
                                    crate::vm::Value::Junction(
                                        crate::ast::JunctionType::Any,
                                        vec![
                                            crate::vm::Value::Str("Match".to_string()),
                                            crate::vm::Value::Str("World".to_string()),
                                        ],
                                    ),
                                ],
                            ));
                            app_state.status_msg = "Grammar Reset".to_string();
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('M') => {
                        if let ViewMode::Babel = app_state.view_mode {
                            // Initialize if needed
                            if app_state.babel_ast.is_none() {
                                // Seed: Seq(Match("A"), Match("B"))
                                app_state.babel_ast = Some(crate::vm::Value::Junction(
                                    crate::ast::JunctionType::Any,
                                    vec![
                                        crate::vm::Value::Str("Seq".to_string()),
                                        crate::vm::Value::Junction(
                                            crate::ast::JunctionType::Any,
                                            vec![
                                                crate::vm::Value::Str("Match".to_string()),
                                                crate::vm::Value::Str("A".to_string()),
                                            ],
                                        ),
                                        crate::vm::Value::Junction(
                                            crate::ast::JunctionType::Any,
                                            vec![
                                                crate::vm::Value::Str("Match".to_string()),
                                                crate::vm::Value::Str("B".to_string()),
                                            ],
                                        ),
                                    ],
                                ));
                            }

                            if let Some(ast) = &app_state.babel_ast {
                                let new_ast = crate::vm::babel::mutate_grammar(ast, 0.2); // 20% rate
                                app_state.babel_ast = Some(new_ast);
                                app_state.status_msg = "Grammar Mutated".to_string();
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('S') => {
                        if let ViewMode::Arena = app_state.view_mode {
                            if let Some(arena) = &mut vm.arena {
                                // Add random gladiators if empty
                                if arena.combatants.is_empty() {
                                    // Use some existing strands or random
                                    let mut rng = rand::thread_rng();
                                    use rand::Rng;
                                    if !vm.dna.helix.strands.is_empty() {
                                        let s1 = vm.dna.helix.strands
                                            [rng.gen_range(0..vm.dna.helix.strands.len())]
                                        .clone();
                                        let s2 = vm.dna.helix.strands
                                            [rng.gen_range(0..vm.dna.helix.strands.len())]
                                        .clone();
                                        arena.add_gladiator(s1, rng.gen());
                                        arena.add_gladiator(s2, rng.gen());
                                    }
                                }
                                arena.start();
                                app_state.status_msg = "Arena Started!".to_string();
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('1') => {
                        if let ViewMode::Pandemonium = app_state.view_mode {
                            app_state.pandemonium_selected_tool = 0;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('2') => {
                        if let ViewMode::Pandemonium = app_state.view_mode {
                            app_state.pandemonium_selected_tool = 1;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('3') => {
                        if let ViewMode::Pandemonium = app_state.view_mode {
                            app_state.pandemonium_selected_tool = 2;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('4') => {
                        if let ViewMode::Pandemonium = app_state.view_mode {
                            app_state.pandemonium_selected_tool = 3;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('[') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            if app_state.kaleidoscope_hue_idx > 0 {
                                app_state.kaleidoscope_hue_idx -= 1;
                            } else {
                                app_state.kaleidoscope_hue_idx = 5;
                            }
                        } else if let ViewMode::Pandemonium = app_state.view_mode {
                            app_state.pandemonium_radius =
                                (app_state.pandemonium_radius - 1.0).max(1.0);
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char(']') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            app_state.kaleidoscope_hue_idx =
                                (app_state.kaleidoscope_hue_idx + 1) % 6;
                        } else if let ViewMode::Pandemonium = app_state.view_mode {
                            app_state.pandemonium_radius += 1.0;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('{') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            if app_state.kaleidoscope_light_idx > 0 {
                                app_state.kaleidoscope_light_idx -= 1;
                            } else {
                                app_state.kaleidoscope_light_idx = 2;
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('}') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            app_state.kaleidoscope_light_idx =
                                (app_state.kaleidoscope_light_idx + 1) % 3;
                        }
                    }
                    KeyCode::Char('m') => vm.mutate(),
                    KeyCode::Char('c') => vm.chaos_mode = !vm.chaos_mode,
                    KeyCode::Down => match app_state.view_mode {
                        ViewMode::Genome => {
                            let s_len = vm.dna.helix.strands.len();
                            if s_len > 0 {
                                let g_len =
                                    vm.dna.helix.strands[app_state.selected_strand].genes.len();
                                if app_state.selected_gene + 1 < g_len {
                                    app_state.selected_gene += 1;
                                } else if app_state.selected_strand + 1 < s_len {
                                    app_state.selected_strand += 1;
                                    app_state.selected_gene = 0;
                                }
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Kaleidoscope => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Genesis => {
                            if app_state.genesis_focus == 2 && app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Metazoa => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Crispr => {}
                        ViewMode::Catalyst => {
                            if !vm.catalysts.is_empty()
                                && app_state.catalyst_scroll + 1 < vm.catalysts.len()
                            {
                                app_state.catalyst_scroll += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Chronos => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Logos => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Pandemonium => {
                            app_state.pandemonium_cursor.1 += 1.0;
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Void => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Signals => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Sovereignty => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Spectrogram => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Market => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Ballistics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Scent => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Fishing => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Arena => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Garden => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Orca => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Babel => {
                            app_state.babel_focus = (app_state.babel_focus + 1) % 2;
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Strings => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quipu => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Hydra => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "elektra")]
                        ViewMode::Elektra => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        ViewMode::Grid => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        ViewMode::Microscope => {}
                        #[cfg(feature = "resonance")]
                        ViewMode::Resonance => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Graveyard => {
                            if app_state.selected_graveyard_strand + 1 < vm.graveyard.len() {
                                app_state.selected_graveyard_strand += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::PianoRoll => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Retina => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quantum => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Dream => {
                            if app_state.selected_dream_trace + 1 < vm.dream_traces.len() {
                                app_state.selected_dream_trace += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Phylogeny => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Alchemy => {
                            if app_state.alchemy_selection == 0 {
                                if app_state.alchemy_shelf_idx < 7 {
                                    // 8 items
                                    app_state.alchemy_shelf_idx += 1;
                                }
                            } else if app_state.alchemy_strand_idx + 1 < vm.dna.helix.strands.len()
                            {
                                app_state.alchemy_strand_idx += 1;
                            }
                        }
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Schematic => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Foundry => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        ViewMode::BioticChaos => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Hyperspace => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Hologram => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Weaver | ViewMode::Laboratory => {
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
                        ViewMode::Grimoire => {
                            if app_state.selected_sigil_index + 1 < vm.sigil_registry.len() {
                                app_state.selected_sigil_index += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Topology => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Memetics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Egregore => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Bestiary => {
                            if !vm.organelles.is_empty()
                                && app_state.selected_organelle_index + 1 < vm.organelles.len()
                            {
                                app_state.selected_organelle_index += 1;
                            }
                        }
                        #[cfg(feature = "biophysics")]
                        ViewMode::Cortex => {
                            let mut neurons_sorted: Vec<_> = vm.neurons.keys().collect();
                            neurons_sorted.sort();
                            if let Some(current) = app_state.selected_neuron_coords {
                                if let Some(pos) =
                                    neurons_sorted.iter().position(|&c| *c == current)
                                {
                                    if pos + 1 < neurons_sorted.len() {
                                        app_state.selected_neuron_coords =
                                            Some(*neurons_sorted[pos + 1]);
                                        app_state.voltage_history.clear(); // Reset history on switch
                                    }
                                }
                            } else if !neurons_sorted.is_empty() {
                                app_state.selected_neuron_coords = Some(*neurons_sorted[0]);
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Terminal => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Attractor => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Virology => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        ViewMode::Evolution => {}
                        #[cfg(feature = "nova")]
                        ViewMode::BioMesh => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Reactor => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Biolum => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Ecology => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Up => match app_state.view_mode {
                        ViewMode::Genome => {
                            if app_state.selected_gene > 0 {
                                app_state.selected_gene -= 1;
                            } else if app_state.selected_strand > 0 {
                                app_state.selected_strand -= 1;
                                let g_len =
                                    vm.dna.helix.strands[app_state.selected_strand].genes.len();
                                if g_len > 0 {
                                    app_state.selected_gene = g_len - 1;
                                } else {
                                    app_state.selected_gene = 0;
                                }
                            }
                        }
                        ViewMode::Grid => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Crispr => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Chronos => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Logos => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Pandemonium => {
                            app_state.pandemonium_cursor.1 -= 1.0;
                        }
                        ViewMode::Microscope => {}
                        #[cfg(feature = "resonance")]
                        ViewMode::Resonance => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Grimoire => {
                            if app_state.selected_sigil_index > 0 {
                                app_state.selected_sigil_index -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Weaver | ViewMode::Laboratory => {
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
                        ViewMode::Cortex => {
                            let mut neurons_sorted: Vec<_> = vm.neurons.keys().collect();
                            neurons_sorted.sort();
                            if let Some(current) = app_state.selected_neuron_coords {
                                if let Some(pos) =
                                    neurons_sorted.iter().position(|&c| *c == current)
                                {
                                    if pos > 0 {
                                        app_state.selected_neuron_coords =
                                            Some(*neurons_sorted[pos - 1]);
                                        app_state.voltage_history.clear();
                                    }
                                }
                            } else if !neurons_sorted.is_empty() {
                                app_state.selected_neuron_coords = Some(*neurons_sorted[0]);
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Graveyard => {
                            if app_state.selected_graveyard_strand > 0 {
                                app_state.selected_graveyard_strand -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::PianoRoll => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Retina => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quantum => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Dream => {
                            if app_state.selected_dream_trace > 0 {
                                app_state.selected_dream_trace -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Phylogeny => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Alchemy => {
                            if app_state.alchemy_selection == 0 {
                                if app_state.alchemy_shelf_idx > 0 {
                                    app_state.alchemy_shelf_idx -= 1;
                                }
                            } else if app_state.alchemy_strand_idx > 0 {
                                app_state.alchemy_strand_idx -= 1;
                            }
                        }
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Schematic => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Foundry => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        ViewMode::BioticChaos => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Hyperspace => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Hologram => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Topology => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Memetics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Egregore => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Bestiary => {
                            if app_state.selected_organelle_index > 0 {
                                app_state.selected_organelle_index -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Kaleidoscope => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Void => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Signals => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Sovereignty => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Spectrogram => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Market => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Ballistics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Scent => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Fishing => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Arena => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Garden => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Orca => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Babel => {
                            if app_state.babel_focus > 0 {
                                app_state.babel_focus -= 1;
                            } else {
                                app_state.babel_focus = 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Strings => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quipu => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Hydra => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        ViewMode::Catalyst => {
                            if app_state.catalyst_scroll > 0 {
                                app_state.catalyst_scroll -= 1;
                            }
                        }
                        #[cfg(feature = "elektra")]
                        ViewMode::Elektra => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Terminal => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Attractor => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Virology => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        ViewMode::Evolution => {}
                        #[cfg(feature = "nova")]
                        ViewMode::BioMesh => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Reactor => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Biolum => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Ecology => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Right => match app_state.view_mode {
                        #[cfg(feature = "nova")]
                        ViewMode::Babel => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Strings => {}
                        ViewMode::Genome => {}
                        ViewMode::Grid => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Crispr => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Chronos => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Logos => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Pandemonium => {
                            app_state.pandemonium_cursor.0 += 1.0;
                        }
                        #[cfg(feature = "elektra")]
                        ViewMode::Elektra => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        ViewMode::Microscope => {}
                        #[cfg(feature = "resonance")]
                        ViewMode::Resonance => {}
                        #[cfg(feature = "biophysics")]
                        ViewMode::Cortex => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Grimoire => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Topology => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Graveyard => {}
                        #[cfg(feature = "nova")]
                        ViewMode::PianoRoll => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Retina => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quantum => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Schematic => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Foundry => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        ViewMode::BioticChaos => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Hyperspace => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Hologram => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        ViewMode::Catalyst => {}
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Weaver | ViewMode::Laboratory => {
                            if app_state.selected_strand < 2 {
                                app_state.selected_strand += 1;
                            } else {
                                app_state.selected_strand = 0;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Dream => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Phylogeny => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Alchemy => {
                            app_state.alchemy_selection = 1;
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Memetics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Egregore => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Bestiary => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Kaleidoscope => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Void => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Signals => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Sovereignty => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Spectrogram => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Market => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Ballistics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Scent => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Fishing => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Arena => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Garden => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Orca => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Quipu => {
                            if vm.quipu.active_cord + 1 < vm.quipu.cords.len() {
                                vm.quipu.active_cord += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Hydra => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Terminal => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Attractor => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Virology => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        ViewMode::Evolution => {}
                        #[cfg(feature = "nova")]
                        ViewMode::BioMesh => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Reactor => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Biolum => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Ecology => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Left => match app_state.view_mode {
                        #[cfg(feature = "nova")]
                        ViewMode::Babel => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Strings => {}
                        ViewMode::Genome => {}
                        ViewMode::Grid => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Crispr => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Chronos => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Logos => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Pandemonium => {
                            app_state.pandemonium_cursor.0 -= 1.0;
                        }
                        #[cfg(feature = "silicon")]
                        ViewMode::Foundry => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        ViewMode::BioticChaos => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Hyperspace => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Hologram => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        ViewMode::Catalyst => {}
                        #[cfg(feature = "elektra")]
                        ViewMode::Elektra => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        ViewMode::Microscope => {}
                        #[cfg(feature = "resonance")]
                        ViewMode::Resonance => {}
                        #[cfg(feature = "biophysics")]
                        ViewMode::Cortex => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Grimoire => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Topology => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Graveyard => {}
                        #[cfg(feature = "nova")]
                        ViewMode::PianoRoll => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Retina => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Schematic => {}
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Weaver | ViewMode::Laboratory => {
                            if app_state.selected_strand > 0 {
                                app_state.selected_strand -= 1;
                            } else {
                                app_state.selected_strand = 2;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Quantum => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Dream => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Phylogeny => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Alchemy => {
                            app_state.alchemy_selection = 0;
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Memetics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Egregore => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Bestiary => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Kaleidoscope => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Void => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Signals => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Sovereignty => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Spectrogram => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Market => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Ballistics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Scent => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Fishing => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Arena => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Garden => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Orca => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Quipu => {
                            if vm.quipu.active_cord > 0 {
                                vm.quipu.active_cord -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Hydra => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Terminal => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Attractor => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Virology => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        ViewMode::Evolution => {}
                        #[cfg(feature = "nova")]
                        ViewMode::BioMesh => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Reactor => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Biolum => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Ecology => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Enter => {
                        #[cfg(feature = "nova")]
                        if let ViewMode::Reactor = app_state.view_mode {
                            app_state.input_mode = InputMode::Editing;
                            app_state.input_buffer.clear();
                        }

                        if let ViewMode::Evolution = app_state.view_mode {
                            app_state.input_mode = InputMode::Editing;
                            app_state.input_buffer.clear();
                        }

                        #[cfg(feature = "silicon")]
                        if let ViewMode::Foundry = app_state.view_mode {
                            // Trace current circuit
                            let (x, y) = app_state.grid_cursor;
                            vm.stack.push(crate::vm::Value::Int(y as i64));
                            vm.stack.push(crate::vm::Value::Int(x as i64));
                            crate::vm::silicon::exec_silicon_op(
                                vm,
                                crate::opcode::OpCode::Trace,
                                &[],
                            );
                            if let Some(crate::vm::Value::Int(idx)) = vm.stack.last() {
                                app_state.status_msg = format!("Traced circuit to strand {}", idx);
                                app_state.selected_strand = *idx as usize;
                            }
                            return Ok(true);
                        }

                        app_state.input_mode = InputMode::Editing;
                        match app_state.view_mode {
                            #[cfg(feature = "nova")]
                            ViewMode::Phylogeny => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Alchemy => {
                                // Prevent entering edit mode for Alchemy (uses keys instead)
                                app_state.input_mode = InputMode::Normal;
                            }
                            ViewMode::Catalyst => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Biolum => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Reactor => {}
                            #[cfg(feature = "nova")]
                            ViewMode::BioMesh => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Crispr => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Hyperspace => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Hologram => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Weaver => {
                                // Allow editing mode for Pattern entry
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Terminal => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Attractor => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Virology => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            ViewMode::Evolution => {}
                            ViewMode::Genome => {
                                if app_state.selected_strand < vm.dna.helix.strands.len() {
                                    let g_len =
                                        vm.dna.helix.strands[app_state.selected_strand].genes.len();
                                    if app_state.selected_gene < g_len {
                                        let gene = &vm.dna.helix.strands[app_state.selected_strand]
                                            .genes[app_state.selected_gene];
                                        let mut s = format!("{}(", gene.op);
                                        for (i, arg) in gene.args.iter().enumerate() {
                                            if i > 0 {
                                                s.push(' ');
                                            }
                                            match arg {
                                                crate::ast::Nucleotide::Number(n) => {
                                                    s.push_str(&n.to_string())
                                                }
                                                crate::ast::Nucleotide::String(str_val) => {
                                                    s.push_str(&format!("\"{}\"", str_val))
                                                }
                                                crate::ast::Nucleotide::Identifier(id) => {
                                                    s.push_str(id)
                                                }
                                                crate::ast::Nucleotide::Junction(t, vals) => {
                                                    let t_str = match t {
                                                        crate::ast::JunctionType::Any => "any",
                                                        crate::ast::JunctionType::All => "all",
                                                        crate::ast::JunctionType::Dish => "dish",
                                                    };
                                                    s.push_str(t_str);
                                                    s.push('(');
                                                    for (k, v) in vals.iter().enumerate() {
                                                        if k > 0 {
                                                            s.push(' ');
                                                        }
                                                        match v {
                                                            crate::ast::Nucleotide::Number(n) => {
                                                                s.push_str(&n.to_string())
                                                            }
                                                            crate::ast::Nucleotide::String(
                                                                str_val,
                                                            ) => s.push_str(&format!(
                                                                "\"{}\"",
                                                                str_val
                                                            )),
                                                            crate::ast::Nucleotide::Identifier(
                                                                id,
                                                            ) => s.push_str(id),
                                                            crate::ast::Nucleotide::Junction(
                                                                _,
                                                                _,
                                                            ) => s.push_str("nested"),
                                                        }
                                                    }
                                                    s.push(')');
                                                }
                                            }
                                        }
                                        s.push(')');
                                        app_state.input_buffer = s;
                                    }
                                }
                            }
                            ViewMode::BioticChaos => {
                                let (x, y) = app_state.grid_cursor;
                                let val = vm.chaos_struct.grid[y][x];
                                app_state.input_buffer = format!("{:.4}", val);
                            }
                            ViewMode::Grid => {
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                match val {
                                    crate::vm::Value::Int(n) => {
                                        app_state.input_buffer = n.to_string()
                                    }
                                    crate::vm::Value::Str(s) => app_state.input_buffer = s.clone(),
                                    _ => app_state.input_buffer = String::new(),
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Chronos => {
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                match val {
                                    crate::vm::Value::Int(n) => {
                                        app_state.input_buffer = n.to_string()
                                    }
                                    crate::vm::Value::Str(s) => app_state.input_buffer = s.clone(),
                                    _ => app_state.input_buffer = String::new(),
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Logos => {
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                app_state.input_buffer = format!("{}", val);
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Pandemonium => {
                                app_state.input_mode = InputMode::Normal; // No editing mode for now
                            }
                            #[cfg(feature = "silicon")]
                            ViewMode::Foundry => {
                                // Prepare input buffer for editing
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                match val {
                                    crate::vm::Value::Int(n) => {
                                        app_state.input_buffer = n.to_string()
                                    }
                                    crate::vm::Value::Str(s) => app_state.input_buffer = s.clone(),
                                    _ => app_state.input_buffer = String::new(),
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Orca => {
                                // Enable editing grid from Orca view
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                match val {
                                    crate::vm::Value::Int(n) => {
                                        app_state.input_buffer = n.to_string()
                                    }
                                    crate::vm::Value::Str(s) => app_state.input_buffer = s.clone(),
                                    _ => app_state.input_buffer = String::new(),
                                }
                            }
                            ViewMode::Microscope => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "biophysics")]
                            ViewMode::Cortex => {
                                // Prevent entering edit mode for Cortex
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "resonance")]
                            ViewMode::Resonance => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Grimoire => {
                                app_state.input_mode = InputMode::Normal;
                                let mut registry: Vec<_> =
                                    vm.sigil_registry.keys().cloned().collect();
                                registry.sort();
                                if app_state.selected_sigil_index < registry.len() {
                                    let key = &registry[app_state.selected_sigil_index];
                                    if let Some(sigil) = vm.sigil_registry.get_mut(key) {
                                        sigil.auto_cast = !sigil.auto_cast;
                                        let status = if sigil.auto_cast {
                                            "ENABLED"
                                        } else {
                                            "DISABLED"
                                        };
                                        app_state.status_msg =
                                            format!("{} Auto-Cast: {}", key, status);
                                    }
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Laboratory => {
                                app_state.input_mode = InputMode::Normal;
                                let splice_op = crate::opcode::OpCode::Splice;
                                let args = vec![
                                    crate::ast::Nucleotide::Number(app_state.lab_parent_a as i64),
                                    crate::ast::Nucleotide::Number(app_state.lab_parent_b as i64),
                                    crate::ast::Nucleotide::Number(app_state.lab_method as i64),
                                ];
                                vm.execute_gene_inner(splice_op, &args);
                                app_state.status_msg = format!(
                                    "Spliced {} & {} (Method {})",
                                    app_state.lab_parent_a,
                                    app_state.lab_parent_b,
                                    app_state.lab_method
                                );
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Topology => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Graveyard => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::PianoRoll => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Retina => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Quantum => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Dream => {
                                app_state.input_mode = InputMode::Normal;
                                if app_state.selected_dream_trace < vm.dream_traces.len() {
                                    let (target_idx, mutated_strand) = {
                                        let trace =
                                            &vm.dream_traces[app_state.selected_dream_trace];
                                        (trace.strand_idx, trace.mutated_strand.clone())
                                    };

                                    if let Some(strand) = mutated_strand {
                                        // Lucid Dreaming: Inject the strand
                                        if target_idx < vm.dna.helix.strands.len() {
                                            vm.dna.helix.strands[target_idx] = strand;
                                            app_state.status_msg = format!(
                                                "LUCID DREAM: Realized mutations for strand {}",
                                                target_idx
                                            );
                                        }
                                    }
                                }
                            }
                            ViewMode::Heatmap => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "silicon")]
                            ViewMode::Schematic => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Memetics => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Egregore => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Bestiary => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Kaleidoscope => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Void => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Signals => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Sovereignty => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Spectrogram => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Market => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Ballistics => {
                                app_state.input_mode = InputMode::Normal;
    Ok(false)
}
