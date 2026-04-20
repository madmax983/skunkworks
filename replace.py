import re

content = open("experiments/chimera-lang/src/tui/app/handlers/normal/chars.rs").read()

start_idx = content.find("pub(crate) fn handle_char_input(")
end_idx = content.find("fn handle_char_s_upper(", start_idx)

new_handle_char_input = """pub(crate) fn handle_char_input(
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
        KeyCode::Char('h') => { app_state.view_mode = ViewMode::Heatmap; Ok(false) },
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
        KeyCode::Char('m') => { vm.mutate(); Ok(false) },
        KeyCode::Char('c') => { vm.chaos_mode = !vm.chaos_mode; Ok(false) },
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
        '\\'=> app_state.view_mode = ViewMode::Prologue,
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
    {
        if let ViewMode::Evolution = app_state.view_mode {
            app_state.evolution_state.auto_run = !app_state.evolution_state.auto_run;
        } else if let ViewMode::Alchemy = app_state.view_mode {
            match app_state.alchemy_selection {
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

"""

new_file_content = content[:start_idx] + new_handle_char_input + content[end_idx:]

with open("experiments/chimera-lang/src/tui/app/handlers/normal/chars.rs", "w") as f:
    f.write(new_file_content)
