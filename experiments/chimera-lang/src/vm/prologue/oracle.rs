use super::normalize_coords;
#[cfg(feature = "oracle")]
use crate::ast::JunctionType;
#[cfg(feature = "oracle")]
use crate::vm::oracle;
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "oracle")]
use std::collections::HashMap;

#[cfg(feature = "oracle")]
pub fn apply_oracle_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "¶" => {
            // Assert: West (Fact) -> KB
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(fact) = &vm.prologue_state.signal_grid[wy][wx] {
                    // Avoid duplicates
                    if !vm.knowledge_base.contains(fact) {
                        vm.knowledge_base.push(fact.clone());
                        vm.output.push(format!("PROLOGUE: Asserted {}", fact));
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        // Feedback
                    }
                }
            }
        }
        "λ" => {
            // Query: West (Goal) -> Solve -> South (Result)
            // Result is boolean (1/0) or maybe the first binding?
            // For now, let's output 1 if true, 0 if false.
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(goal) = &vm.prologue_state.signal_grid[wy][wx] {
                    let goals = match goal {
                        Value::Junction(JunctionType::All, args) => args.clone(),
                        _ => vec![goal.clone()],
                    };

                    let mut solutions = Vec::new();
                    // We need a read-only view of VM for solve
                    // We hold &mut vm.
                    // safe to reborrow as immutable
                    oracle::solve(
                        &goals,
                        HashMap::new(),
                        &vm.knowledge_base,
                        vm,
                        &mut solutions,
                        0,
                    );

                    let result = if !solutions.is_empty() {
                        Value::Int(1)
                    } else {
                        Value::Int(0)
                    };

                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.grid[sy][sx] = result;
                        vm.prologue_state.signal_grid[y][x] =
                            Some(Value::Int(solutions.len() as i64));
                    }
                }
            }
        }
        _ => {}
    }

    match rune {
        "¥" => {
            // Retract: West (Fact) -> Remove from KB
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(fact) = &vm.prologue_state.signal_grid[wy][wx] {
                    if let Some(pos) = vm.knowledge_base.iter().position(|x| x == fact) {
                        vm.knowledge_base.remove(pos);
                        vm.output.push(format!("PROLOGUE: Retracted {}", fact));
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        "∃" => {
            // Omen: West (Condition), North (Effect) -> Register Omen
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(condition) = &vm.prologue_state.signal_grid[wy][wx] {
                    if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                        if let Some(effect) = &vm.prologue_state.signal_grid[ny][nx] {
                            let omen = oracle::Omen {
                                condition: condition.clone(),
                                effect: effect.clone(),
                            };
                            vm.omens.push(omen);
                            vm.output.push(format!(
                                "PROLOGUE: Omen Registered: {} -> {}",
                                condition, effect
                            ));
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

#[cfg(not(feature = "oracle"))]
pub fn apply_oracle_sinks(_vm: &mut ChimeraVM, _rune: &str, _y: usize, _x: usize) {}
