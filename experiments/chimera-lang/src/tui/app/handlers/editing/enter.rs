use crate::ast::Gene;
use crate::tui::parse_grid_value;
use crate::tui::state::{AppState, InputMode, ViewMode};
use crate::vm::ChimeraVM;
use crate::{ChimeraParser, Rule};
use anyhow::Result;
use pest::Parser;

fn apply_grid_edit(vm: &mut ChimeraVM, app_state: &mut AppState) -> Option<bool> {
    let val = parse_grid_value(&app_state.input_buffer);
    let (x, y) = app_state.grid_cursor;
    vm.grid[y][x] = val;
    app_state.status_msg = format!("Grid updated at {},{}", x, y);
    Some(true)
}

#[cfg(feature = "nova")]
fn handle_crispr_enter(vm: &mut ChimeraVM, app_state: &mut AppState) -> Option<bool> {
    use std::str::FromStr;

    // ⚡ Bolt: Removed intermediate `.collect::<Vec<_>>()` allocations for tokens.
    // Iterating directly over `split_whitespace()` avoids O(N) heap allocations for temporary string slices.
    let mut guide_ops = Vec::new();
    for t in app_state.crispr_guide.split_whitespace() {
        if let Ok(op) = crate::opcode::OpCode::from_str(t) {
            guide_ops.push(op);
        }
    }
    let mut replace_genes = Vec::new();
    for t in app_state.crispr_replace.split_whitespace() {
        if let Ok(op) = crate::opcode::OpCode::from_str(t) {
            replace_genes.push(crate::ast::Gene { op, args: vec![] });
        }
    }

    if guide_ops.is_empty() {
        app_state.crispr_result = "Error: Empty Guide Pattern".to_string();
    } else {
        let s_idx = app_state.crispr_target_strand;
        if s_idx < vm.dna.helix.strands.len() {
            let strand = &mut vm.dna.helix.strands[s_idx];
            // ⚡ Bolt: Pre-allocate capacity for new_genes based on original strand length
            // to avoid immediate reallocation as items are pushed or extended.
            let mut new_genes = Vec::with_capacity(strand.genes.len());
            let mut i = 0;
            let mut matches = 0;
            while i < strand.genes.len() {
                let mut matched = true;
                for (j, op) in guide_ops.iter().enumerate() {
                    if i + j >= strand.genes.len() || strand.genes[i + j].op != *op {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    // ⚡ Bolt: Use .iter().cloned() instead of .clone() on the Vec to avoid
                    // an intermediate heap allocation during the `extend` operation.
                    new_genes.extend(replace_genes.iter().cloned());
                    i += guide_ops.len();
                    matches += 1;
                } else {
                    new_genes.push(strand.genes[i].clone());
                    i += 1;
                }
            }
            strand.genes = new_genes;
            app_state.crispr_result = format!("CRISPR: Replaced {} occurrences.", matches);
        } else {
            app_state.crispr_result = "Error: Invalid Strand".to_string();
        }
    }
    None
}

#[cfg(feature = "nova")]
fn handle_paradox_enter(vm: &mut ChimeraVM, app_state: &mut AppState) -> Option<bool> {
    if !app_state.paradox_editor_buffer.is_empty() {
        match vm.paradox.parse_rule(&app_state.paradox_editor_buffer) {
            Ok(_) => {
                app_state.status_msg = "Paradox Rule Compiled.".to_string();
                app_state.paradox_editor_buffer.clear();
            }
            Err(e) => {
                app_state.status_msg = format!("Error: {}", e);
            }
        }
    }
    Some(false)
}

fn handle_genome_enter(vm: &mut ChimeraVM, app_state: &mut AppState) -> Option<bool> {
    // Genome Editing Logic
    let mut pairs = match ChimeraParser::parse(Rule::gene, &app_state.input_buffer) {
        Ok(p) => p,
        Err(e) => {
            app_state.status_msg = format!("Parse Error: {}", e);
            return None;
        }
    };

    let Some(pair) = pairs.next() else {
        app_state.status_msg = "Parse Error: Empty input".to_string();
        return None;
    };

    let gene = match Gene::try_from_pair(pair) {
        Ok(g) => g,
        Err(e) => {
            app_state.status_msg = format!("Parse Error: {}", e);
            return None;
        }
    };

    if app_state.selected_strand < vm.dna.helix.strands.len()
        && app_state.selected_gene < vm.dna.helix.strands[app_state.selected_strand].genes.len()
    {
        vm.dna.helix.strands[app_state.selected_strand].genes[app_state.selected_gene] = gene;
        app_state.status_msg = "Gene updated successfully".to_string();
    }

    Some(true)
}

#[cfg(feature = "nova")]
fn handle_logos_enter(vm: &mut ChimeraVM, app_state: &mut AppState) -> Option<bool> {
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
    Some(true)
}

#[cfg(feature = "nova")]
fn handle_quipu_enter(vm: &mut ChimeraVM, app_state: &mut AppState) -> Option<bool> {
    // Edit cord value?
    // Let's allow setting value of active cord
    let val = parse_grid_value(&app_state.input_buffer);
    if let crate::vm::Value::Int(n) = val {
        if let Some(cord) = vm.quipu.cords.get_mut(vm.quipu.active_cord) {
            *cord = n;
            app_state.status_msg = format!("Cord {} set to {}", vm.quipu.active_cord, n);
        }
    }
    Some(true)
}

fn handle_biotic_chaos_enter(vm: &mut ChimeraVM, app_state: &mut AppState) -> Option<bool> {
    // Allow editing Chaos Grid?
    // Parse buffer as float
    if let Ok(v) = app_state.input_buffer.parse::<f64>() {
        let (x, y) = app_state.grid_cursor;
        vm.chaos_struct.grid[y][x] = v.clamp(0.0, 1.0);
        app_state.status_msg = format!("Chaos Grid updated at {},{}", x, y);
    }
    Some(true)
}

#[cfg(feature = "nova")]
fn handle_virology_enter(_vm: &mut ChimeraVM, app_state: &mut AppState) -> Option<bool> {
    match app_state.virus_design_focus {
        0 => app_state.virus_design_name = app_state.input_buffer.clone(),
        1 => app_state.virus_design_pattern = app_state.input_buffer.clone(),
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
    Some(true)
}

#[cfg(feature = "nova")]
fn handle_lexicon_enter(vm: &mut ChimeraVM, app_state: &mut AppState) -> Option<bool> {
    let val = if app_state.input_buffer.len() == 1 {
        crate::vm::Value::Str(app_state.input_buffer.clone())
    } else {
        parse_grid_value(&app_state.input_buffer)
    };
    let (x, y) = app_state.grid_cursor;
    vm.grid[y][x] = val;
    app_state.status_msg = format!("Grid updated at {},{}", x, y);
    Some(true)
}

fn handle_evolution_enter(_vm: &mut ChimeraVM, app_state: &mut AppState) -> Option<bool> {
    if let Ok(val) = app_state.input_buffer.parse::<i64>() {
        app_state.evolution_state.challenge = crate::vm::evolution::Challenge::Target(val);
        if let Some(engine) = &mut app_state.evolution_state.engine {
            engine.challenge = crate::vm::evolution::Challenge::Target(val);
        }
        app_state.status_msg = format!("Target set to {}", val);
    }
    Some(true)
}

#[cfg(feature = "nova")]
fn handle_ecology_enter(vm: &mut ChimeraVM, app_state: &mut AppState) -> Option<bool> {
    // Inject Gene into Selected Organelle
    let gene_src = app_state.input_buffer.clone();
    if gene_src.is_empty() {
        return Some(true);
    }

    // 1. Compile gene
    // We use a hack: wrap in strand to compile, then extract gene
    let src = format!("strand injection {{ {} }}", gene_src);
    let dna = match crate::compiler::compile(&src, None) {
        Ok(dna) => dna,
        Err(e) => {
            app_state.status_msg = format!("Compilation Error: {}", e);
            return Some(true);
        }
    };

    let Some(strand) = dna.helix.strands.first() else {
        return Some(true);
    };

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
            vm.dna.helix.strands.push(strand.clone());
            let new_idx = vm.dna.helix.strands.len() - 1;

            // Save current IP to call stack
            org.call_stack.push(org.ip);
            org.ip = (new_idx, 0);

            found = true;
            app_state.status_msg = format!("Injected code into {}", org.name);
            break;
        }
    }

    if !found {
        app_state.status_msg = "No organelle at cursor.".to_string();
    }

    Some(true)
}

