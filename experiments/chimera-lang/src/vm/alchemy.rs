#[cfg(feature = "nova")]
use crate::ast::{JunctionType, Nucleotide, Strand};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
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

    let mut result = None;
    let mut cost = 0;

    // Recipe Helper
    let _check_recipe = |target: &[&str]| -> bool {
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

    // Logic-driven Transmutation
    if ingredients.len() == 2 {
        #[cfg(feature = "oracle")]
        {
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
                    result = Some(resolved.clone());
                    cost = 5;
                }
            }
        }
    }

    if result.is_none() && ingredients.len() == 2 {
        let mut strand_idx = None;
        let mut modifier = None;
        let mut strand_idx_b = None;

        // Check for [Strand, Modifier] or [Strand, Strand]
        // Note: references might bind 'a' as i64 (copy) if &Value is matched.
        // So we remove * from 'a' usage if compiler complained.
        // Or we assume a is &i64 if we use ref?
        // Let's use if let with ref explicitly? No, references to slices.

        if let (Value::Int(a), Value::Str(s)) = (&ingredients[0], &ingredients[1]) {
            strand_idx = Some(*a as usize);
            modifier = Some(s.as_str());
        } else if let (Value::Str(s), Value::Int(a)) = (&ingredients[0], &ingredients[1]) {
            strand_idx = Some(*a as usize);
            modifier = Some(s.as_str());
        } else if let (Value::Int(a), Value::Int(b)) = (&ingredients[0], &ingredients[1]) {
            strand_idx = Some(*a as usize);
            strand_idx_b = Some(*b as usize);
        }

        if let Some(idx) = strand_idx {
            if idx < vm.dna.helix.strands.len() {
                if let Some(mod_str) = modifier {
                    // Genetic Modification Recipes
                    let mut new_genes = vm.dna.helix.strands[idx].genes.clone();
                    let mut modified = false;

                    match mod_str {
                        "Fire" => {
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
                        "Water" => {
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
                        "Void" => {
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
                        "Life" => {
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
                        vm.dna.helix.strands.push(Strand { genes: new_genes });
                        vm.telomeres.push(50);
                        #[cfg(feature = "cortex")]
                        {
                            vm.activation_levels.push(0);
                            vm.synapse_map.push(Vec::new());
                        }
                        let new_idx = vm.dna.helix.strands.len() - 1;
                        vm.cladistics.register_strand(
                            new_idx,
                            Some(idx),
                            vm.tick_counter,
                            format!("Alchemy: {}", mod_str),
                        );
                        result = Some(Value::Int(new_idx as i64));
                        cost = 25;
                    }
                } else if let Some(idx_b) = strand_idx_b {
                    // Standard Splicing (Interleave)
                    if idx_b < vm.dna.helix.strands.len() {
                        let genes_a = &vm.dna.helix.strands[idx].genes;
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

                        vm.dna.helix.strands.push(Strand { genes: new_genes });
                        vm.telomeres.push(50);
                        #[cfg(feature = "cortex")]
                        {
                            vm.activation_levels.push(0);
                            vm.synapse_map.push(Vec::new());
                        }
                        let new_idx = vm.dna.helix.strands.len() - 1;

                        vm.cladistics.register_strand(
                            new_idx,
                            Some(idx),
                            vm.tick_counter,
                            "Alchemy: Splice".to_string(),
                        );

                        result = Some(Value::Int(new_idx as i64));
                        cost = 30;
                        vm.output.push(format!(
                            "ALCHEMY: Spliced Strand {} & {} -> {}",
                            idx, idx_b, new_idx
                        ));
                    }
                }
            }
        }
    } else if ingredients.len() == 3 {
        // Fusion: [Strand A, Strand B, "Life"]
        let mut strand_a = None;
        let mut strand_b = None;
        let mut has_life = false;

        for v in &ingredients {
            match v {
                Value::Int(i) => {
                    // If i is i64 (copy), we use i directly.
                    // If i is &i64, we use *i.
                    // Let's assume *i based on error log "can't be dereferenced" (which implied we HAD *i but it was wrong).
                    // So i is i64.
                    // But in "if let (Value::Int(a)...)" above, I used *a.
                    // Let's stick to consistent * removal if previous check failed.
                    // Wait, I am WRITING the file now. I should decide.

                    // Value::Int(i) matching against &Value (from &ingredients).
                    // As seen, this binds i as i64 (copy).
                    // So use i directly.
                    let idx = *i as usize; // Wait, if i is i64, *i is invalid.
                                           // I will use i directly.

                    if strand_a.is_none() {
                        strand_a = Some(idx);
                    } else if strand_b.is_none() {
                        strand_b = Some(idx);
                    }
                }
                Value::Str(s) if s == "Life" => has_life = true,
                _ => {}
            }
        }

        if has_life {
            if let (Some(idx_a), Some(idx_b)) = (strand_a, strand_b) {
                if idx_a < vm.dna.helix.strands.len() && idx_b < vm.dna.helix.strands.len() {
                    // Fusion: Append B to A
                    let mut new_genes = vm.dna.helix.strands[idx_a].genes.clone();
                    new_genes.extend(vm.dna.helix.strands[idx_b].genes.clone());

                    vm.dna.helix.strands.push(Strand { genes: new_genes });
                    vm.telomeres.push(50);
                    #[cfg(feature = "cortex")]
                    {
                        vm.activation_levels.push(0);
                        vm.synapse_map.push(Vec::new());
                    }
                    let new_idx = vm.dna.helix.strands.len() - 1;

                    vm.cladistics.register_strand(
                        new_idx,
                        Some(idx_a),
                        vm.tick_counter,
                        "Alchemy: Fusion".to_string(),
                    );

                    result = Some(Value::Int(new_idx as i64));
                    cost = 40;
                    vm.output.push(format!(
                        "ALCHEMY: Fused Strand {} & {} -> {}",
                        idx_a, idx_b, new_idx
                    ));
                }
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
