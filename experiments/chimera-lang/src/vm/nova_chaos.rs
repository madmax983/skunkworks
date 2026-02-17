#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value, GRID_SIZE};
#[cfg(feature = "nova")]
use rand::Rng;
#[cfg(feature = "nova")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "nova")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlchemyRecipe {
    pub inputs: Vec<String>,
    pub output: String,
    pub probability: f64,
}

#[cfg(feature = "nova")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChaosCartridge {
    pub recipes: Vec<AlchemyRecipe>,
    pub active: bool,
}

#[cfg(feature = "nova")]
impl ChaosCartridge {
    pub fn new() -> Self {
        Self {
            recipes: Vec::new(),
            active: true,
        }
    }

    pub fn add_recipe(&mut self, inputs: Vec<String>, output: String, probability: f64) {
        self.recipes.push(AlchemyRecipe {
            inputs,
            output,
            probability,
        });
    }

    pub fn scramble(&mut self) {
        let mut rng = rand::thread_rng();
        // Clear existing or mutate? Let's add some random nonsense.
        let elements = ["Fire", "Water", "Earth", "Air", "Void", "Life", "Death", "Chaos"];
        let inputs: Vec<String> = (0..rng.gen_range(2..4))
            .map(|_| elements[rng.gen_range(0..elements.len())].to_string())
            .collect();
        let output = elements[rng.gen_range(0..elements.len())].to_string();

        self.add_recipe(inputs, output, rng.gen_range(0.1..1.0));
    }
}

#[cfg(feature = "nova")]
pub fn exec_chaos_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::ChaosDefine => {
            // Stack: [ ..., output, input1, input2, ... ] -> we need a way to delineate inputs.
            // Let's assume binary recipes for simplicity for now, or use a Junction.
            // Or simpler: Stack: [ ..., input_junction, output_str ]

            if vm.stack.len() >= 2 {
                let out_val = vm.stack.pop().unwrap();
                let in_val = vm.stack.pop().unwrap();

                if let (Value::Str(output), Value::Junction(_, inputs)) = (out_val, in_val) {
                    let mut input_strs = Vec::new();
                    for v in inputs {
                        if let Value::Str(s) = v {
                            input_strs.push(s);
                        }
                    }
                    if !input_strs.is_empty() {
                        vm.chaos_cartridge.add_recipe(input_strs, output.clone(), 1.0);
                        vm.output.push(format!("CHAOS: Defined recipe -> {}", output));
                    }
                } else {
                    vm.output.push("Error: ChaosDefine requires [input_junction, output_str]".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for ChaosDefine".to_string());
            }
        }
        OpCode::ChaosScramble => {
            vm.chaos_cartridge.scramble();
            vm.output.push("CHAOS: Recipes scrambled!".to_string());
        }
        OpCode::ChaosInvoke => {
            process_chaos_physics(vm);
            vm.output.push("CHAOS: Physics invoked manually.".to_string());
        }
        _ => {}
    }
    None
}

#[cfg(feature = "nova")]
pub fn process_chaos_physics(vm: &mut ChimeraVM) {
    if !vm.chaos_cartridge.active {
        return;
    }

    let size = GRID_SIZE;
    let mut transmutations = Vec::new();

    // Iterate over grid to find recipe matches
    // This is expensive (O(N*M*R)). N=256, M=4 (neighbors), R=Recipes.
    // Optimization: Only check cells that have neighbors.

    for y in 0..size {
        for x in 0..size {
            // Center cell is the reaction site?
            // Or center is empty and neighbors react?
            // Let's say: If a cell contains "Chaos" or we just scan every cell context.

            // Collect local context (Von Neumann)
            let neighbors = [
                (-1, 0), (1, 0), (0, -1), (0, 1)
            ];

            let mut context_vals = Vec::new();
            let mut context_coords = Vec::new();

            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Value::Str(s) = &vm.grid[ny][nx] {
                        context_vals.push(s.clone());
                        context_coords.push((ny, nx));
                    }
                }
            }

            if context_vals.is_empty() {
                continue;
            }

            // Check recipes
            for recipe in &vm.chaos_cartridge.recipes {
                // Check if context contains all inputs
                // Simple multiset inclusion check?
                // Or exact match? Let's do loose matching: if neighbors contain all inputs.

                let mut remaining_inputs = recipe.inputs.clone();
                let mut used_indices = Vec::new();

                for (i, val) in context_vals.iter().enumerate() {
                    if let Some(pos) = remaining_inputs.iter().position(|r| r == val) {
                        remaining_inputs.remove(pos);
                        used_indices.push(i);
                    }
                }

                if remaining_inputs.is_empty() {
                    // Match!
                    let mut rng = rand::thread_rng();
                    if rng.gen_bool(recipe.probability) {
                        let consumed_coords: Vec<(usize, usize)> = used_indices.iter().map(|&i| context_coords[i]).collect();
                        transmutations.push((y, x, recipe.output.clone(), consumed_coords));
                        break; // One reaction per cell per tick
                    }
                }
            }
        }
    }

    // Apply transmutations
    for (y, x, output, consumed) in transmutations {
        vm.grid[y][x] = Value::Str(output);
        for (cy, cx) in consumed {
            vm.grid[cy][cx] = Value::Int(0); // Consume ingredients
        }
    }
}