#[cfg(feature = "nova")]
fn apply_genesis_editor(vm: &mut ChimeraVM, app_state: &mut AppState) {
    let src = format!("strand genesis {{ {} }}", app_state.genesis_editor_buffer);
    let dna = match crate::compiler::compile(&src, None) {
        Ok(dna) => dna,
        Err(e) => {
            app_state.status_msg = format!("Compile Error: {}", e);
            return;
        }
    };

    let Some(strand) = dna.helix.strands.first() else {
        return;
    };

    for gene in &strand.genes {
        vm.execute_gene_inner(gene.op.clone(), &gene.args);
    }
    app_state.status_msg = "Genesis: Executed.".to_string();
}

#[cfg(feature = "nova")]
fn apply_genesis_grammar(vm: &mut ChimeraVM, app_state: &mut AppState) -> Option<bool> {
    let exprs = match crate::lisp::parse(&app_state.genesis_grammar_buffer) {
        Ok(exprs) => exprs,
        Err(e) => {
            app_state.status_msg = format!("Lisp Error: {}", e);
            return None;
        }
    };

    let Some(expr) = exprs.first() else {
        app_state.status_msg = "Error: Empty Grammar".to_string();
        return Some(false);
    };

    match crate::lisp::sexpr_to_value(expr) {
        Ok(grammar) => {
            vm.active_grammar = grammar;
            app_state.status_msg = "Genesis: Grammar Updated.".to_string();
        }
        Err(e) => app_state.status_msg = format!("Value Conversion Error: {}", e),
    }
    Some(false)
}

