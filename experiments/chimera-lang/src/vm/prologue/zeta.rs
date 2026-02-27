use super::{normalize_coords, PrologueAgent};
use crate::lisp::compile_fragment;
use crate::vm::{ChimeraVM, Value};

/// The Zeta Agent (ζ) executes Lisp-like expressions found on the grid.
///
/// It scans in its current direction (default East) following wires (`~`)
/// and collecting tokens. It reconstructs a string, compiles it using the
/// Lisp compiler, and executes the resulting genes.
///
/// Example:
///   ζ ~ ( ~ + ~ 1 ~ 2 ~ )
///
/// Execution:
///   1. Scans tokens: "(", "+", "1", "2", ")"
///   2. Compiles: "( + 1 2 )" -> [Push(1), Push(2), Add]
///   3. Executes: Pushes 3 to the stack.
pub fn process_zeta_agent(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let mut updated_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);

    // 1. Determine Direction from State (or default East)
    let (dy, dx) = match &updated_agent.state {
        Value::Junction(_, list) if list.len() >= 2 => {
            if let (Value::Int(dy), Value::Int(dx)) = (&list[0], &list[1]) {
                (*dy as i64, *dx as i64)
            } else {
                (0, 1)
            }
        }
        _ => (0, 1),
    };

    // 2. Scan and Execute Logic
    // Only execute if there is a wire or token immediately in front?
    // Or just scan.
    if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
        let tokens = scan_wire_path(ny, nx, dy, dx, grid_snapshot);
        if !tokens.is_empty() {
            // Reconstruct Lisp string
            let source = tokens.join(" ");
            match compile_fragment(&source) {
                Ok(genes) => {
                    // Execute genes
                    // We need to execute on the agent's stack?
                    // The genes modify the VM stack (which is usually the main stack).
                    // But PrologueAgents have their own stack.
                    // We should temporarily swap the VM stack with the Agent stack?
                    // Or execute on a temporary stack and push result?

                    // Swap stacks
                    let mut agent_stack = std::mem::take(&mut updated_agent.stack);
                    let vm_stack = std::mem::take(&mut vm.stack);

                    // Use agent stack as VM stack
                    vm.stack = agent_stack;

                    // Set context location for GRead/GWrite to agent's position
                    let old_context = vm.context_loc;
                    vm.context_loc = (y, x);

                    for gene in genes {
                        let _ = vm.execute_gene_inner(gene.op, &gene.args);
                    }

                    // Restore context
                    vm.context_loc = old_context;

                    // Retrieve stack
                    agent_stack = std::mem::take(&mut vm.stack);
                    updated_agent.stack = agent_stack;

                    // Restore VM stack
                    vm.stack = vm_stack;

                    // Log success
                    // vm.output.push(format!("ζ Executed: {}", source));
                }
                Err(e) => {
                    vm.output.push(format!("ζ Error: {}", e));
                }
            }
        }
    }

    // 3. Move Logic (Same as other agents)
    // Move forward if possible, else turn or stay.
    // Simple logic: Move forward if empty or wire.
    let target = normalize_coords(y as i64 + dy, x as i64 + dx);

    if let Some((ny, nx)) = target {
        // Check if destination is blocked by another agent
        if let Value::Str(s) = &grid_snapshot[ny][nx] {
            if matches!(s.as_str(), "@" | "K" | "H" | "C" | "♻" | "♬" | "₣" | "ζ") {
                // Blocked - Turn Randomly? Or Reverse?
                // Let's reverse for now.
                updated_agent.state = Value::Junction(
                    crate::ast::JunctionType::All,
                    vec![Value::Int(-dy), Value::Int(-dx)],
                );
                return Some((updated_agent, None));
            }
        }

        // Move if not blocked (we overwrite/move onto wires/empty)
        // Check strict blocking for walls?
        // Assuming wires are walkable.

        return Some((updated_agent, Some((ny, nx))));
    }

    // Hit edge, reverse
    updated_agent.state = Value::Junction(
        crate::ast::JunctionType::All,
        vec![Value::Int(-dy), Value::Int(-dx)],
    );
    Some((updated_agent, None))
}

fn scan_wire_path(
    start_y: usize,
    start_x: usize,
    dy: i64,
    dx: i64,
    grid_snapshot: &[Vec<Value>],
) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut cy = start_y as i64;
    let mut cx = start_x as i64;
    let mut steps = 0;
    const MAX_STEPS: usize = 64;

    while steps < MAX_STEPS {
        if let Some((ny, nx)) = normalize_coords(cy, cx) {
            let cell = &grid_snapshot[ny][nx];
            match cell {
                Value::Str(s) => {
                    if s == "~" {
                        // Wire - continue scanning
                    } else if s == "(" || s == ")" || s == "[" || s == "]" || s == "{" || s == "}" {
                        tokens.push(s.clone());
                    } else if s == "ζ" {
                        // Hit another agent or self (loop), stop
                        break;
                    } else if !s.is_empty() {
                        // Likely an atom (OpCode or String)
                        tokens.push(s.clone());
                    } else {
                        // Empty string - stop
                        break;
                    }
                }
                Value::Int(n) => {
                    if *n == 0 {
                        // Empty cell (0) - stop
                        break;
                    }
                    tokens.push(n.to_string());
                }
                _ => break,
            }

            // Advance
            cy += dy;
            cx += dx;
            steps += 1;
        } else {
            break;
        }
    }
    tokens
}
