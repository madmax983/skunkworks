use crate::ast::JunctionType;
use crate::vm::oracle::solve;
use crate::vm::ChimeraVM;
use crate::vm::Value;
use std::collections::HashMap;

/// The Prolog Agent (℘)
/// Reads a Query from West (String or Junction)
/// Evaluates it against `vm.knowledge_base`
/// Outputs the result (1 for true, 0 for false, or a Junction of bindings) to East.
pub fn process_prolog_agent(vm: &mut ChimeraVM, agent_y: usize, agent_x: usize) {
    let mut query_val = None;

    // Scan West for query
    if let Some((wy, wx)) = vm.normalize_coords(agent_y as i64, agent_x as i64 - 1) {
        if let Some(sig) = &vm.prologue_state.signal_grid[wy][wx] {
            query_val = Some(sig.clone());
            vm.prologue_state.signal_grid[wy][wx] = None;
        } else {
            let val = vm.grid[wy][wx].clone();
            if !matches!(val, Value::Int(0)) && !matches!(val, Value::Str(ref s) if s == "." || s == "0") {
                query_val = Some(val);
                vm.grid[wy][wx] = Value::Int(0);
            }
        }
    }

    if let Some(query) = query_val {
        let goals = match query {
            Value::Junction(_, terms) => terms,
            _ => vec![query],
        };

        let mut solutions = Vec::new();
        // Since we are borrowing `vm` to get the KB, we must clone it to pass to solve.
        let kb = vm.knowledge_base.clone();
        solve(&goals, HashMap::new(), &kb, vm, &mut solutions, 0);

        let result = if solutions.is_empty() {
            Value::Int(0)
        } else {
            // Build a list of solutions
            let mut sol_vals = Vec::new();
            for sol in solutions {
                let mut bindings = Vec::new();
                for (k, v) in sol {
                    bindings.push(Value::Str(format!("{}={}", k, v)));
                }
                sol_vals.push(Value::Junction(JunctionType::Any, bindings));
            }
            if sol_vals.len() == 1 {
                sol_vals.pop().unwrap()
            } else {
                Value::Junction(JunctionType::Any, sol_vals)
            }
        };

        // Output to East
        if let Some((ey, ex)) = vm.normalize_coords(agent_y as i64, agent_x as i64 + 1) {
            vm.prologue_state.signal_grid[ey][ex] = Some(result);
        }
    }
}
