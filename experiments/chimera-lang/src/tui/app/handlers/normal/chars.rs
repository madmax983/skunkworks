use crate::tui::state::{AppState, InputMode, ViewMode};
use crate::vm::ChimeraVM;
use anyhow::Result;
use crossterm::event::KeyCode;

pub(crate) fn handle_char_input(
    c: char,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
    match KeyCode::Char(c) {
        KeyCode::Char('C') => {
            app_state.chaos_mode = !app_state.chaos_mode;
            app_state.status_msg = format!("Chaos Mode: {}", app_state.chaos_mode);
            Ok(false)
        }
        KeyCode::Char('h') => {
            app_state.view_mode = ViewMode::Heatmap;
            Ok(false)
        }
        KeyCode::Char('i') => {
            app_state.input_mode = InputMode::Injection;
            app_state.input_buffer.clear();
            Ok(false)
        }
        KeyCode::Char('?') => {
            app_state.show_view_selector = !app_state.show_view_selector;
            if app_state.show_view_selector {
                app_state.view_selector_state.borrow_mut().select(Some(0));
            }
            Ok(false)
        }
        KeyCode::Char('q') => Ok(true),
        KeyCode::Char(' ') => handle_char_space(vm, app_state),
        KeyCode::Char('s') => handle_char_s(vm, app_state),
        KeyCode::Char('f') => handle_char_f(vm, app_state),
        KeyCode::Char('m') => {
            vm.mutate();
            Ok(false)
        }
        KeyCode::Char('c') => {
            vm.chaos_mode = !vm.chaos_mode;
            Ok(false)
        }
        KeyCode::Char(c) => handle_mode_specific_chars(c, vm, app_state),
        _ => Ok(false),
    }
}

fn handle_mode_specific_chars(
    c: char,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
    match c {
        'K' => handle_char_k_upper(vm, app_state),
        'a' => handle_char_a(vm, app_state),
        'x' => handle_char_x(vm, app_state),
        't' => handle_char_t(vm, app_state),
        'e' => handle_char_e(vm, app_state),
        'G' => handle_char_g_upper(vm, app_state),
        'T' => handle_char_t_upper(vm, app_state),
        'I' => handle_char_i_upper(vm, app_state),
        'O' => handle_char_o_upper(vm, app_state),
        '+' => handle_char_plus(vm, app_state),
        '-' => handle_char_minus(vm, app_state),
        'r' => handle_char_r(vm, app_state),
        'p' => handle_char_p(vm, app_state),
        '/' => handle_char_slash(vm, app_state),
        '[' | ']' | '{' | '}' | '1' | '2' | '3' | '4' | '5' => {
            handle_char_brackets_numbers(c, vm, app_state)
        }
        #[cfg(feature = "nova")]
        'R' => handle_char_r_upper(vm, app_state),
        #[cfg(feature = "nova")]
        'M' => handle_char_m_upper(vm, app_state),
        #[cfg(feature = "nova")]
        'S' => handle_char_s_upper(vm, app_state),
        _ => handle_view_switch_chars(c, app_state),
    }
}

