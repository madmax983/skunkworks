#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
use crate::ast::{Nucleotide, Strand};
use crate::{ChimeraParser, Rule};
use pest::Parser;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn exec_fossilize(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., strand_idx ]
    if let Some(Value::Int(idx)) = vm.stack.pop() {
        let s_idx = idx as usize;
        if s_idx < vm.dna.helix.strands.len() {
            let strand = &vm.dna.helix.strands[s_idx];

            // 1. Calculate Hash
            let mut hasher = DefaultHasher::new();
            strand.hash(&mut hasher);
            let hash = hasher.finish();

            // 2. Serialize to String (Decompile logic)
            // We use a simplified version of the logic in nova.rs
            fn format_nucleotide(n: &Nucleotide, depth: usize) -> String {
                if depth > 10 {
                    return "...".to_string();
                }
                match n {
                    Nucleotide::Number(i) => i.to_string(),
                    Nucleotide::String(s) => format!("\"{}\"", s),
                    Nucleotide::Identifier(s) => s.clone(),
                    Nucleotide::Junction(t, args) => {
                        let t_str = match t {
                            crate::ast::JunctionType::Any => "any",
                            crate::ast::JunctionType::All => "all",
                            crate::ast::JunctionType::Dish => "dish",
                        };
                        let args_str: Vec<String> = args
                            .iter()
                            .map(|arg| format_nucleotide(arg, depth + 1))
                            .collect();
                        format!("{}({})", t_str, args_str.join(" "))
                    }
                }
            }

            let mut dna_str = String::from("[ ");
            for gene in &strand.genes {
                dna_str.push_str(gene.op.as_ref());
                dna_str.push('(');
                for (i, arg) in gene.args.iter().enumerate() {
                    if i > 0 {
                        dna_str.push(' ');
                    }
                    dna_str.push_str(&format_nucleotide(arg, 0));
                }
                dna_str.push(')');
                dna_str.push(' ');
            }
            dna_str.push(']');

            // 3. Format Fossil String
            let fossil_str = format!("Fossil:{}:{}:{}", vm.tick_counter, hash, dna_str);

            // 4. Write to Grid
            let (cy, cx) = vm.context_loc;
            vm.grid[cy][cx] = Value::Str(fossil_str);

            vm.energy = vm.energy.saturating_sub(25); // Cost to fossilize
            vm.output.push(format!(
                "FOSSILIZE: Preserved strand {} at {},{}",
                s_idx, cx, cy
            ));
        } else {
            vm.output
                .push("Error: Strand index out of bounds for fossilize".to_string());
        }
    } else {
        vm.output
            .push("Error: Type mismatch or stack underflow for fossilize".to_string());
    }
    None
}

pub fn exec_unearth(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., y, x ]
    if vm.stack.len() >= 2 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();

        if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                let fossil_content = if let Value::Str(s) = &vm.grid[ny][nx] {
                    if s.starts_with("Fossil:") {
                        Some(s.clone())
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(s) = fossil_content {
                    let parts: Vec<&str> = s.splitn(4, ':').collect();
                    if parts.len() == 4 {
                        let _tick = parts[1];
                        let _hash = parts[2];
                        let dna_content = parts[3];

                        // Parse
                        match ChimeraParser::parse(Rule::strand, dna_content) {
                            Ok(mut pairs) => {
                                let pair = pairs.next().unwrap();
                                match Strand::try_from_pair(pair) {
                                    Ok(strand) => {
                                        vm.dna.helix.strands.push(strand);
                                        vm.telomeres.push(50);
                                        #[cfg(feature = "cortex")]
                                        {
                                            vm.activation_levels.push(0);
                                            vm.synapse_map.push(Vec::new());
                                        }
                                        let new_idx = vm.dna.helix.strands.len() - 1;

                                        // Register ancestry
                                        vm.cladistics.register_strand(
                                            new_idx,
                                            vec![vm.ip.0],
                                            vm.tick_counter,
                                            "Unearth".to_string(),
                                        );

                                        vm.stack.push(Value::Int(new_idx as i64));
                                        vm.energy = vm.energy.saturating_sub(15);
                                        vm.output.push(format!(
                                            "UNEARTH: Recovered strand from fossil at {},{}",
                                            nx, ny
                                        ));
                                    }
                                    Err(e) => {
                                        vm.output
                                            .push(format!("UNEARTH ERROR: Invalid DNA: {}", e));
                                        vm.stack.push(Value::Int(-1));
                                    }
                                }
                            }
                            Err(e) => {
                                vm.output
                                    .push(format!("UNEARTH ERROR: Parse failed: {}", e));
                                vm.stack.push(Value::Int(-1));
                            }
                        }
                    } else {
                        vm.output.push("UNEARTH: Malformed fossil data".to_string());
                        vm.stack.push(Value::Int(-1));
                    }
                } else {
                    vm.output
                        .push("UNEARTH: No fossil found at location".to_string());
                    vm.stack.push(Value::Int(-1));
                }
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for unearth".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for unearth".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for unearth".to_string());
    }
    None
}

pub fn exec_carbon_date(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., y, x ]
    if vm.stack.len() >= 2 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();

        if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                if let Value::Str(s) = &vm.grid[ny][nx] {
                    if s.starts_with("Fossil:") {
                        let parts: Vec<&str> = s.splitn(4, ':').collect();
                        if parts.len() >= 2 {
                            if let Ok(tick) = parts[1].parse::<u64>() {
                                let age = vm.tick_counter.saturating_sub(tick);
                                vm.stack.push(Value::Int(age as i64));
                            } else {
                                vm.stack.push(Value::Int(-1)); // Error parsing tick
                            }
                        } else {
                            vm.stack.push(Value::Int(-1));
                        }
                    } else {
                        vm.stack.push(Value::Int(-1)); // Not a fossil
                    }
                } else {
                    vm.stack.push(Value::Int(-1));
                }
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for carbon_date".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for carbon_date".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for carbon_date".to_string());
    }
    None
}
