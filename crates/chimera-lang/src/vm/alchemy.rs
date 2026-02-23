#[cfg(feature = "nova")]
use crate::ast::{JunctionType, Nucleotide, Strand};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};

// Crucible Ingredients (Capitalized)
const ING_FIRE: &str = "Fire";
const ING_WATER: &str = "Water";
const ING_VOID: &str = "Void";
const ING_LIFE: &str = "Life";

// Alchemy Grid Ingredients (Lowercase)
const ELEM_FIRE: &str = "fire";
const ELEM_WATER: &str = "water";
const ELEM_EARTH: &str = "earth";
const ELEM_AIR: &str = "air";
const ELEM_LIFE: &str = "life";
const ELEM_DEATH: &str = "death";
const ELEM_LEAD: &str = "lead";
const ELEM_ENERGY: &str = "energy";

// Alchemy Grid Results (Lowercase)
const RES_STEAM: &str = "steam";
const RES_LAVA: &str = "lava";
const RES_CLOUD: &str = "cloud";
const RES_SPIRIT: &str = "spirit";
const RES_GOLD: &str = "gold";

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Default)]
pub struct Crucible {
    pub contents: Vec<Value>,
}

#[cfg(feature = "nova")]
impl Crucible {
    pub fn new() -> Self {
        Self {
            contents: Vec::new(),
        }
    }

    pub fn add(&mut self, val: Value) {
        self.contents.push(val);
    }

    pub fn clear(&mut self) {
        self.contents.clear();
    }
}

