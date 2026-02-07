#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};

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
pub fn transmute_crucible(vm: &mut ChimeraVM) {
    // Take contents out to avoid double borrow
    let mut ingredients = std::mem::take(&mut vm.crucible.contents);

    // Sort for consistent matching
    ingredients.sort_by_key(|v| format!("{}", v));

    let mut result = None;
    let mut cost = 0;

    // Recipe Helper
    let check_recipe = |target: &[&str]| -> bool {
        if ingredients.len() != target.len() {
            return false;
        }
        let mut target_sorted = target.to_vec();
        target_sorted.sort();

        for (i, v) in ingredients.iter().enumerate() {
            let s = match v {
                Value::Str(s) => s.as_str(),
                _ => return false,
            };
            if s != target_sorted[i] {
                return false;
            }
        }
        true
    };

    if check_recipe(&["Fire", "Water"]) {
        result = Some(Value::Str("Steam".to_string()));
        cost = 5;
    } else if check_recipe(&["Earth", "Fire"]) {
        result = Some(Value::Str("Lava".to_string()));
        cost = 5;
    } else if check_recipe(&["Air", "Water"]) {
        result = Some(Value::Str("Cloud".to_string()));
        cost = 5;
    } else if check_recipe(&["Life", "Death"]) {
        result = Some(Value::Str("Spirit".to_string()));
        cost = 20;
    } else if check_recipe(&["Energy", "Lead"]) {
        result = Some(Value::Str("Gold".to_string()));
        cost = 50;
    } else if ingredients.len() == 2 {
        // Genetic Splicing Recipe: Two Strand Indices
        if let (Value::Int(a), Value::Int(b)) = (&ingredients[0], &ingredients[1]) {
            let idx_a = *a as usize;
            let idx_b = *b as usize;
            if idx_a < vm.dna.helix.strands.len() && idx_b < vm.dna.helix.strands.len() {
                // Splice logic (Interleave)
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

                vm.dna
                    .helix
                    .strands
                    .push(crate::ast::Strand { genes: new_genes });
                vm.telomeres.push(50);
                #[cfg(feature = "cortex")]
                {
                    vm.activation_levels.push(0);
                    vm.synapse_map.push(Vec::new());
                }
                let new_idx = vm.dna.helix.strands.len() - 1;

                vm.cladistics.register_strand(
                    new_idx,
                    Some(idx_a), // Primary parent
                    vm.tick_counter,
                    "Alchemy".to_string(),
                );

                result = Some(Value::Int(new_idx as i64));
                cost = 30;
                vm.output.push(format!(
                    "ALCHEMY: Spliced Strand {} & {} -> {}",
                    idx_a, idx_b, new_idx
                ));
            }
        }
    }

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
pub fn perform_alchemy(vm: &mut ChimeraVM, y: usize, x: usize) -> bool {
    // Recipes:
    // "fire" + "water" -> "steam"
    // "earth" + "fire" -> "lava"
    // "air" + "water" -> "cloud"
    // "life" + "death" -> "spirit"
    // "lead" + "energy" (center=lead) -> "gold"

    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut ingredients = Vec::new();
    let mut coords = Vec::new();

    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
            ingredients.push(vm.grid[ny][nx].clone());
            coords.push((ny, nx));
        }
    }

    let has_ingredient = |s: &str| -> bool {
        ingredients
            .iter()
            .any(|v| matches!(v, Value::Str(val) if val == s))
    };

    let center_val = vm.grid[y][x].clone();
    let mut transmuted = false;
    let mut result = Value::Int(0);

    if has_ingredient("fire") && has_ingredient("water") {
        result = Value::Str("steam".to_string());
        transmuted = true;
    } else if has_ingredient("earth") && has_ingredient("fire") {
        result = Value::Str("lava".to_string());
        transmuted = true;
    } else if has_ingredient("air") && has_ingredient("water") {
        result = Value::Str("cloud".to_string());
        transmuted = true;
    } else if has_ingredient("life") && has_ingredient("death") {
        result = Value::Str("spirit".to_string());
        transmuted = true;
    } else if let Value::Str(c) = center_val {
        if c == "lead" && has_ingredient("energy") {
            result = Value::Str("gold".to_string());
            transmuted = true;
        }
    }

    if transmuted {
        vm.grid[y][x] = result;
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
