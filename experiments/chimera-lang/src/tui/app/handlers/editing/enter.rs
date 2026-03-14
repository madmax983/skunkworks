use crate::ast::Gene;
use crate::vm::ChimeraVM;
use crate::{ChimeraParser, Rule};
use super::super::super::state::{AppState, InputMode, ViewMode};
use anyhow::Result;
use pest::Parser;

pub(crate) fn handle_enter(
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<bool> {
    let _ =  {
                            match app_state.view_mode {
                                #[cfg(feature = "nova")]
                                ViewMode::Paradox => {
                                    if !app_state.paradox_editor_buffer.is_empty() {
                                        match vm
                                            .paradox
                                            .parse_rule(&app_state.paradox_editor_buffer)
                                        {
                                            Ok(_) => {
                                                app_state.status_msg =
                                                    "Paradox Rule Compiled.".to_string();
                                                app_state.paradox_editor_buffer.clear();
                                            }
                                            Err(e) => {
                                                app_state.status_msg = format!("Error: {}", e);
                                            }
                                        }
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                }
                                ViewMode::Genome => {
                                    // Genome Editing Logic
                                    match ChimeraParser::parse(Rule::gene, &app_state.input_buffer)
                                    {
                                        Ok(mut pairs) => {
                                            let pair = pairs.next().unwrap();
                                            match Gene::try_from_pair(pair) {
                                                Ok(gene) => {
                                                    if app_state.selected_strand
                                                        < vm.dna.helix.strands.len()
                                                        && app_state.selected_gene
                                                            < vm.dna.helix.strands
                                                                [app_state.selected_strand]
                                                                .genes
                                                                .len()
                                                    {
                                                        vm.dna.helix.strands
                                                            [app_state.selected_strand]
                                                            .genes[app_state.selected_gene] = gene;
                                                        app_state.status_msg =
                                                            "Gene updated successfully".to_string();
                                                    }
                                                    app_state.input_mode = InputMode::Normal;
                                                    app_state.input_buffer.clear();
                                                }
                                                Err(e) => {
                                                    app_state.status_msg =
                                                        format!("Parse Error: {}", e);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            app_state.status_msg = format!("Parse Error: {}", e);
                                        }
                                    }
                                }
                                ViewMode::Grid => {
                                    // Grid Editing Logic
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    let (x, y) = app_state.grid_cursor;
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Crispr => {
                                    // Execute CRISPR Logic
                                    let guide_tokens: Vec<&str> =
                                        app_state.crispr_guide.split_whitespace().collect();
                                    let replace_tokens: Vec<&str> =
                                        app_state.crispr_replace.split_whitespace().collect();
                                    use std::str::FromStr;

                                    let mut guide_ops = Vec::new();
                                    for t in &guide_tokens {
                                        if let Ok(op) = crate::opcode::OpCode::from_str(t) {
                                            guide_ops.push(op);
                                        }
                                    }
                                    let mut replace_genes = Vec::new();
                                    for t in &replace_tokens {
                                        if let Ok(op) = crate::opcode::OpCode::from_str(t) {
                                            replace_genes
                                                .push(crate::ast::Gene { op, args: vec![] });
                                        }
                                    }

                                    if guide_ops.is_empty() {
                                        app_state.crispr_result =
                                            "Error: Empty Guide Pattern".to_string();
                                    } else {
                                        let s_idx = app_state.crispr_target_strand;
                                        if s_idx < vm.dna.helix.strands.len() {
                                            let strand = &mut vm.dna.helix.strands[s_idx];
                                            let mut new_genes = Vec::new();
                                            let mut i = 0;
                                            let mut matches = 0;
                                            while i < strand.genes.len() {
                                                let mut matched = true;
                                                for (j, op) in guide_ops.iter().enumerate() {
                                                    if i + j >= strand.genes.len()
                                                        || strand.genes[i + j].op != *op
                                                    {
                                                        matched = false;
                                                        break;
                                                    }
                                                }
                                                if matched {
                                                    new_genes.extend(replace_genes.clone());
                                                    i += guide_ops.len();
                                                    matches += 1;
                                                } else {
                                                    new_genes.push(strand.genes[i].clone());
                                                    i += 1;
                                                }
                                            }
                                            strand.genes = new_genes;
                                            app_state.crispr_result = format!(
                                                "CRISPR: Replaced {} occurrences.",
                                                matches
                                            );
                                        } else {
                                            app_state.crispr_result =
                                                "Error: Invalid Strand".to_string();
                                        }
                                    }
                                    // Stay in Editing mode
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Chronos => {
                                    // Enable editing grid from Chronos view
                                    let (x, y) = app_state.grid_cursor;
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Logos => {
                                    // Enable editing grid from Logos view
                                    let (x, y) = app_state.grid_cursor;
                                    // Should parse as String usually for Atoms
                                    // parse_grid_value handles numbers.
                                    let val = if app_state.input_buffer.starts_with('?') {
                                        // Variable
                                        crate::vm::Value::Str(app_state.input_buffer.clone())
                                    } else {
                                        parse_grid_value(&app_state.input_buffer)
                                    };
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Pandemonium => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Orca => {
                                    // Grid Editing Logic
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    let (x, y) = app_state.grid_cursor;
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                ViewMode::Microscope => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "biophysics")]
                                ViewMode::Cortex => {
                                    // No editing for Cortex view yet
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "resonance")]
                                ViewMode::Resonance => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Grimoire => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Laboratory => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Topology => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Graveyard => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::PianoRoll => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Retina => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Quantum => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                ViewMode::Heatmap => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "silicon")]
                                ViewMode::Schematic => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Dream => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Phylogeny => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Alchemy => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Memetics => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Egregore => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Bestiary => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Kaleidoscope => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Void => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Signals => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Sovereignty => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Spectrogram => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Market => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Ballistics => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Scent => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Fishing => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Garden => {
                                    // Enable editing for Garden (Sowing rules)
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "elektra")]
                                ViewMode::Elektra => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Arena => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Babel => {
                                    // In Babel, Enter in Normal mode enters Editing mode.
                                    // Editing happens directly on the strings, no buffer commit needed here.
                                    // But we use input_buffer as scratchpad in other modes.
                                    // Here we edit in place.
                                    // So we just clear buffer and exit?
                                    // Wait, if we are in Editing mode, keys append to buffer.
                                    // We need to implement custom handling for Babel in Editing mode loop.
                                    // See below.
                                    app_state.input_mode = InputMode::Normal;
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Strings => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Quipu => {
                                    // Edit cord value?
                                    // Let's allow setting value of active cord
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    if let crate::vm::Value::Int(n) = val {
                                        if let Some(cord) =
                                            vm.quipu.cords.get_mut(vm.quipu.active_cord)
                                        {
                                            *cord = n;
                                            app_state.status_msg = format!(
                                                "Cord {} set to {}",
                                                vm.quipu.active_cord, n
                                            );
                                        }
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Hydra => {
                                    // Enable editing grid from Hydra view
                                    let (x, y) = app_state.grid_cursor;
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "silicon")]
                                ViewMode::Foundry => {
                                    // Same as Schematic/Grid?
                                    // Allow editing grid in Foundry
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    let (x, y) = app_state.grid_cursor;
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                ViewMode::BioticChaos => {
                                    // Allow editing Chaos Grid?
                                    // Parse buffer as float
                                    if let Ok(v) = app_state.input_buffer.parse::<f64>() {
                                        let (x, y) = app_state.grid_cursor;
                                        vm.chaos_struct.grid[y][x] = v.clamp(0.0, 1.0);
                                        app_state.status_msg =
                                            format!("Chaos Grid updated at {},{}", x, y);
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                ViewMode::Catalyst => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Hyperspace => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Hologram => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Weaver => {
                                    // Use input buffer as pattern
                                    app_state.input_mode = InputMode::Normal;
                                    // Don't clear buffer, keep it for preview
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Terminal => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Attractor => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Virology => {
                                    match app_state.virus_design_focus {
                                        0 => {
                                            app_state.virus_design_name =
                                                app_state.input_buffer.clone()
                                        }
                                        1 => {
                                            app_state.virus_design_pattern =
                                                app_state.input_buffer.clone()
                                        }
                                        2 => {
                                            if let Ok(n) = app_state.input_buffer.parse::<u8>() {
                                                app_state.virus_design_rate = n.clamp(0, 100);
                                            }
                                        }
                                        3 => {
                                            if let Ok(n) = app_state.input_buffer.parse::<i64>() {
                                                app_state.virus_design_payload = n;
                                            }
                                        }
                                        _ => {}
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Prologue => {
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    let (x, y) = app_state.grid_cursor;
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Lexicon => {
                                    let val = if app_state.input_buffer.len() == 1 {
                                        crate::vm::Value::Str(app_state.input_buffer.clone())
                                    } else {
                                        parse_grid_value(&app_state.input_buffer)
                                    };
                                    let (x, y) = app_state.grid_cursor;
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                ViewMode::Evolution => {
                                    if let Ok(val) = app_state.input_buffer.parse::<i64>() {
                                        app_state.evolution_state.challenge =
                                            crate::vm::evolution::Challenge::Target(val);
                                        if let Some(engine) = &mut app_state.evolution_state.engine
                                        {
                                            engine.challenge =
                                                crate::vm::evolution::Challenge::Target(val);
                                        }
                                        app_state.status_msg = format!("Target set to {}", val);
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Ecology => {
                                    // Inject Gene into Selected Organelle
                                    let gene_src = app_state.input_buffer.clone();
                                    if !gene_src.is_empty() {
                                        // 1. Compile gene
                                        // We use a hack: wrap in strand to compile, then extract gene
                                        let src = format!("strand injection {{ {} }}", gene_src);
                                        match crate::compiler::compile(&src, None) {
                                            Ok(dna) => {
                                                if let Some(strand) = dna.helix.strands.first() {
                                                    // 2. Inject into selected organelle
                                                    let mut found = false;
                                                    let (cx, cy) = app_state.grid_cursor;
                                                    for org in vm.organelles.iter_mut() {
                                                        if org.context_loc == (cy, cx) {
                                                            // Push to stack or execute immediately?
                                                            // Let's append to their current strand? No, shared DNA.
                                                            // Let's force execute immediately (Interrupt)
                                                            // Or push to their stack?

                                                            // "Mad Science" Injection: Modify the Organelle's IP to a new ephemeral strand?
                                                            // Complicated.
                                                            // Let's just try to execute the genes on the organelle's stack context?
                                                            // VM doesn't support executing genes on organelle directly easily without setting IP.

                                                            // Simplest: Add genes to the end of the Helix, and Jump the organelle there.
                                                            vm.dna
                                                                .helix
                                                                .strands
                                                                .push(strand.clone());
                                                            let new_idx =
                                                                vm.dna.helix.strands.len() - 1;

                                                            // Save current IP to call stack
                                                            org.call_stack.push(org.ip);
                                                            org.ip = (new_idx, 0);

                                                            found = true;
                                                            app_state.status_msg = format!(
                                                                "Injected code into {}",
                                                                org.name
                                                            );
                                                            break;
                                                        }
                                                    }
                                                    if !found {
                                                        app_state.status_msg =
                                                            "No organelle at cursor.".to_string();
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                app_state.status_msg =
                                                    format!("Compilation Error: {}", e);
                                            }
                                        }
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::BioMesh => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Reactor => {
                                    let (x, y) = app_state.grid_cursor;
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Biolum => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Fractal => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Metazoa => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Genesis => {
                                    // Commit change based on focus
                                    if app_state.genesis_focus == 0 {
                                        // Compile Editor Code
                                        let src = format!(
                                            "strand genesis {{ {} }}",
                                            app_state.genesis_editor_buffer
                                        );
                                        match crate::compiler::compile(&src, None) {
                                            Ok(dna) => {
                                                if let Some(strand) = dna.helix.strands.first() {
                                                    // Execute immediately
                                                    for gene in &strand.genes {
                                                        vm.execute_gene_inner(
                                                            gene.op.clone(),
                                                            &gene.args,
                                                        );
                                                    }
                                                    app_state.status_msg =
                                                        "Genesis: Executed.".to_string();
                                                }
                                            }
                                            Err(e) => {
                                                app_state.status_msg =
                                                    format!("Compile Error: {}", e)
                                            }
                                        }
                                        // Clear buffer? Maybe keep it for repeated editing.
                                        app_state.input_mode = InputMode::Normal;
                                    } else if app_state.genesis_focus == 1 {
                                        // Update Grammar
                                        match crate::lisp::parse(&app_state.genesis_grammar_buffer)
                                        {
                                            Ok(exprs) => {
                                                // Take the first expression as the grammar
                                                if let Some(expr) = exprs.first() {
                                                    match crate::lisp::sexpr_to_value(expr) {
                                                        Ok(grammar) => {
                                                            vm.active_grammar = grammar;
                                                            app_state.status_msg =
                                                                "Genesis: Grammar Updated."
                                                                    .to_string();
                                                        }
                                                        Err(e) => {
                                                            app_state.status_msg = format!(
                                                                "Value Conversion Error: {}",
                                                                e
                                                            )
                                                        }
                                                    }
                                                } else {
                                                    app_state.status_msg =
                                                        "Error: Empty Grammar".to_string();
                                                }
                                                app_state.input_mode = InputMode::Normal;
                                            }
                                            Err(e) => {
                                                app_state.status_msg = format!("Lisp Error: {}", e)
                                            }
                                        }
                                    } else {
                                        // Grid
                                        let val = parse_grid_value(&app_state.input_buffer);
                                        let (x, y) = app_state.grid_cursor;
                                        vm.grid[y][x] = val;
                                        app_state.input_mode = InputMode::Normal;
                                        app_state.input_buffer.clear();
                                    }
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Forge => {
                                    if app_state.forge_focus == 1 {
                                        // Define Rule
                                        if !app_state.forge_selected_rule.is_empty() {
                                            vm.prologue_state.logos_engine.define_rule(
                                                &app_state.forge_selected_rule,
                                                &app_state.forge_editor_buffer,
                                            );
                                            app_state.status_msg = format!(
                                                "Forge: Rule '{}' updated.",
                                                app_state.forge_selected_rule
                                            );
                                        }
                                    } else if app_state.forge_focus == 2 {
                                        // Test Rule
                                        if !app_state.forge_selected_rule.is_empty() {
                                            match vm.prologue_state.logos_engine.parse_input(
                                                &app_state.forge_selected_rule,
                                                &app_state.forge_test_input,
                                            ) {
                                                Ok(val) => {
                                                    app_state.forge_test_output =
                                                        format!("Success: {}", val);
                                                }
                                                Err(e) => {
                                                    app_state.forge_test_output =
                                                        format!("Error: {}", e);
                                                }
                                            }
                                        }
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                }
                                _ => {}
                            }
                        };
    Ok(true)
}
