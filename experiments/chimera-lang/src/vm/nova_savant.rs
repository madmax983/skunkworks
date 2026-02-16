#![cfg(feature = "nova")]

use crate::ast::{JunctionType, Nucleotide};
use crate::opcode::OpCode;
use crate::vm::nova::Organelle;
#[cfg(feature = "oracle")]
use crate::vm::oracle;
use crate::vm::{ChimeraVM, Value};
use std::collections::HashMap;

pub fn process_savant(vm: &mut ChimeraVM, organelle: &mut Organelle) {
    #[cfg(not(feature = "oracle"))]
    {
        // Without Oracle, Savant is just a Worker
        return;
    }

    #[cfg(feature = "oracle")]
    {
        // 1. Construct Knowledge Base from DNA
        // We simulate the execution of the strand to build a temporary KB.
        // This is safe because Rule/Assert only modify the KB, not the world state (mostly).
        // However, we don't want side effects.
        // So we just scan for specific patterns:
        // Push(A), Push(B), Rule -> rule(B, A) (Note: Rule pops Body then Head, so Stack: [Body, Head] -> Rule)
        // Wait, OpCode::Rule pops Body, then Head.
        // Stack: [Body, Head]. Rule -> rule(Head, Body).
        // Compiler: (rule Head Body) -> Push(Head), Push(Body), Rule.
        // So executing them: Push(Head) -> Stack[Head]; Push(Body) -> Stack[Head, Body]; Rule -> pops Body, pops Head.

        let s_idx = organelle.ip.0;
        if s_idx >= vm.dna.helix.strands.len() {
            return;
        }

        let strand = &vm.dna.helix.strands[s_idx];
        let mut kb = Vec::new();
        let mut sim_stack: Vec<Value> = Vec::new();

        // Limit gene scan to prevent DOS
        for gene in strand.genes.iter().take(100) {
            match gene.op {
                OpCode::Push => {
                    if !gene.args.is_empty() {
                        if let Some(val) = nucleotide_to_value(&gene.args[0]) {
                            sim_stack.push(val);
                        }
                    }
                }
                OpCode::Rule => {
                    if sim_stack.len() >= 2 {
                        let body = sim_stack.pop().unwrap();
                        let head = sim_stack.pop().unwrap();

                        let mut rule_args = vec![Value::Str("rule".to_string()), head];
                        match body {
                            Value::Junction(JunctionType::All, body_terms) => {
                                rule_args.extend(body_terms);
                            }
                            _ => {
                                rule_args.push(body);
                            }
                        }
                        kb.push(Value::Junction(JunctionType::Any, rule_args));
                    }
                }
                OpCode::Assert => {
                    if let Some(fact) = sim_stack.pop() {
                        kb.push(fact);
                    }
                }
                _ => {
                    // Ignore other ops for KB construction
                    // Or maybe we should allow partial execution?
                    // For safety, we only interpret literals for now.
                }
            }
        }

        // 2. Add Sensory Facts
        // neighbor(Dir, Val)
        let (cy, cx) = organelle.context_loc;
        let neighbors = [
            (-1, 0, "north"),
            (1, 0, "south"),
            (0, 1, "east"),
            (0, -1, "west"),
        ];

        for (dy, dx, dir_name) in neighbors {
            if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                let val = vm.grid[ny][nx].clone();
                // fact: neighbor(dir, val)
                let fact = Value::Junction(
                    JunctionType::Any,
                    vec![
                        Value::Str("neighbor".to_string()),
                        Value::Str(dir_name.to_string()),
                        val,
                    ],
                );
                kb.push(fact);
            } else {
                // Boundary
                let fact = Value::Junction(
                    JunctionType::Any,
                    vec![
                        Value::Str("neighbor".to_string()),
                        Value::Str(dir_name.to_string()),
                        Value::Str("void".to_string()),
                    ],
                );
                kb.push(fact);
            }
        }

        // Add self status
        // energy(E)
        kb.push(Value::Junction(
            JunctionType::Any,
            vec![Value::Str("energy".to_string()), Value::Int(vm.energy)],
        ));

        // 3. Query for Action
        // Goal: action(?A)
        let goal = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("action".to_string()), Value::Str("?A".to_string())],
        );

        let mut solutions = Vec::new();
        oracle::solve(
            &[goal],
            HashMap::new(),
            &kb,
            vm,
            &mut solutions,
            0,
        );

        // 4. Execute Action
        // We take the first solution
        if let Some(sol) = solutions.first() {
            if let Some(action) = sol.get("?A") {
                execute_savant_action(vm, organelle, action);
            }
        }
    }
}