fn handle_view_switch_chars(c: char, app_state: &mut AppState) -> Result<bool> {
    match c {
        #[cfg(feature = "nova")]
        '^' => app_state.view_mode = ViewMode::Cambrian,
        #[cfg(feature = "nova")]
        '&' => app_state.view_mode = ViewMode::Semiotics,
        #[cfg(feature = "nova")]
        '*' => app_state.view_mode = ViewMode::Fractal,
        #[cfg(feature = "nova")]
        'y' => app_state.view_mode = ViewMode::LifeCycle,
        #[cfg(feature = "nova")]
        '|' => app_state.view_mode = ViewMode::Savant,
        #[cfg(feature = "nova")]
        '#' => app_state.view_mode = ViewMode::Akashic,
        #[cfg(feature = "nova")]
        '\\' => app_state.view_mode = ViewMode::Prologue,
        #[cfg(feature = "nova")]
        '6' => app_state.view_mode = ViewMode::Lexicon,
        #[cfg(feature = "nova")]
        'N' => app_state.view_mode = ViewMode::Narrative,
        #[cfg(feature = "silicon")]
        'F' => app_state.view_mode = ViewMode::Foundry,
        #[cfg(feature = "elektra")]
        'E' => app_state.view_mode = ViewMode::Elektra,
        #[cfg(feature = "nova")]
        'z' => app_state.view_mode = ViewMode::Bestiary,
        #[cfg(feature = "biophysics")]
        'b' => app_state.view_mode = ViewMode::Cortex,
        #[cfg(feature = "nova")]
        'k' => app_state.view_mode = ViewMode::Kaleidoscope,
        #[cfg(feature = "nova")]
        '$' => app_state.view_mode = ViewMode::Market,
        #[cfg(feature = "nova")]
        '!' => app_state.view_mode = ViewMode::Ballistics,
        #[cfg(feature = "nova")]
        '~' => app_state.view_mode = ViewMode::Scent,
        #[cfg(feature = "nova")]
        'V' => app_state.view_mode = ViewMode::Arena,
        #[cfg(feature = "nova")]
        'l' => app_state.view_mode = ViewMode::Biolum,
        #[cfg(feature = "nova")]
        'L' => app_state.view_mode = ViewMode::Babel,
        #[cfg(feature = "nova")]
        '=' => app_state.view_mode = ViewMode::Strings,
        #[cfg(feature = "nova")]
        'Y' => app_state.view_mode = ViewMode::Hydra,
        #[cfg(feature = "nova")]
        'U' => app_state.view_mode = ViewMode::Logos,
        #[cfg(feature = "nova")]
        'P' => app_state.view_mode = ViewMode::Pandemonium,
        #[cfg(feature = "nova")]
        'H' => app_state.view_mode = ViewMode::Hyperspace,
        #[cfg(feature = "nova")]
        'W' => app_state.view_mode = ViewMode::Weaver,
        #[cfg(feature = "nova")]
        '`' => app_state.view_mode = ViewMode::Terminal,
        #[cfg(feature = "nova")]
        'A' => app_state.view_mode = ViewMode::Attractor,
        #[cfg(feature = "nova")]
        'v' => app_state.view_mode = ViewMode::Virology,
        #[cfg(feature = "nova")]
        'B' => app_state.view_mode = ViewMode::BioMesh,
        #[cfg(feature = "nova")]
        'X' => app_state.view_mode = ViewMode::Reactor,
        _ => {}
    }
    Ok(false)
}

fn handle_char_k_upper(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
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
    #[cfg(not(feature = "nova"))]
    {
        let _ = vm;
        let _ = app_state;
    }
    Ok(false)
}

fn handle_char_a(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    #[cfg(feature = "nova")]
    match app_state.view_mode {
        ViewMode::Evolution => {
            app_state.evolution_state.auto_run = !app_state.evolution_state.auto_run;
        }
        ViewMode::Alchemy => match app_state.alchemy_selection {
            0 => {
                let elements = [
                    "Fire", "Water", "Earth", "Air", "Life", "Death", "Lead", "Energy",
                ];
                if app_state.alchemy_shelf_idx < elements.len() {
                    vm.crucible.add(crate::vm::Value::Str(
                        elements[app_state.alchemy_shelf_idx].to_string(),
                    ));
                }
            }
            1 => {
                if app_state.alchemy_strand_idx < vm.dna.helix.strands.len() {
                    vm.crucible
                        .add(crate::vm::Value::Int(app_state.alchemy_strand_idx as i64));
                }
            }
            _ => {}
        },
        _ => {}
    }
    #[cfg(not(feature = "nova"))]
    {
        let _ = vm;
        let _ = app_state;
    }
    Ok(false)
}

