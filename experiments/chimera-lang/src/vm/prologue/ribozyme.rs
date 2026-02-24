use super::{normalize_coords, PrologueAgent};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

/// The Ribozyme Agent (🛠) is a Programmable Enzyme.
///
/// It reads:
/// - **West**: Reactant (Input Value or List)
/// - **North**: Enzyme (Function/Code as List of OpCodes/Strings)
///
/// It executes the Enzyme on the Reactant and writes the result to **East**.
///
/// # Mechanics
/// 1. Scans West and North.
/// 2. If both are present, it compiles the Enzyme.
/// 3. It creates a temporary execution context (using a temporary stack).
/// 4. Pushes the Reactant onto the stack.
/// 5. Executes the Enzyme genes.
/// 6. Pops the result from the stack and writes it to East.
/// 7. Consumes the Reactant (West).
pub fn process_ribozyme_agent(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let current_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);

    // 1. Scan Neighbors
    let w_pos = normalize_coords(y as i64, x as i64 - 1);
    let n_pos = normalize_coords(y as i64 - 1, x as i64);

    if let (Some((wy, wx)), Some((ny, nx))) = (w_pos, n_pos) {
        let reactant = &grid_snapshot[wy][wx];
        let enzyme = &grid_snapshot[ny][nx];

        // Ensure inputs exist
        let has_reactant = !matches!(reactant, Value::Int(0));
        let has_enzyme = !matches!(enzyme, Value::Int(0));

        if has_reactant && has_enzyme {
            // 2. Prepare Execution
            // Convert Enzyme to Gene List
            let genes = match enzyme {
                Value::Junction(_, list) => {
                    let mut genes = Vec::new();
                    for val in list {
                        if let Value::Str(s) = val {
                            if let Ok(op) = s.parse::<OpCode>() {
                                genes.push(crate::ast::Gene { op, args: vec![] });
                            }
                        }
                    }
                    genes
                }
                Value::Str(s) => {
                    if let Ok(op) = s.parse::<OpCode>() {
                        vec![crate::ast::Gene { op, args: vec![] }]
                    } else {
                        vec![]
                    }
                }
                _ => vec![],
            };

            if !genes.is_empty() {
                // 3. Execute
                // Save current stack
                let original_stack = std::mem::take(&mut vm.stack);

                // Push Reactant
                vm.stack.push(reactant.clone());

                // Run Genes
                for gene in genes {
                    vm.execute_gene_inner(gene.op, &gene.args);
                }

                // Pop Result
                let result = vm.stack.pop().unwrap_or(Value::Int(0));

                // Restore stack
                vm.stack = original_stack;

                // 4. Write Result to East
                if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                    vm.grid[ey][ex] = result;
                }

                // 5. Consume Reactant
                vm.grid[wy][wx] = Value::Int(0);

                // Signal Success
                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));

                // Do not move
                return Some((current_agent, None));
            }
        }
    }

    // Move Logic: Random Walk (Brownian Motion)
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut rng = rand::thread_rng();
    use rand::seq::SliceRandom;

    let target_pos = neighbors
        .choose(&mut rng)
        .and_then(|(dy, dx)| normalize_coords(y as i64 + dy, x as i64 + dx))
        .and_then(|(ny, nx)| match &grid_snapshot[ny][nx] {
            Value::Int(0) => Some((ny, nx)),
            _ => None,
        });

    Some((current_agent, target_pos))
}