#[cfg(feature = "nova")]
fn handle_genesis_enter(vm: &mut ChimeraVM, app_state: &mut AppState) -> Option<bool> {
    // Commit change based on focus
    if app_state.genesis_focus == 0 {
        apply_genesis_editor(vm, app_state);
        Some(false)
    } else if app_state.genesis_focus == 1 {
        apply_genesis_grammar(vm, app_state)
    } else {
        // Grid
        apply_grid_edit(vm, app_state)
    }
}

#[cfg(feature = "nova")]
fn apply_forge_define_rule(vm: &mut ChimeraVM, app_state: &mut AppState) {
    if app_state.forge_selected_rule.is_empty() {
        return;
    }
    vm.prologue_state.logos_engine.define_rule(
        &app_state.forge_selected_rule,
        &app_state.forge_editor_buffer,
    );
    app_state.status_msg = format!("Forge: Rule '{}' updated.", app_state.forge_selected_rule);
}

#[cfg(feature = "nova")]
fn apply_forge_test_rule(vm: &mut ChimeraVM, app_state: &mut AppState) {
    if app_state.forge_selected_rule.is_empty() {
        return;
    }
    match vm
        .prologue_state
        .logos_engine
        .parse_input(&app_state.forge_selected_rule, &app_state.forge_test_input)
    {
        Ok(val) => {
            app_state.forge_test_output = format!("Success: {}", val);
        }
        Err(e) => {
            app_state.forge_test_output = format!("Error: {}", e);
        }
    }
}

#[cfg(feature = "nova")]
fn handle_forge_enter(vm: &mut ChimeraVM, app_state: &mut AppState) -> Option<bool> {
    if app_state.forge_focus == 1 {
        apply_forge_define_rule(vm, app_state);
    } else if app_state.forge_focus == 2 {
        apply_forge_test_rule(vm, app_state);
    }
    Some(false)
}

pub(crate) fn handle_enter_key(vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {
    let action = match app_state.view_mode {
        #[cfg(feature = "nova")]
        ViewMode::Paradox => handle_paradox_enter(vm, app_state),
        ViewMode::Genome => handle_genome_enter(vm, app_state),
        ViewMode::Grid => apply_grid_edit(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Chronos
        | ViewMode::Orca
        | ViewMode::Hydra
        | ViewMode::Prologue
        | ViewMode::Reactor => apply_grid_edit(vm, app_state),
        #[cfg(feature = "silicon")]
        ViewMode::Foundry => apply_grid_edit(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Babel | ViewMode::Weaver => Some(false),
        #[cfg(feature = "nova")]
        ViewMode::Crispr => {
            handle_crispr_enter(vm, app_state)
            // Stay in Editing mode
        }
        #[cfg(feature = "nova")]
        ViewMode::Logos => handle_logos_enter(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Quipu => handle_quipu_enter(vm, app_state),
        ViewMode::BioticChaos => handle_biotic_chaos_enter(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Virology => handle_virology_enter(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Lexicon => handle_lexicon_enter(vm, app_state),
        ViewMode::Evolution => handle_evolution_enter(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Ecology => handle_ecology_enter(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Genesis => handle_genesis_enter(vm, app_state),
        #[cfg(feature = "nova")]
        ViewMode::Forge => handle_forge_enter(vm, app_state),
        _ => Some(true),
    };
    if let Some(clear_buffer) = action {
        app_state.input_mode = InputMode::Normal;
        if clear_buffer {
            app_state.input_buffer.clear();
        }
    }

    Ok(true)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dna, Helix};

    #[test]
    fn test_handle_genome_enter_empty_input_no_panic() {
        // Initialize an empty VM and AppState
        let mut vm = ChimeraVM::new(Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        });

        let mut app_state = AppState::new(None, None);
        app_state.view_mode = ViewMode::Genome;
        app_state.input_buffer = "".to_string(); // Empty string!

        // Attempt to handle the enter key with the empty string
        let result = handle_genome_enter(&mut vm, &mut app_state);

        // Should return None due to our new guard, and set an error status
        assert_eq!(result, None);
        // Sometimes pest returns a Parse Error right away instead of empty iter, either way it shouldn't panic
        assert!(app_state.status_msg.starts_with("Parse Error: "));
    }

    #[test]
    fn test_handle_genome_enter_invalid_input_no_panic() {
        // Initialize an empty VM and AppState
        let mut vm = ChimeraVM::new(Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        });

        let mut app_state = AppState::new(None, None);
        app_state.view_mode = ViewMode::Genome;
        app_state.input_buffer = "invalid_gene".to_string(); // Invalid!

        // Attempt to handle the enter key
        let result = handle_genome_enter(&mut vm, &mut app_state);

        // Should return None due to parse failure without panicking
        assert_eq!(result, None);
        assert!(app_state.status_msg.starts_with("Parse Error: "));
    }
}
