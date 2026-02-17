use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::vm::{Value, GRID_SIZE, ChimeraVM};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrologueState {
    pub active: bool,
    pub runes: HashSet<(usize, usize)>,
    pub rules: Vec<String>,
}

impl PrologueState {
    pub fn new() -> Self {
        Self {
            active: false,
            runes: HashSet::new(),
            rules: Vec::new(),
        }
    }

    pub fn scan_grid_rules(&mut self, grid: &Vec<Vec<Value>>) {
        self.runes.clear();
        self.rules.clear();

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if let Value::Str(s) = &grid[y][x] {
                    // Identify Runes
                    if matches!(s.as_str(), "?" | "!" | "~" | "&" | "|" | "@") {
                        self.runes.insert((y, x));

                        // Simple Logic: If '?' is found, add a rule "Query at (y,x)"
                        if s == "?" {
                            self.rules.push(format!("Query({},{})", y, x));
                        }
                    }
                }
            }
        }
    }
}

pub fn exec_prologue_tick(vm: &mut ChimeraVM) {
    if !vm.prologue_state.active {
        return;
    }

    // Clone grid to scan without borrowing vm mutably for the scan
    let grid = vm.grid.clone();
    vm.prologue_state.scan_grid_rules(&grid);

    // Process Runes
    // We iterate over detected runes
    let runes: Vec<(usize, usize)> = vm.prologue_state.runes.iter().cloned().collect();

    for (y, x) in runes {
        // Safe because we are just reading grid which is part of vm,
        // but we need to index it.
        // vm.grid is available.
        if let Value::Str(s) = &vm.grid[y][x] {
            match s.as_str() {
                "?" => {
                    // Query: Read neighbor (North)
                    if y > 0 {
                        let val = &vm.grid[y-1][x];
                        vm.output.push(format!("PROLOGUE: Query at {},{} found {:?}", x, y, val));
                        // In a real Prolog logic, this would trigger unification.
                    }
                }
                "!" => {
                    // Fact: Assert neighbor (North)
                    if y > 0 {
                         let val = &vm.grid[y-1][x];
                         vm.output.push(format!("PROLOGUE: Asserting fact at {},{}: {:?}", x, y, val));
                    }
                }
                "@" => {
                    // Agent: Moves towards '?'
                    // Simple logic: move right
                    if x + 1 < GRID_SIZE {
                        // Move logic would require mutating grid, which is allowed here
                        // But we must be careful not to overwrite other runes without logic
                        // For now, just log
                        vm.output.push(format!("PROLOGUE: Agent active at {},{}", x, y));
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::ChimeraVM;
    use crate::ast::Dna;

    use crate::ast::Helix;

    #[test]
    fn test_prologue_scan() {
        let dna = Dna {
            helix: Helix {
                strands: vec![],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        vm.grid[5][5] = Value::Str("?".to_string());
        vm.grid[4][5] = Value::Int(42);

        exec_prologue_tick(&mut vm);

        assert!(vm.prologue_state.runes.contains(&(5, 5)));
        assert_eq!(vm.prologue_state.rules.len(), 1);
        assert_eq!(vm.prologue_state.rules[0], "Query(5,5)");

        // Output should contain query log
        let output = vm.output.join("\n");
        assert!(output.contains("PROLOGUE: Query at 5,5 found Int(42)"));
    }
}
