use super::{normalize_coords, PrologueAgent};
use crate::vm::{ChimeraVM, Value};
use std::collections::{HashMap, HashSet};
#[cfg(feature = "oracle")]
use crate::vm::oracle;

/// The Logic Agent (∃) performs unification on the grid.
///
/// It holds a "Goal" pattern in its state.
/// When it encounters a Fact structure `(...)` on the grid, it attempts to unify the Goal with the Fact.
///
/// If unification succeeds:
/// - Emits a signal (1) to the current cell.
/// - Logs the variable bindings.
///
/// State Structure:
/// [Goal, Dy, Dx]
/// - Goal: A Value representing the pattern to match (e.g. `(foo ?X)`).
/// - Dy, Dx: Direction of movement.
pub fn process_logic_agent(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let mut updated_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);

    // 1. Extract State: [Goal, Dy, Dx]
    let (goal, dy, dx) = match &updated_agent.state {
        Value::Junction(_, list) if list.len() >= 3 => {
            let goal = list[0].clone();
            if let (Value::Int(dy), Value::Int(dx)) = (&list[1], &list[2]) {
                (goal, *dy as i64, *dx as i64)
            } else {
                (Value::Str("?".to_string()), 0, 1)
            }
        }
        _ => (Value::Str("?".to_string()), 0, 1),
    };

    // 1.5 Gate Mode Check (Oracle Integration)
    #[cfg(feature = "oracle")]
    {
        let mut found_vars = HashSet::new();
        collect_vars(&goal, &mut found_vars);

        let gate_vars = ["?N", "?S", "?E", "?W"];
        let is_gate_mode = gate_vars.iter().any(|v| found_vars.contains(*v));

        if is_gate_mode {
            let mut subst = HashMap::new();
            let mut unbound_outputs = Vec::new();

            // Gather Inputs from Grid
            let neighbors = [
                ("?N", -1, 0),
                ("?S", 1, 0),
                ("?E", 0, 1),
                ("?W", 0, -1),
            ];

            for (var, ndy, ndx) in neighbors {
                if found_vars.contains(var) {
                    let mut val = Value::Int(0);
                    if let Some((ny, nx)) = normalize_coords(y as i64 + ndy, x as i64 + ndx) {
                        val = grid_snapshot[ny][nx].clone();
                    }

                    if !is_empty_val(&val) {
                        subst.insert(var.to_string(), val);
                    } else {
                        unbound_outputs.push((var, ndy, ndx));
                    }
                }
            }

            // Solve
            let mut solutions = Vec::new();
            oracle::solve(
                &[goal.clone()],
                subst,
                &vm.knowledge_base,
                vm,
                &mut solutions,
                0
            );

            // Apply Outputs
            if let Some(sol) = solutions.first() {
                // If inputs provided, we likely want the first solution that satisfies them.
                // We write back any variables that were unbound (Empty on grid) but are now bound.
                for (var, ndy, ndx) in unbound_outputs {
                    if let Some(bound_val) = sol.get(var) {
                        if let Some((ny, nx)) = normalize_coords(y as i64 + ndy, x as i64 + ndx) {
                            // Write to VM Grid directly
                            vm.grid[ny][nx] = bound_val.clone();
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1)); // Light up
                        }
                    }
                }
            }
        }
    }

    // 2. Scan ahead for Facts
    if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
        let cell = &grid_snapshot[ny][nx];
        if let Value::Str(s) = cell {
            if s == "(" {
                // Found start of a Fact. Scan it.
                if let Some(fact) = scan_fact(ny, nx, dy, dx, grid_snapshot) {
                    // Attempt Unification
                    if let Some(bindings) = unify(&goal, &fact) {
                        // Success!
                        vm.output
                            .push(format!("∃ Logic: Unified {:?} with {:?}", goal, fact));
                        for (var, val) in bindings {
                            vm.output.push(format!("  {} = {:?}", var, val));
                        }
                        // Emit Signal (1) to current location (to trigger things)
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));

                        // Move past the fact? Or stay?
                        // Let's move past it to avoid re-unifying constantly.
                        // Calculate length of fact on grid (approx).
                        // For now, just move one step (onto the '('), next tick will be inside.
                        // Wait, if we move onto '(', next tick we see 'foo'.
                        // We should skip the whole fact?
                        // Zeta scans wires. Logic Agent should probably just consume it?
                        // Let's just step onto it.
                        return Some((updated_agent, Some((ny, nx))));
                    } else {
                        // Failure
                        vm.output.push(format!(
                            "∃ Logic: Failed to unify {:?} with {:?}",
                            goal, fact
                        ));
                    }
                }
            }
        }
    }

    // 3. Move Logic
    let target = normalize_coords(y as i64 + dy, x as i64 + dx);

    if let Some((ny, nx)) = target {
        // Check if destination is blocked by another agent
        if let Value::Str(s) = &grid_snapshot[ny][nx] {
            if matches!(
                s.as_str(),
                "@" | "K" | "H" | "C" | "♻" | "♬" | "₣" | "ζ" | "∃"
            ) {
                // Blocked - Reverse
                updated_agent.state = Value::Junction(
                    crate::ast::JunctionType::All,
                    vec![goal, Value::Int(-dy), Value::Int(-dx)],
                );
                return Some((updated_agent, None));
            }
        }
        return Some((updated_agent, Some((ny, nx))));
    }

    // Edge of world, reverse
    updated_agent.state = Value::Junction(
        crate::ast::JunctionType::All,
        vec![goal, Value::Int(-dy), Value::Int(-dx)],
    );
    Some((updated_agent, None))
}

