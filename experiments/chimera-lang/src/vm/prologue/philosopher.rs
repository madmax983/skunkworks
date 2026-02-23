use super::{normalize_coords, PrologueAgent};
use crate::ast::JunctionType;
#[cfg(feature = "oracle")]
use crate::vm::oracle;
use crate::vm::{ChimeraVM, Value};
use std::collections::HashMap;

/// Processes the logic for a Philosopher Agent ('Φ').
///
/// The Philosopher uses the Prologue Oracle to determine its actions based on its Goal.
/// State: "goal(Term)" or just Term.
pub fn process_philosopher_logic(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    _grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    #[cfg(not(feature = "oracle"))]
    {
        // Without Oracle, Philosopher meditates (does nothing)
        return Some((agent.clone(), None));
    }

    #[cfg(feature = "oracle")]
    {
        let mut updated_agent = agent.clone();
        let (cy, cx) = (agent.y, agent.x);

        // 1. Extract Goal from State
        let goal = if let Value::Str(s) = &agent.state {
            if s.starts_with("goal:") {
                // Parse? For now, we assume the state IS the goal string if simple,
                // or we use a parser. Let's assume the state is a Value::Str that represents a term.
                // We'll construct a goal term: "solve(Goal)"?
                // Actually, let's just treat the state string as the goal predicate name or term.
                Value::Str(s.trim_start_matches("goal:").to_string())
            } else {
                Value::Str(s.clone())
            }
        } else {
            agent.state.clone()
        };

        // If no goal, default to "exist"
        let effective_goal = if matches!(goal, Value::Int(0)) {
            Value::Str("exist".to_string())
        } else {
            goal
        };

        // 2. Inject Local Perception into Temporary KB
        let mut local_kb = Vec::new();
        let neighbors = [
            (-1, 0, "north"),
            (1, 0, "south"),
            (0, 1, "east"),
            (0, -1, "west"),
        ];

        for (dy, dx, dir_name) in neighbors {
            if let Some((ny, nx)) = normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                let val = vm.grid[ny][nx].clone();
                let fact = Value::Junction(
                    JunctionType::Any,
                    vec![
                        Value::Str("neighbor".to_string()),
                        Value::Str(dir_name.to_string()),
                        val,
                    ],
                );
                local_kb.push(fact);
            } else {
                let fact = Value::Junction(
                    JunctionType::Any,
                    vec![
                        Value::Str("neighbor".to_string()),
                        Value::Str(dir_name.to_string()),
                        Value::Str("void".to_string()),
                    ],
                );
                local_kb.push(fact);
            }
        }

        // Add energy/self facts
        local_kb.push(Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("self".to_string()),
                Value::Int(cx as i64),
                Value::Int(cy as i64),
            ],
        ));

        // 3. Solve for Action
        // We want to find an Action X such that `effective_goal` is true given `do(X)`.
        // Or simpler: The user provides a rule `goal :- condition, move(Dir).`
        // We query the goal. If it unifies and provides a `move` or `action` binding?
        //
        // Let's adopt the Savant approach: Query `action(?A)` given `goal(G)`.
        // We construct a query: `action(?A)` and assert `goal(effective_goal)`.

        local_kb.push(Value::Junction(
            JunctionType::Any,
            vec![Value::Str("goal".to_string()), effective_goal.clone()],
        ));

        let query = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("action".to_string()),
                Value::Str("?A".to_string()),
            ],
        );

        // Merge global KB and local KB
        let mut full_kb = vm.knowledge_base.clone();
        full_kb.extend(local_kb);

        let mut solutions = Vec::new();
        oracle::solve(
            &[query],
            HashMap::new(),
            &full_kb,
            vm, // Immutable access to VM context
            &mut solutions,
            0,
        );

        // 4. Interpret Result
        let mut move_target = None;

        if let Some(sol) = solutions.first() {
            if let Some(action) = sol.get("?A") {
                // Execute Action
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
                                        if let Some((ny, nx)) = normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                                            if matches!(vm.grid[ny][nx], Value::Int(0)) {
                                                move_target = Some((ny, nx));
                                                // vm.output.push(format!("PHILOSOPHER: Moving {}", dir));
                                            }
                                        }
                                    }
                                }
                            }
                            "write" => {
                                // write(Dir, Val)
                                if args.len() >= 3 {
                                    if let Value::Str(dir) = &args[1] {
                                        let val = &args[2];
                                        let (dy, dx) = match dir.as_str() {
                                            "north" => (-1, 0),
                                            "south" => (1, 0),
                                            "east" => (0, 1),
                                            "west" => (0, -1),
                                            _ => (0, 0),
                                        };
                                        if let Some((ny, nx)) = normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                                            vm.grid[ny][nx] = val.clone();
                                            vm.output.push(format!("PHILOSOPHER: Wrote {:?}", val));
                                        }
                                    }
                                }
                            }
                            "ponder" => {
                                // Update internal state/goal
                                if args.len() >= 2 {
                                    let new_goal = args[1].clone();
                                    updated_agent.state = new_goal;
                                    vm.output.push("PHILOSOPHER: Paradigm Shift".to_string());
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Some((updated_agent, move_target))
    }
}