fn nucleotide_to_value(n: &Nucleotide) -> Option<Value> {
    match n {
        Nucleotide::Number(i) => Some(Value::Int(*i)),
        Nucleotide::String(s) => Some(Value::Str(s.clone())),
        Nucleotide::Identifier(s) => Some(Value::Str(s.clone())), // Identifiers are Strings in Logic
        Nucleotide::Junction(t, args) => {
            let mut vals = Vec::new();
            for arg in args {
                if let Some(v) = nucleotide_to_value(arg) {
                    vals.push(v);
                } else {
                    return None;
                }
            }
            Some(Value::Junction(*t, vals))
        }
    }
}

fn execute_savant_action(vm: &mut ChimeraVM, organelle: &mut Organelle, action: &Value) {
    // Action can be:
    // - move(Dir)
    // - consume
    // - mitosis
    // - OpCode string?

    if let Value::Junction(JunctionType::Any, args) = action {
        if let Some(Value::Str(cmd)) = args.first() {
            match cmd.as_str() {
                "move" => {
                    if args.len() >= 2 {
                        if let Value::Str(dir) = &args[1] {
                            let (dy, dx) = match dir.as_str() {
                                "north" => (-1, 0),
                                "south" => (1, 0),
                                "east" => (0, 1),
                                "west" => (0, -1),
                                _ => (0, 0),
                            };
                            organelle.direction = (dy, dx);

                            // Try to move physically
                            let (cy, cx) = organelle.context_loc;
                            if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy as i64, cx as i64 + dx as i64) {
                                // Check collision
                                // Simple collision: if cell is empty?
                                // Or Savants can share space?
                                // Let's allow sharing for now, or check for Wall.
                                // Actually, migrate op checks membranes.
                                // Let's emulate migrate logic simply.
                                organelle.context_loc = (ny, nx);
                                vm.energy = vm.energy.saturating_sub(1);
                                vm.output.push(format!("SAVANT: Moved {}", dir));
                            }
                        }
                    }
                }
                "consume" => {
                    // Consume local cell
                    let (cy, cx) = organelle.context_loc;
                    let val = &vm.grid[cy][cx];
                    match val {
                        Value::Int(n) => vm.energy = vm.energy.saturating_add(n),
                        Value::Str(s) => vm.energy = vm.energy.saturating_add(s.len() as i64),
                        _ => {}
                    }
                    vm.grid[cy][cx] = Value::Int(0); // Eat it
                    vm.output.push("SAVANT: Consumed".to_string());
                }
                "mitosis" => {
                    // Clone self
                    if vm.organelles.len() < crate::vm::MAX_ORGANELLES && vm.energy > 50 {
                        let mut child = organelle.clone();
                        child.id = vm.organelle_id_counter + 1;
                        vm.organelle_id_counter += 1;
                        child.name = format!("{} II", organelle.name);

                        // Mutate child DNA? No, mitosis is cloning.

                        vm.organelles.push(child);
                        vm.energy -= 50;
                        vm.output.push("SAVANT: Mitosis".to_string());
                    }
                }
                "write" => {
                    // write(Val)
                    if args.len() >= 2 {
                        let val = args[1].clone();
                        let (cy, cx) = organelle.context_loc;
                        vm.grid[cy][cx] = val;
                        vm.energy = vm.energy.saturating_sub(1);
                    }
                }
                _ => {}
            }
        }
    } else if let Value::Str(s) = action {
        // Simple command
        match s.as_str() {
            "die" => {
                organelle.halted = true;
                vm.output.push("SAVANT: Accepted death".to_string());
            }
            _ => {}
        }
    }
}
