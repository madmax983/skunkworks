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
        let elements = [
            "Fire", "Water", "Earth", "Air", "Void", "Life", "Death", "Chaos",
        ];
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
                        vm.chaos_cartridge
                            .add_recipe(input_strs, output.clone(), 1.0);
                        vm.output
                            .push(format!("CHAOS: Defined recipe -> {}", output));
                    }
                } else {
                    vm.output.push(
                        "Error: ChaosDefine requires [input_junction, output_str]".to_string(),
                    );
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for ChaosDefine".to_string());
            }
        }
        OpCode::ChaosScramble => {
            vm.chaos_cartridge.scramble();
            vm.output.push("CHAOS: Recipes scrambled!".to_string());
        }
        OpCode::ChaosInvoke => {
            process_chaos_physics(vm);
            vm.output
                .push("CHAOS: Physics invoked manually.".to_string());
        }
        OpCode::ChaosLearn => {
            // Learn a recipe from a DNA strand
            // Stack: [ ..., strand_idx ]
            if let Some(Value::Int(idx)) = vm.stack.pop() {
                if let Some(recipe) = parse_strand_to_recipe(vm, idx as usize) {
                    vm.chaos_cartridge.recipes.push(recipe.clone());
                    vm.output
                        .push(format!("CHAOS: Learned recipe -> {}", recipe.output));
                } else {
                    vm.output
                        .push("CHAOS: Failed to learn recipe from strand.".to_string());
                }
            }
        }
        OpCode::Mercury => {
            let (cy, cx) = vm.context_loc;
            if let Some((output, consumed)) = check_local_transmutation(vm, cy, cx) {
                vm.grid[cy][cx] = Value::Str(output);
                for (y, x) in consumed {
                    vm.grid[y][x] = Value::Int(0);
                }
                vm.output
                    .push("MERCURY: Transmutation successful.".to_string());
            }
        }
        OpCode::Venus => {
            let (cy, cx) = vm.context_loc;
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut count = 0;
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                    if let Some((output, consumed)) = check_local_transmutation(vm, ny, nx) {
                        vm.grid[ny][nx] = Value::Str(output);
                        for (y, x) in consumed {
                            vm.grid[y][x] = Value::Int(0);
                        }
                        count += 1;
                    }
                }
            }
            if count > 0 {
                vm.output
                    .push(format!("VENUS: Transmuted {} neighbors.", count));
            }
        }
        OpCode::Salt => {
            vm.stack.push(Value::Str("Salt".to_string()));
        }
        OpCode::Sulfur => {
            vm.stack.push(Value::Str("Sulfur".to_string()));
        }
        _ => {}
    }
    None
}

#[cfg(feature = "nova")]
fn parse_strand_to_recipe(vm: &ChimeraVM, strand_idx: usize) -> Option<AlchemyRecipe> {
    if strand_idx >= vm.dna.helix.strands.len() {
        return None;
    }
    let strand = &vm.dna.helix.strands[strand_idx];

    // Format: [Input1, Input2, ..., Output]
    // We treat genes as string inputs based on their OpCode name or arg
    // Actually, simpler: Use Push(String) genes.

    let mut tokens = Vec::new();
    for gene in &strand.genes {
        if let OpCode::Push = gene.op {
            if let Some(arg) = gene.args.first() {
                match arg {
                    Nucleotide::String(s) => tokens.push(s.clone()),
                    Nucleotide::Number(n) => tokens.push(n.to_string()),
                    _ => {}
                }
            }
        }
    }

    if tokens.len() < 2 {
        return None;
    }

    let output = tokens.pop().unwrap();
    let inputs = tokens;

    Some(AlchemyRecipe {
        inputs,
        output,
        probability: 1.0,
    })
}

#[cfg(feature = "nova")]
pub fn process_chaos_physics(vm: &mut ChimeraVM) {
    if !vm.chaos_cartridge.active {
        return;
    }

    let size = GRID_SIZE;
    let mut transmutations = Vec::new();

    for y in 0..size {
        for x in 0..size {
            if let Some((output, consumed)) = check_local_transmutation(vm, y, x) {
                transmutations.push((y, x, output, consumed));
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

/// Checks if a transmutation can occur at the given coordinates.
/// Returns Some((OutputString, Vec<ConsumedCoords>)) if successful.
#[cfg(feature = "nova")]
pub fn check_local_transmutation(
    vm: &ChimeraVM,
    y: usize,
    x: usize,
) -> Option<(String, Vec<(usize, usize)>)> {
    if !vm.chaos_cartridge.active {
        return None;
    }

    // Gather Context (Neighbors)
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut context_vals = Vec::new();
    let mut context_coords = Vec::new();
    let mut has_salt = false;
    let mut has_sulfur = false;

    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
            let val = &vm.grid[ny][nx];
            match val {
                Value::Str(s) => {
                    // Check for Reagents/Catalysts
                    if s.eq_ignore_ascii_case("salt") || s == "$" {
                        has_salt = true;
                    } else if s.eq_ignore_ascii_case("sulfur") || s == "&" {
                        has_sulfur = true;
                    } else {
                        context_vals.push(s.clone());
                        context_coords.push((ny, nx));
                    }
                }
                Value::Int(n) => {
                    // Treat non-zero ints as strings? Or ignore?
                    // Let's ignore 0s.
                    if *n != 0 {
                        context_vals.push(n.to_string());
                        context_coords.push((ny, nx));
                    }
                }
                _ => {}
            }
        }
    }

    if context_vals.is_empty() {
        return None;
    }

    // Check Recipes
    for recipe in &vm.chaos_cartridge.recipes {
        let mut remaining_inputs = recipe.inputs.clone();
        let mut used_indices = Vec::new();

        // Greedy matching
        for (i, val) in context_vals.iter().enumerate() {
            if let Some(pos) = remaining_inputs.iter().position(|r| r == val) {
                remaining_inputs.remove(pos);
                used_indices.push(i);
            }
        }

        if remaining_inputs.is_empty() {
            // Match Found!
            let prob = if has_sulfur { 1.0 } else { recipe.probability };
            let mut rng = rand::thread_rng();

            if rng.gen_bool(prob) {
                let consumed = if has_salt {
                    Vec::new() // Preserve ingredients
                } else {
                    used_indices.iter().map(|&i| context_coords[i]).collect()
                };

                return Some((recipe.output.clone(), consumed));
            }
        }
    }

    None
}