#[cfg(feature = "nova")]
pub fn exec_crucible_op(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(mode) = val {
            match mode {
                0 => {
                    // Add
                    if let Some(item) = vm.stack.pop() {
                        vm.crucible.add(item.clone());
                        vm.output.push(format!("CRUCIBLE: Added {}", item));
                    } else {
                        vm.output.push("CRUCIBLE: Stack empty".to_string());
                    }
                }
                1 => {
                    // Clear
                    vm.crucible.clear();
                    vm.output.push("CRUCIBLE: Cleared".to_string());
                }
                2 => {
                    // Transmute
                    transmute_crucible(vm);
                }
                3 => {
                    // Withdraw
                    if let Some(item) = vm.crucible.contents.pop() {
                        vm.stack.push(item.clone());
                        vm.output.push(format!("CRUCIBLE: Withdrew {}", item));
                    } else {
                        vm.output.push("CRUCIBLE: Empty".to_string());
                    }
                }
                _ => {
                    vm.output.push("CRUCIBLE: Invalid mode".to_string());
                }
            }
        } else {
            vm.output.push("CRUCIBLE: Mode must be Int".to_string());
        }
    } else {
        vm.output.push("CRUCIBLE: Stack underflow".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn transmute_crucible(vm: &mut ChimeraVM) {
    // Take contents out to avoid double borrow
    let mut ingredients: Vec<Value> = std::mem::take(&mut vm.crucible.contents);

    // Sort for consistent matching
    ingredients.sort_by_key(|v| format!("{}", v));

    let (result, cost) = match ingredients.len() {
        2 => handle_two_ingredients(vm, &ingredients),
        3 => handle_three_ingredients(vm, &ingredients),
        _ => (None, 0),
    };

    if let Some(res) = result {
        vm.crucible.contents.push(res);
        vm.energy = vm.energy.saturating_sub(cost);
        vm.output
            .push("ALCHEMY: Transmutation successful!".to_string());
    } else {
        // Restore ingredients if failed
        vm.crucible.contents = ingredients;
        vm.output
            .push("ALCHEMY: Fizzle. Nothing happened.".to_string());
    }
}

#[cfg(feature = "nova")]
fn handle_two_ingredients(vm: &mut ChimeraVM, ingredients: &[Value]) -> (Option<Value>, i64) {
    // Try Oracle first
    #[cfg(feature = "oracle")]
    {
        if let Some((res, cost)) = try_oracle_alchemy(vm, ingredients) {
            return (Some(res), cost);
        }
    }

    // Try Genetic/Splice
    try_genetic_alchemy(vm, ingredients)
}

#[cfg(feature = "nova")]
#[cfg(feature = "oracle")]
fn try_oracle_alchemy(vm: &mut ChimeraVM, ingredients: &[Value]) -> Option<(Value, i64)> {
    let i1 = ingredients[0].clone();
    let i2 = ingredients[1].clone();
    let goal = Value::Junction(
        JunctionType::Any,
        vec![
            Value::Str("reaction".to_string()),
            i1,
            i2,
            Value::Str("?Result".to_string()),
        ],
    );

    let mut solutions = Vec::new();
    crate::vm::oracle::solve(
        &[goal],
        std::collections::HashMap::new(),
        &vm.knowledge_base,
        vm,
        &mut solutions,
        0,
    );

    if let Some(sol) = solutions.first() {
        let res_var = Value::Str("?Result".to_string());
        let resolved = crate::vm::oracle::resolve(&res_var, sol);
        // Ensure we got a concrete value, not a variable
        if !matches!(resolved, Value::Str(ref s) if s.starts_with('?')) {
            return Some((resolved.clone(), 5));
        }
    }
    None
}

#[cfg(feature = "nova")]
fn try_genetic_alchemy(vm: &mut ChimeraVM, ingredients: &[Value]) -> (Option<Value>, i64) {
    let (idx_a, modifier, idx_b) = parse_two_ingredients(ingredients);

    if let Some(idx) = idx_a {
        if idx >= vm.dna.helix.strands.len() {
            return (None, 0);
        }

        if let Some(mod_str) = modifier {
            return apply_genetic_modifier(vm, idx, mod_str);
        } else if let Some(idx_b) = idx_b {
            if idx_b < vm.dna.helix.strands.len() {
                return apply_genetic_splice(vm, idx, idx_b);
            }
        }
    }
    (None, 0)
}

#[cfg(feature = "nova")]
fn parse_two_ingredients(ingredients: &[Value]) -> (Option<usize>, Option<&str>, Option<usize>) {
    match (&ingredients[0], &ingredients[1]) {
        (Value::Int(a), Value::Str(s)) => (Some(*a as usize), Some(s.as_str()), None),
        (Value::Str(s), Value::Int(a)) => (Some(*a as usize), Some(s.as_str()), None),
        (Value::Int(a), Value::Int(b)) => (Some(*a as usize), None, Some(*b as usize)),
        _ => (None, None, None),
    }
}

#[cfg(feature = "nova")]
fn apply_genetic_modifier(vm: &mut ChimeraVM, idx: usize, mod_str: &str) -> (Option<Value>, i64) {
    let mut new_genes = vm.dna.helix.strands[idx].genes.clone();
    let mut modified = false;

    match mod_str {
        ING_FIRE => {
            // Heat: Increase numeric args
            for gene in &mut new_genes {
                for arg in &mut gene.args {
                    if let Nucleotide::Number(ref mut n) = arg {
                        *n = n.saturating_add(1);
                        modified = true;
                    }
                }
            }
            if modified {
                vm.output
                    .push(format!("ALCHEMY: Heated Strand {} (Fire)", idx));
            }
        }
        ING_WATER => {
            // Cold: Decrease numeric args
            for gene in &mut new_genes {
                for arg in &mut gene.args {
                    if let Nucleotide::Number(ref mut n) = arg {
                        *n = n.saturating_sub(1);
                        modified = true;
                    }
                }
            }
            if modified {
                vm.output
                    .push(format!("ALCHEMY: Cooled Strand {} (Water)", idx));
            }
        }
        ING_VOID => {
            // Corruption: Random Nop
            let mut rng = rand::thread_rng();
            use rand::Rng;
            for gene in &mut new_genes {
                if rng.gen_bool(0.1) {
                    gene.op = OpCode::Nop;
                    gene.args.clear();
                    modified = true;
                }
            }
            if modified {
                vm.output
                    .push(format!("ALCHEMY: Corrupted Strand {} (Void)", idx));
            }
        }
        ING_LIFE => {
            // Growth: Duplicate random genes
            let mut rng = rand::thread_rng();
            use rand::Rng;
            let mut grown_genes: Vec<crate::ast::Gene> = Vec::new();
            for gene in new_genes {
                grown_genes.push(gene.clone());
                if rng.gen_bool(0.1) {
                    grown_genes.push(gene);
                    modified = true;
                }
            }
            new_genes = grown_genes;
            if modified {
                vm.output
                    .push(format!("ALCHEMY: Grew Strand {} (Life)", idx));
            }
        }
        _ => {}
    }

    if modified {
        let new_idx =
            register_new_strand(vm, new_genes, Some(idx), format!("Alchemy: {}", mod_str));
        return (Some(Value::Int(new_idx as i64)), 25);
    }

    (None, 0)
}

#[cfg(feature = "nova")]
fn apply_genetic_splice(vm: &mut ChimeraVM, idx_a: usize, idx_b: usize) -> (Option<Value>, i64) {
    let genes_a = &vm.dna.helix.strands[idx_a].genes;
    let genes_b = &vm.dna.helix.strands[idx_b].genes;
    let mut new_genes = Vec::new();
    let max_len = genes_a.len().max(genes_b.len());
    for i in 0..max_len {
        if i < genes_a.len() {
            new_genes.push(genes_a[i].clone());
        }
        if i < genes_b.len() {
            new_genes.push(genes_b[i].clone());
        }
    }

    let new_idx = register_new_strand(vm, new_genes, Some(idx_a), "Alchemy: Splice".to_string());

    vm.output.push(format!(
        "ALCHEMY: Spliced Strand {} & {} -> {}",
        idx_a, idx_b, new_idx
    ));

    (Some(Value::Int(new_idx as i64)), 30)
}

#[cfg(feature = "nova")]
fn handle_three_ingredients(vm: &mut ChimeraVM, ingredients: &[Value]) -> (Option<Value>, i64) {
    // Fusion: [Strand A, Strand B, "Life"]
    let mut strand_a = None;
    let mut strand_b = None;
    let mut has_life = false;

    for v in ingredients {
        match v {
            Value::Int(i) => {
                let idx = *i as usize;
                if strand_a.is_none() {
                    strand_a = Some(idx);
                } else if strand_b.is_none() {
                    strand_b = Some(idx);
                }
            }
            Value::Str(s) if s == ING_LIFE => has_life = true,
            _ => {}
        }
    }

    if has_life {
        if let (Some(idx_a), Some(idx_b)) = (strand_a, strand_b) {
            if idx_a < vm.dna.helix.strands.len() && idx_b < vm.dna.helix.strands.len() {
                // Fusion: Append B to A
                let mut new_genes = vm.dna.helix.strands[idx_a].genes.clone();
                new_genes.extend(vm.dna.helix.strands[idx_b].genes.clone());

                let new_idx =
                    register_new_strand(vm, new_genes, Some(idx_a), "Alchemy: Fusion".to_string());

                vm.output.push(format!(
                    "ALCHEMY: Fused Strand {} & {} -> {}",
                    idx_a, idx_b, new_idx
                ));

                return (Some(Value::Int(new_idx as i64)), 40);
            }
        }
    }
    (None, 0)
}

#[cfg(feature = "nova")]
fn register_new_strand(
    vm: &mut ChimeraVM,
    genes: Vec<crate::ast::Gene>,
    parent: Option<usize>,
    label: String,
) -> usize {
    vm.dna.helix.strands.push(Strand { genes });
    vm.telomeres.push(50);
    #[cfg(feature = "cortex")]
    {
        vm.activation_levels.push(0);
        vm.synapse_map.push(Vec::new());
    }
    let new_idx = vm.dna.helix.strands.len() - 1;

    vm.cladistics
        .register_strand(new_idx, parent, vm.tick_counter, label);
    new_idx
}

#[cfg(feature = "nova")]
pub fn perform_alchemy(vm: &mut ChimeraVM, y: usize, x: usize) -> bool {
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut ingredients = Vec::new();
    let mut coords = Vec::new();

    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
            ingredients.push(vm.grid[ny][nx].clone());
            coords.push((ny, nx));
        }
    }

    let center_val = vm.grid[y][x].clone();

    if let Some(res_str) = check_transmutation_recipe(&ingredients, &center_val) {
        vm.grid[y][x] = Value::Str(res_str);
        // Consume ingredients (set to 0/void)
        for (ny, nx) in coords {
            vm.grid[ny][nx] = Value::Int(0);
        }
        vm.output
            .push(format!("ALCHEMY: Transmutation occurred at {},{}", x, y));
        return true;
    }

    false
}

#[cfg(feature = "nova")]
fn check_transmutation_recipe(ingredients: &[Value], center: &Value) -> Option<String> {
    if has_ingredient(ingredients, ELEM_FIRE) && has_ingredient(ingredients, ELEM_WATER) {
        return Some(RES_STEAM.to_string());
    }
    if has_ingredient(ingredients, ELEM_EARTH) && has_ingredient(ingredients, ELEM_FIRE) {
        return Some(RES_LAVA.to_string());
    }
    if has_ingredient(ingredients, ELEM_AIR) && has_ingredient(ingredients, ELEM_WATER) {
        return Some(RES_CLOUD.to_string());
    }
    if has_ingredient(ingredients, ELEM_LIFE) && has_ingredient(ingredients, ELEM_DEATH) {
        return Some(RES_SPIRIT.to_string());
    }

    if let Value::Str(c) = center {
        if c == ELEM_LEAD && has_ingredient(ingredients, ELEM_ENERGY) {
            return Some(RES_GOLD.to_string());
        }
    }

    None
}

#[cfg(feature = "nova")]
fn has_ingredient(ingredients: &[Value], s: &str) -> bool {
    ingredients
        .iter()
        .any(|v| matches!(v, Value::Str(val) if val == s))
}
