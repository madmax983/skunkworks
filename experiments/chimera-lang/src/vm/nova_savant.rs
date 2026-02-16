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
        // 1. Inject Sensors (Transient)
        let sensor_facts = inject_sensors(vm, organelle);

        // 2. Execute DNA (Burst Mode)
        // Execute up to 50 instructions per tick to build KB / deduce
        let mut steps = 0;
        let max_steps = 50;

        while steps < max_steps && !organelle.halted {
            if vm.ip.0 >= vm.dna.helix.strands.len() {
                break;
            }
            let strand_len = vm.dna.helix.strands[vm.ip.0].genes.len();
            if vm.ip.1 >= strand_len {
                vm.ip.0 += 1;
                vm.ip.1 = 0;
                continue;
            }

            let (gene_op, gene_args) = {
                let gene = &vm.dna.helix.strands[vm.ip.0].genes[vm.ip.1];
                (gene.op.clone(), gene.args.clone())
            };

            // Execute
            let jump_target = vm.execute_gene(gene_op, &gene_args);

            if let Some(target) = jump_target {
                vm.ip = target;
            } else {
                vm.ip.1 += 1;
            }
            steps += 1;
        }

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
            &vm.knowledge_base,
            vm,
            &mut solutions,
            0,
        );

        // 4. Execute Action
        if let Some(sol) = solutions.first() {
            if let Some(action) = sol.get("?A") {
                execute_savant_action(vm, organelle, action);
            }
        }

        // 5. Cleanup Sensors
        // Retract facts we added to avoid pollution
        for fact in sensor_facts {
            if let Some(pos) = vm.knowledge_base.iter().position(|x| *x == fact) {
                vm.knowledge_base.remove(pos);
            }
        }
    }
}

#[cfg(feature = "oracle")]
fn inject_sensors(vm: &mut ChimeraVM, organelle: &mut Organelle) -> Vec<Value> {
    let mut added = Vec::new();

    // neighbor(Dir, Val)
    // Note: We use vm.context_loc because tick_organelle swaps the organelle's location into vm.context_loc
    // before calling this function. organelle.context_loc currently holds the swapped-out VM context.
    let (cy, cx) = vm.context_loc;

    let neighbors = [
        (-1, 0, "north"),
        (1, 0, "south"),
        (0, 1, "east"),
        (0, -1, "west"),
    ];

    for (dy, dx, dir_name) in neighbors {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
            let val = vm.grid[ny][nx].clone();
            let fact = Value::Junction(
                JunctionType::Any,
                vec![
                    Value::Str("neighbor".to_string()),
                    Value::Str(dir_name.to_string()),
                    val,
                ],
            );
            if !vm.knowledge_base.contains(&fact) {
                vm.knowledge_base.push(fact.clone());
                added.push(fact);
            }
        } else {
            let fact = Value::Junction(
                JunctionType::Any,
                vec![
                    Value::Str("neighbor".to_string()),
                    Value::Str(dir_name.to_string()),
                    Value::Str("void".to_string()),
                ],
            );
            if !vm.knowledge_base.contains(&fact) {
                vm.knowledge_base.push(fact.clone());
                added.push(fact);
            }
        }
    }

    // energy(E)
    let fact_energy = Value::Junction(
        JunctionType::Any,
        vec![Value::Str("energy".to_string()), Value::Int(vm.energy)],
    );
    if !vm.knowledge_base.contains(&fact_energy) {
        vm.knowledge_base.push(fact_energy.clone());
        added.push(fact_energy);
    }

    added
}

fn execute_savant_action(vm: &mut ChimeraVM, organelle: &mut Organelle, action: &Value) {
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
                            organelle.direction = (dy, dx); // Update direction

                            let (cy, cx) = vm.context_loc;
                            if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy as i64, cx as i64 + dx as i64) {
                                // Simple collision check: if grid is empty (0)
                                if let Value::Int(0) = vm.grid[ny][nx] {
                                    vm.context_loc = (ny, nx);
                                    vm.energy = vm.energy.saturating_sub(1);
                                    vm.output.push(format!("SAVANT: Moved {}", dir));
                                } else {
                                    vm.output.push(format!("SAVANT: Blocked {}", dir));
                                }
                            }
                        }
                    }
                }
                "consume" => {
                    let (cy, cx) = vm.context_loc;
                    let val = &vm.grid[cy][cx];
                    match val {
                        Value::Int(n) => vm.energy = vm.energy.saturating_add(n),
                        Value::Str(s) => vm.energy = vm.energy.saturating_add(s.len() as i64),
                        _ => {}
                    }
                    vm.grid[cy][cx] = Value::Int(0);
                    vm.output.push("SAVANT: Consumed".to_string());
                }
                "mitosis" => {
                    if vm.organelles.len() < crate::vm::MAX_ORGANELLES && vm.energy > 50 {
                        let mut child = organelle.clone();
                        child.id = vm.organelle_id_counter + 1;
                        vm.organelle_id_counter += 1;
                        child.name = format!("{} II", organelle.name);
                        vm.organelles.push(child);
                        vm.energy -= 50;
                        vm.output.push("SAVANT: Mitosis".to_string());
                    }
                }
                "write" => {
                    if args.len() >= 2 {
                        let val = args[1].clone();
                        let (cy, cx) = vm.context_loc;
                        vm.grid[cy][cx] = val;
                        vm.energy = vm.energy.saturating_sub(1);
                    }
                }
                "execute" => {
                    // execute(OpCode, Arg) or execute(OpCode)
                    if args.len() >= 2 {
                        if let Value::Str(op_str) = &args[1] {
                            if let Ok(op) = op_str.parse::<OpCode>() {
                                let gene_args = if args.len() >= 3 {
                                    if let Value::Int(n) = args[2] {
                                        vec![Nucleotide::Number(n)]
                                    } else if let Value::Str(s) = &args[2] {
                                        vec![Nucleotide::String(s.clone())]
                                    } else {
                                        vec![]
                                    }
                                } else {
                                    vec![]
                                };

                                let _ = vm.execute_gene(op, &gene_args);
                                vm.output.push(format!("SAVANT: Executed {}", op_str));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    } else if let Value::Str(s) = action {
        match s.as_str() {
            "die" => {
                organelle.halted = true;
                vm.output.push("SAVANT: Accepted death".to_string());
            }
            _ => {}
        }
    }
}