fn handle_char_x(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    #[cfg(feature = "nova")]
    {
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
    #[cfg(not(feature = "nova"))]
    {
        let _ = vm;
        let _ = app_state;
    }
    Ok(false)
}

fn handle_char_t(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    #[cfg(feature = "nova")]
    if let ViewMode::Alchemy = app_state.view_mode {
        crate::vm::alchemy::transmute_crucible(vm);
    }
    #[cfg(not(feature = "nova"))]
    {
        let _ = vm;
        let _ = app_state;
    }
    Ok(false)
}

fn handle_char_e(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    if let ViewMode::Genome = app_state.view_mode {
        if let Some(strand) = vm.dna.helix.strands.get(app_state.selected_strand) {
            let engine = crate::vm::evolution::EvolutionEngine::new(
                strand.clone(),
                20,
                app_state.evolution_state.challenge.clone(),
            );
            app_state.evolution_state.engine = Some(engine);
            app_state.view_mode = ViewMode::Evolution;
            app_state.status_msg = "Evolution Initialized".to_string();
        }
    }
    Ok(false)
}

fn handle_char_g_upper(_vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    #[cfg(feature = "nova")]
    {
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
    #[cfg(not(feature = "nova"))]
    {
        let _ = _vm;
        let _ = app_state;
    }
    Ok(false)
}

fn handle_char_t_upper(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    #[cfg(feature = "nova")]
    {
        if let ViewMode::Babel = app_state.view_mode {
            if let Some(ast) = &app_state.babel_ast {
                vm.stack.push(ast.clone());
                vm.stack
                    .push(crate::vm::Value::Str(app_state.babel_input.clone()));
                crate::vm::babel::exec_babel_op(vm, crate::opcode::OpCode::Tongue, &[]);
                if let Some(res) = vm.stack.pop() {
                    app_state.babel_result = format!("{}", res);
                }
            }
        } else {
            app_state.view_mode = ViewMode::Chronos;
        }
    }
    #[cfg(not(feature = "nova"))]
    {
        let _ = vm;
        let _ = app_state;
    }
    Ok(false)
}

fn handle_char_i_upper(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    #[cfg(feature = "nova")]
    {
        if let ViewMode::Hologram = app_state.view_mode {
            let idx = app_state.selected_strand;
            vm.stack.push(crate::vm::Value::Int(idx as i64));
            crate::vm::nova_hologram::exec_interfere(vm, crate::opcode::OpCode::Interfere, &[]);
            app_state.status_msg = format!("Interfered strand {}", idx);
        } else if let ViewMode::Ecology = app_state.view_mode {
            app_state.input_mode = InputMode::Editing;
            app_state.input_buffer.clear();
            app_state.status_msg = "Injecting Gene... (Type & Enter)".to_string();
        } else {
            app_state.view_mode = ViewMode::Hologram;
        }
    }
    #[cfg(not(feature = "nova"))]
    {
        let _ = vm;
        let _ = app_state;
    }
    Ok(false)
}

fn handle_char_o_upper(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    #[cfg(feature = "nova")]
    {
        if let ViewMode::Hologram = app_state.view_mode {
            crate::vm::nova_hologram::exec_refract(vm, crate::opcode::OpCode::Refract, &[]);
        } else {
            app_state.view_mode = ViewMode::Orca;
        }
    }
    #[cfg(not(feature = "nova"))]
    {
        let _ = vm;
        let _ = app_state;
    }
    Ok(false)
}

fn handle_char_plus(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    #[cfg(feature = "nova")]
    if let ViewMode::Hologram = app_state.view_mode {
        let (x, y) = app_state.grid_cursor;
        vm.hologram_grid[y][x].0 += 0.1;
        vm.hologram_grid[y][x].1 += 0.1;
    }
    #[cfg(not(feature = "nova"))]
    {
        let _ = vm;
        let _ = app_state;
    }
    Ok(false)
}

fn handle_char_minus(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    #[cfg(feature = "nova")]
    if let ViewMode::Hologram = app_state.view_mode {
        let (x, y) = app_state.grid_cursor;
        vm.hologram_grid[y][x].0 -= 0.1;
        vm.hologram_grid[y][x].1 -= 0.1;
    }
    #[cfg(not(feature = "nova"))]
    {
        let _ = vm;
        let _ = app_state;
    }
    Ok(false)
}

fn handle_char_r(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    #[cfg(feature = "nova")]
    {
        if let ViewMode::Graveyard = app_state.view_mode {
            match vm.resurrect_from_graveyard(app_state.selected_graveyard_strand) {
                Ok(idx) => {
                    app_state.status_msg = format!("Resurrected strand {}!", idx);
                    if app_state.selected_graveyard_strand >= vm.graveyard.len()
                        && !vm.graveyard.is_empty()
                    {
                        app_state.selected_graveyard_strand = vm.graveyard.len() - 1;
                    }
                }
                Err(e) => app_state.status_msg = format!("Error: {}", e),
            }
        }
    }
    #[cfg(not(feature = "nova"))]
    {
        let _ = vm;
        let _ = app_state;
    }
    Ok(false)
}

fn handle_char_p(_vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    if let ViewMode::Grid = app_state.view_mode {
        app_state.palette_open = !app_state.palette_open;
    } else {
        #[cfg(feature = "nova")]
        {
            app_state.view_mode = ViewMode::PianoRoll;
        }
    }
    Ok(false)
}

fn handle_char_slash(_vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    #[cfg(all(feature = "oracle", feature = "nova"))]
    {
        if let ViewMode::Grimoire = app_state.view_mode {
            app_state.query_mode = true;
            app_state.query_input.clear();
            app_state.query_results.clear();
        }
    }
    #[cfg(not(all(feature = "oracle", feature = "nova")))]
    {
        let _ = _vm;
        let _ = app_state;
    }
    Ok(false)
}

fn handle_char_brackets_numbers(
    c: char,
    _vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
    #[cfg(feature = "nova")]
    {
        match c {
            '1' => {
                if let ViewMode::Pandemonium = app_state.view_mode {
                    app_state.pandemonium_selected_tool = 0;
                }
            }
            '2' => {
                if let ViewMode::Pandemonium = app_state.view_mode {
                    app_state.pandemonium_selected_tool = 1;
                }
            }
            '3' => {
                if let ViewMode::Pandemonium = app_state.view_mode {
                    app_state.pandemonium_selected_tool = 2;
                }
            }
            '4' => {
                if let ViewMode::Pandemonium = app_state.view_mode {
                    app_state.pandemonium_selected_tool = 3;
                }
            }
            '5' => {
                if let ViewMode::Pandemonium = app_state.view_mode {
                    app_state.pandemonium_selected_tool = 4;
                }
            }
            '[' => {
                if let ViewMode::Kaleidoscope = app_state.view_mode {
                    if app_state.kaleidoscope_hue_idx > 0 {
                        app_state.kaleidoscope_hue_idx -= 1;
                    } else {
                        app_state.kaleidoscope_hue_idx = 5;
                    }
                } else if let ViewMode::Pandemonium = app_state.view_mode {
                    app_state.pandemonium_radius = (app_state.pandemonium_radius - 1.0).max(1.0);
                }
            }
            ']' => {
                if let ViewMode::Kaleidoscope = app_state.view_mode {
                    app_state.kaleidoscope_hue_idx = (app_state.kaleidoscope_hue_idx + 1) % 6;
                } else if let ViewMode::Pandemonium = app_state.view_mode {
                    app_state.pandemonium_radius += 1.0;
                }
            }
            '{' => {
                if let ViewMode::Kaleidoscope = app_state.view_mode {
                    if app_state.kaleidoscope_light_idx > 0 {
                        app_state.kaleidoscope_light_idx -= 1;
                    } else {
                        app_state.kaleidoscope_light_idx = 2;
                    }
                }
            }
            '}' => {
                if let ViewMode::Kaleidoscope = app_state.view_mode {
                    app_state.kaleidoscope_light_idx = (app_state.kaleidoscope_light_idx + 1) % 3;
                }
            }
            _ => {}
        }
    }
    #[cfg(not(feature = "nova"))]
    {
        let _ = _vm;
        let _ = app_state;
        let _ = c;
    }
    Ok(false)
}

fn handle_char_s_upper(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    #[cfg(feature = "nova")]
    {
        let ViewMode::Arena = app_state.view_mode else {
            return Ok(false);
        };

        let Some(arena) = &mut vm.arena else {
            return Ok(false);
        };

        // Add random gladiators if empty
        if arena.combatants.is_empty() {
            // Use some existing strands or random
            let mut rng = rand::thread_rng();
            use rand::Rng;
            if !vm.dna.helix.strands.is_empty() {
                let s1 = vm.dna.helix.strands[rng.gen_range(0..vm.dna.helix.strands.len())].clone();
                let s2 = vm.dna.helix.strands[rng.gen_range(0..vm.dna.helix.strands.len())].clone();
                arena.add_gladiator(s1, rng.gen());
                arena.add_gladiator(s2, rng.gen());
            }
        }
        arena.start();
        app_state.status_msg = "Arena Started!".to_string();
        Ok(false)
    }
    #[cfg(not(feature = "nova"))]
    {
        let _ = vm;
        let _ = app_state;
        Ok(false)
    }
}

#[cfg(feature = "nova")]
fn handle_char_m_upper(_vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
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
    Ok(false)
}

#[cfg(feature = "nova")]
fn handle_char_r_upper(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    match app_state.view_mode {
        ViewMode::Kaleidoscope => {
            vm.piet_state = None;
            app_state.status_msg = "Piet State Reset".to_string();
        }
        ViewMode::Arena => {
            if let Some(arena) = &mut vm.arena {
                arena.reset();
                app_state.status_msg = "Arena Reset".to_string();
            }
        }
        ViewMode::Babel => {
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
        _ => {}
    }
    Ok(false)
}

fn handle_char_s(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    #[cfg(feature = "nova")]
    match app_state.view_mode {
        ViewMode::Ecology => {
            crate::vm::nova_ecology::spawn_random_ecology(vm, 10);
            app_state.status_msg = "Spawned 10 organisms.".to_string();
            return Ok(true);
        }
        ViewMode::Kaleidoscope => {
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
        _ => {}
    }

    #[cfg(feature = "silicon")]
    {
        app_state.view_mode = ViewMode::Schematic;
    }
    Ok(false)
}

fn handle_space_evolution(vm: &mut ChimeraVM, app_state: &mut AppState) {
    if let Some(engine) = &mut app_state.evolution_state.engine {
        engine.step(vm);
    }
}

#[cfg(feature = "nova")]
fn handle_space_babel(vm: &mut ChimeraVM, app_state: &mut AppState) {
    // Run Parse
    vm.stack
        .push(crate::vm::Value::Str(app_state.babel_pattern.clone()));
    let _ = crate::vm::babel::exec_babel_op(vm, crate::opcode::OpCode::ParserRegex, &[]);
    vm.stack
        .push(crate::vm::Value::Str(app_state.babel_input.clone()));
    let _ = crate::vm::babel::exec_babel_op(vm, crate::opcode::OpCode::Parse, &[]);

    if let Some(res) = vm.stack.pop() {
        app_state.babel_result = format!("{}", res);
    } else {
        app_state.babel_result = "Stack Empty/Error".to_string();
    }
}

fn handle_space_grid(vm: &mut ChimeraVM, app_state: &mut AppState) {
    if let Some(c) = app_state.palette_char {
        let (x, y) = app_state.grid_cursor;
        vm.grid[y][x] = crate::vm::Value::Str(c.to_string());
    } else {
        vm.step();
    }
}

#[cfg(feature = "nova")]
fn handle_space_arena(vm: &mut ChimeraVM, _app_state: &mut AppState) {
    if let Some(arena) = &mut vm.arena {
        arena.tick();
    }
}

#[cfg(feature = "nova")]
fn handle_space_pandemonium(vm: &mut ChimeraVM, app_state: &mut AppState) {
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
            1 => crate::vm::pandemonium::apply_scramble(vm, s, g, app_state.pandemonium_radius),
            2 => crate::vm::pandemonium::apply_purge(vm, s, g, app_state.pandemonium_radius),
            3 => crate::vm::pandemonium::apply_duplicate(vm, s, g),
            4 => crate::vm::pandemonium::apply_storm(vm, s, g, app_state.pandemonium_radius),
            _ => {}
        }
        app_state.status_msg = format!("Pandemonium applied at {},{}", s, g);
    }
}

#[cfg(feature = "nova")]
fn handle_space_fishing(vm: &mut ChimeraVM, app_state: &mut AppState) {
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
}

#[cfg(feature = "nova")]
fn handle_space_kaleidoscope(vm: &mut ChimeraVM, app_state: &mut AppState) {
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
        2 => (r / 2, g / 2, b / 2),                                     // Dark
        _ => (r, g, b),                                                 // Normal
    };

    vm.chroma_grid[y][x].fg = Some((r as u8, g as u8, b as u8));
}

fn handle_char_space(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    match app_state.view_mode {
        ViewMode::Evolution => {
            handle_space_evolution(vm, app_state);
            Ok(true)
        }
        #[cfg(feature = "nova")]
        ViewMode::Babel => {
            handle_space_babel(vm, app_state);
            Ok(true)
        }
        ViewMode::Grid => {
            handle_space_grid(vm, app_state);
            Ok(false)
        }
        #[cfg(feature = "nova")]
        ViewMode::Arena => {
            handle_space_arena(vm, app_state);
            Ok(false)
        }
        #[cfg(feature = "nova")]
        ViewMode::Pandemonium => {
            handle_space_pandemonium(vm, app_state);
            Ok(false)
        }
        #[cfg(feature = "nova")]
        ViewMode::Fishing => {
            handle_space_fishing(vm, app_state);
            Ok(false)
        }
        #[cfg(feature = "nova")]
        ViewMode::Kaleidoscope => {
            handle_space_kaleidoscope(vm, app_state);
            Ok(false)
        }
        _ => {
            vm.step();
            Ok(false)
        }
    }
}
fn handle_char_f(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
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
        crate::vm::silicon::exec_silicon_op(vm, crate::opcode::OpCode::Fabricate, &[]);
        app_state.status_msg = format!("Fabricated strand {} at {},{}", s_idx, x, y);
        return Ok(true);
    }

    #[cfg(feature = "nova")]
    {
        app_state.view_mode = ViewMode::Fishing;
    }
    Ok(false)
}