fn collect_vars(val: &Value, vars: &mut HashSet<String>) {
    match val {
        Value::Str(s) if s.starts_with('?') => {
            vars.insert(s.clone());
        }
        Value::Junction(_, list) => {
            for item in list {
                collect_vars(item, vars);
            }
        }
        _ => {}
    }
}

/// Scans a Fact structure `(...)` from the grid in the given direction.
/// Returns a `Value::Junction` (List) representing the fact.
fn scan_fact(
    start_y: usize,
    start_x: usize,
    dy: i64,
    dx: i64,
    grid_snapshot: &[Vec<Value>],
) -> Option<Value> {
    let mut values = Vec::new();
    let mut cy = start_y as i64;
    let mut cx = start_x as i64;

    // Skip the opening '('
    cy += dy;
    cx += dx;

    let mut depth = 1;
    const MAX_STEPS: usize = 32;
    let mut steps = 0;

    while steps < MAX_STEPS && depth > 0 {
        if let Some((ny, nx)) = normalize_coords(cy, cx) {
            let cell = &grid_snapshot[ny][nx];
            match cell {
                Value::Str(s) => {
                    if s == "(" {
                        depth += 1;
                        // Nested lists not fully supported in flat scan, but let's just collect tokens
                        values.push(cell.clone());
                    } else if s == ")" {
                        depth -= 1;
                        if depth == 0 {
                            break; // Done
                        }
                        values.push(cell.clone()); // Add nested ')'
                    } else if s == "~" {
                        // Ignore wires
                    } else {
                        values.push(cell.clone());
                    }
                }
                val => {
                    if !is_empty_val(val) {
                        values.push(val.clone());
                    }
                }
            }
            cy += dy;
            cx += dx;
            steps += 1;
        } else {
            return None; // Run off grid
        }
    }

    if depth == 0 {
        Some(Value::Junction(crate::ast::JunctionType::All, values))
    } else {
        None // Malformed
    }
}

fn is_empty_val(v: &Value) -> bool {
    match v {
        Value::Int(0) => true,
        Value::Str(s) => s.is_empty(),
        _ => false,
    }
}

/// Unifies a Goal pattern with a Fact.
/// Returns a map of variable bindings if successful.
/// Variables in Goal start with '?'.
fn unify(goal: &Value, fact: &Value) -> Option<HashMap<String, Value>> {
    let mut bindings = HashMap::new();
    if unify_recursive(goal, fact, &mut bindings) {
        Some(bindings)
    } else {
        None
    }
}

fn unify_recursive(goal: &Value, fact: &Value, bindings: &mut HashMap<String, Value>) -> bool {
    // 1. Check Goal Variables
    if let Value::Str(s) = goal {
        if s.starts_with('?') {
            // Variable
            if let Some(bound_val) = bindings.get(s).cloned() {
                // Already bound, check consistency
                return unify_recursive(&bound_val, fact, bindings);
            } else {
                // Bind it
                bindings.insert(s.clone(), fact.clone());
                return true;
            }
        }
        if s == "_" {
            return true; // Wildcard matches anything
        }
    }

    // 2. Check Structures
    match (goal, fact) {
        (Value::Junction(_, g_list), Value::Junction(_, f_list)) => {
            if g_list.len() != f_list.len() {
                return false;
            }
            for (g, f) in g_list.iter().zip(f_list.iter()) {
                if !unify_recursive(g, f, bindings) {
                    return false;
                }
            }
            true
        }
        (Value::Int(a), Value::Int(b)) => a == b,
        (Value::Str(a), Value::Str(b)) => a == b,
        _ => false, // Type mismatch
    }
}
