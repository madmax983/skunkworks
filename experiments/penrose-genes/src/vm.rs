use crate::ast::{Dna, Nucleotide};
use crate::penrose::{PenroseTiling, Point};
use rand::Rng;
#[cfg(feature = "nova")]
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Str(String),
}

impl Default for Value {
    fn default() -> Self {
        Value::Int(0)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Str(s) => write!(f, "\"{}\"", s),
        }
    }
}

pub struct ChimeraVM {
    pub dna: Dna,
    pub stack: Vec<Value>,
    pub ip: (usize, usize), // (strand_idx, gene_idx)
    pub output: Vec<String>,
    pub halted: bool,
    pub energy: i64,
    pub tiling: PenroseTiling<Value>,
    pub chaos_mode: bool,
    #[cfg(feature = "nova")]
    pub epigenome: HashSet<(usize, usize)>,
    #[cfg(feature = "nova")]
    pub telomeres: Vec<i64>,
}

impl ChimeraVM {
    pub fn new(dna: Dna) -> Self {
        // Initialize Penrose Tiling
        let mut tiling = PenroseTiling::generate_sun(100.0);
        // Subdivide twice to get a reasonable number of tiles (~40-80)
        tiling.subdivide();
        tiling.subdivide();
        tiling.build_adjacency();

        #[cfg(feature = "nova")]
        let strand_count = dna.helix.strands.len();

        Self {
            dna,
            stack: Vec::new(),
            ip: (0, 0),
            output: Vec::new(),
            halted: false,
            energy: 50,
            tiling,
            chaos_mode: false,
            #[cfg(feature = "nova")]
            epigenome: HashSet::new(),
            #[cfg(feature = "nova")]
            telomeres: vec![50; strand_count],
        }
    }

    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        self.energy -= 1;

        if self.chaos_mode {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.1) {
                // 10% chance per step
                self.mutate();
            }
        }

        if self.energy <= 0 {
            self.halted = true;
            self.output.push("DEATH: STARVATION".to_string());
            return;
        }

        let helix_len = self.dna.helix.strands.len();
        if self.ip.0 >= helix_len {
            self.halted = true;
            return;
        }

        let strand_len = self.dna.helix.strands[self.ip.0].genes.len();
        if self.ip.1 >= strand_len {
            // End of strand, move to next strand
            self.ip.0 += 1;
            self.ip.1 = 0;
            return;
        }

        #[cfg(feature = "nova")]
        {
            // Telomere check at start of strand
            if self.ip.1 == 0 && self.ip.0 < self.telomeres.len() {
                if self.telomeres[self.ip.0] > 0 {
                    self.telomeres[self.ip.0] -= 1;
                }

                if self.telomeres[self.ip.0] <= 0 {
                    self.output.push(format!("SENESCENCE: Strand {} decayed", self.ip.0));
                    self.ip.0 += 1;
                    self.ip.1 = 0;
                    return;
                }
            }

            if self.epigenome.contains(&self.ip) {
                self.ip.1 += 1;
                return;
            }
        }

        // Clone gene info to release borrow on self.dna
        let (gene_name, gene_args) = {
            let gene = &self.dna.helix.strands[self.ip.0].genes[self.ip.1];
            (gene.name.clone(), gene.args.clone())
        };

        let jump_target = self.execute_gene(&gene_name, &gene_args);

        if let Some(target) = jump_target {
            self.ip = target;
        } else {
            // Move to next gene
            self.ip.1 += 1;
        }
    }

    fn execute_gene(&mut self, name: &str, args: &[Nucleotide]) -> Option<(usize, usize)> {
        match name {
            "push" => {
                if let Some(arg) = args.first() {
                    match arg {
                        Nucleotide::Number(n) => self.stack.push(Value::Int(*n)),
                        Nucleotide::String(s) => self.stack.push(Value::Str(s.clone())),
                        _ => self
                            .output
                            .push(format!("Error: Invalid arg for push: {:?}", arg)),
                    }
                }
                None
            }
            // "incubate" removed in this hybrid as it relies on rectangular grid iteration
            "add" => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a + b);
                None
            }
            "sub" => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a - b);
                None
            }
            "mul" => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a * b);
                None
            }
            "div" => {
                if self.stack.len() < 2 {
                    self.output.push("Error: Stack underflow".to_string());
                } else {
                    let b_val = self.stack.pop().unwrap();
                    let a_val = self.stack.pop().unwrap();
                    match (a_val, b_val) {
                        (Value::Int(a), Value::Int(b)) => {
                            if b == 0 {
                                self.output.push("Error: Division by zero".to_string());
                            } else if a == i64::MIN && b == -1 {
                                self.output.push("Error: Division overflow".to_string());
                            } else {
                                self.stack.push(Value::Int(a / b));
                            }
                        }
                        _ => self.output.push("Error: Type mismatch".to_string()),
                    }
                }
                None
            }
            "dup" => {
                if let Some(val) = self.stack.last() {
                    self.stack.push(val.clone());
                }
                None
            }
            "swap" => {
                let len = self.stack.len();
                if len >= 2 {
                    self.stack.swap(len - 1, len - 2);
                } else {
                    self.output
                        .push("Error: Stack underflow for swap".to_string());
                }
                None
            }
            "drop" => {
                self.stack.pop();
                None
            }
            "print" => {
                if let Some(val) = self.stack.pop() {
                    self.output.push(format!("{}", val));
                }
                None
            }
            "jump" => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    Some((*n as usize, 0))
                } else {
                    self.output.push("Error: Invalid arg for jump".to_string());
                    None
                }
            }
            "brz" => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    if let Some(val) = self.stack.pop() {
                        if let Value::Int(i) = val {
                            if i == 0 {
                                return Some((*n as usize, 0));
                            }
                        } else {
                            self.output.push("Error: Type mismatch for brz".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Stack underflow for brz".to_string());
                    }
                } else {
                    self.output.push("Error: Invalid arg for brz".to_string());
                }
                None
            }
            "photosynthesize" => {
                self.energy += 5;
                None
            }
            "consume" => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(n) => self.energy += n,
                        Value::Str(s) => self.energy += s.len() as i64,
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for consume".to_string());
                }
                None
            }
            "g_read" => {
                if self.stack.len() >= 2 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                        // Map x,y to Point
                        let p = Point::new(x as f64, y as f64);
                        if let Some(idx) = self.tiling.get_closest_triangle(p) {
                             self.stack.push(self.tiling.data[idx].clone());
                        } else {
                             // Should theoretically always find one if we search, but maybe range check?
                             // Default to 0 if "out of bounds" (though Tiling is finite but space is infinite)
                             self.stack.push(Value::Int(0));
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for g_read".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for g_read".to_string());
                }
                None
            }
            "g_write" => {
                if self.stack.len() >= 3 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let val = self.stack.pop().unwrap();
                    if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                        let p = Point::new(x as f64, y as f64);
                        if let Some(idx) = self.tiling.get_closest_triangle(p) {
                             self.tiling.data[idx] = val;
                        } else {
                             self.output.push("Error: Coordinates too far from tiling".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for g_write".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for g_write".to_string());
                }
                None
            }
            "virus" => {
                if self.stack.len() >= 2 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();

                    let cell_value = if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                        let p = Point::new(x as f64, y as f64);
                        if let Some(idx) = self.tiling.get_closest_triangle(p) {
                            Some(self.tiling.data[idx].clone())
                        } else {
                            None
                        }
                    } else {
                        self.output.push("Error: Type mismatch for virus".to_string());
                        None
                    };

                    if let Some(val) = cell_value {
                        match val {
                            Value::Int(n) => self.stack.push(Value::Int(n)),
                            Value::Str(s) => {
                                return self.execute_gene(&s, &[]);
                            }
                        }
                    }
                } else {
                    self.output.push("Error: Stack underflow for virus".to_string());
                }
                None
            }
            // --- EVOLUTION ---
            "transcribe" => {
                // stack: value (top), arg_idx, gene_idx, strand_idx (bottom)
                if self.stack.len() < 4 {
                    self.output
                        .push("Error: Stack underflow for transcribe".to_string());
                    return None;
                }
                let val = self.stack.pop().unwrap();
                let arg_idx_val = self.stack.pop().unwrap();
                let gene_idx_val = self.stack.pop().unwrap();
                let strand_idx_val = self.stack.pop().unwrap();

                match (val, arg_idx_val, gene_idx_val, strand_idx_val) {
                    (Value::Int(v), Value::Int(ai), Value::Int(gi), Value::Int(si)) => {
                        if si >= 0 && (si as usize) < self.dna.helix.strands.len() {
                            let strand = &mut self.dna.helix.strands[si as usize];
                            if gi >= 0 && (gi as usize) < strand.genes.len() {
                                let gene = &mut strand.genes[gi as usize];
                                if ai >= 0 && (ai as usize) < gene.args.len() {
                                    gene.args[ai as usize] = Nucleotide::Number(v);
                                    self.output.push(format!(
                                        "TRANSCRIBE: strand {} gene {} arg {} -> {}",
                                        si, gi, ai, v
                                    ));
                                } else {
                                    self.output
                                        .push("Error: Arg index out of bounds".to_string());
                                }
                            } else {
                                self.output
                                    .push("Error: Gene index out of bounds".to_string());
                            }
                        } else {
                            self.output
                                .push("Error: Strand index out of bounds".to_string());
                        }
                    }
                    _ => self
                        .output
                        .push("Error: Type mismatch for transcribe args".to_string()),
                }
                None
            }
            "jump_s" => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(target) => {
                            if target >= 0 {
                                return Some((target as usize, 0));
                            } else {
                                self.output.push("Error: Negative jump target".to_string());
                            }
                        }
                        _ => self.output.push("Error: Type mismatch for jump_s".to_string()),
                    }
                } else {
                    self.output.push("Error: Stack underflow for jump_s".to_string());
                }
                None
            }
            "brz_s" => {
                if self.stack.len() >= 2 {
                    let target_val = self.stack.pop().unwrap();
                    let cond_val = self.stack.pop().unwrap();

                    match (target_val, cond_val) {
                        (Value::Int(target), Value::Int(cond)) => {
                            if cond == 0 {
                                if target >= 0 {
                                    return Some((target as usize, 0));
                                } else {
                                    self.output.push("Error: Negative jump target".to_string());
                                }
                            }
                        }
                        _ => self.output.push("Error: Type mismatch for brz_s".to_string()),
                    }
                } else {
                    self.output.push("Error: Stack underflow for brz_s".to_string());
                }
                None
            }
            "s_len" => {
                self.stack.push(Value::Int(self.stack.len() as i64));
                None
            }
            "helix_len" => {
                self.stack
                    .push(Value::Int(self.dna.helix.strands.len() as i64));
                None
            }
            "gene_len" => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(idx) => {
                            if idx >= 0 && (idx as usize) < self.dna.helix.strands.len() {
                                let len = self.dna.helix.strands[idx as usize].genes.len();
                                self.stack.push(Value::Int(len as i64));
                            } else {
                                self.output.push(
                                    "Error: Strand index out of bounds for gene_len".to_string(),
                                );
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for gene_len".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for gene_len".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "methylate" => {
                if self.stack.len() >= 2 {
                    let gene_val = self.stack.pop().unwrap();
                    let strand_val = self.stack.pop().unwrap();
                    if let (Value::Int(g_idx), Value::Int(s_idx)) = (gene_val, strand_val) {
                        self.epigenome.insert((s_idx as usize, g_idx as usize));
                        self.output.push(format!("METHYLATED: {}:{}", s_idx, g_idx));
                    } else {
                        self.output
                            .push("Error: Invalid args for methylate".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for methylate".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "demethylate" => {
                if self.stack.len() >= 2 {
                    let gene_val = self.stack.pop().unwrap();
                    let strand_val = self.stack.pop().unwrap();
                    if let (Value::Int(g_idx), Value::Int(s_idx)) = (gene_val, strand_val) {
                        self.epigenome.remove(&(s_idx as usize, g_idx as usize));
                        self.output
                            .push(format!("DEMETHYLATED: {}:{}", s_idx, g_idx));
                    } else {
                        self.output
                            .push("Error: Invalid args for demethylate".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for demethylate".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "telomerase" => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(amount) => {
                            if amount > 0 {
                                let idx = self.ip.0;
                                if idx < self.telomeres.len() {
                                    self.telomeres[idx] += amount;
                                    self.energy -= 25; // High cost
                                    self.output.push(format!(
                                        "TELOMERASE: Extended strand {} by {}",
                                        idx, amount
                                    ));
                                }
                            }
                        }
                        _ => self.output.push("Error: Invalid arg for telomerase".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for telomerase".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "t_len" => {
                let idx = self.ip.0;
                if idx < self.telomeres.len() {
                    self.stack.push(Value::Int(self.telomeres[idx]));
                } else {
                    self.stack.push(Value::Int(0));
                }
                None
            }
            #[cfg(feature = "nova")]
            "recombine" => {
                // stack: split_point, strand_b, strand_a (bottom)
                if self.stack.len() >= 3 {
                    let split_val = self.stack.pop().unwrap();
                    let strand_b_val = self.stack.pop().unwrap();
                    let strand_a_val = self.stack.pop().unwrap();

                    match (strand_a_val, strand_b_val, split_val) {
                        (Value::Int(sa), Value::Int(sb), Value::Int(split)) => {
                            let helix_len = self.dna.helix.strands.len();
                            let sa_idx = sa as usize;
                            let sb_idx = sb as usize;
                            let split_idx = split as usize;

                            if sa >= 0
                                && sb >= 0
                                && split >= 0
                                && sa_idx < helix_len
                                && sb_idx < helix_len
                            {
                                let len_a = self.dna.helix.strands[sa_idx].genes.len();
                                let len_b = self.dna.helix.strands[sb_idx].genes.len();

                                if split_idx <= len_a && split_idx <= len_b {
                                    if sa_idx == sb_idx {
                                        self.output.push(
                                            "Warning: Recombining strand with itself".to_string(),
                                        );
                                    } else {
                                        let (lower, upper) = if sa_idx < sb_idx {
                                            (sa_idx, sb_idx)
                                        } else {
                                            (sb_idx, sa_idx)
                                        };

                                        let (first_slice, second_slice) =
                                            self.dna.helix.strands.split_at_mut(upper);
                                        let strand_low = &mut first_slice[lower];
                                        let strand_high = &mut second_slice[0];

                                        let (strand_a, strand_b) = if sa_idx < sb_idx {
                                            (strand_low, strand_high)
                                        } else {
                                            (strand_high, strand_low)
                                        };

                                        let mut tail_a = strand_a.genes.split_off(split_idx);
                                        let mut tail_b = strand_b.genes.split_off(split_idx);

                                        strand_a.genes.append(&mut tail_b);
                                        strand_b.genes.append(&mut tail_a);

                                        self.output.push(format!(
                                            "RECOMBINATION: Swapped tails of strand {} and {} at {}",
                                            sa, sb, split
                                        ));
                                    }
                                } else {
                                    self.output
                                        .push("Error: Split point out of bounds".to_string());
                                }
                            } else {
                                self.output
                                    .push("Error: Strand index out of bounds".to_string());
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for recombine".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for recombine".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "s_index" => {
                self.stack.push(Value::Int(self.ip.0 as i64));
                None
            }
            #[cfg(feature = "nova")]
            "mitosis" => {
                // stack: strand_idx (target to clone)
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(idx) => {
                            let s_idx = idx as usize;
                            if s_idx < self.dna.helix.strands.len() {
                                let new_strand = self.dna.helix.strands[s_idx].clone();
                                self.dna.helix.strands.push(new_strand);
                                self.telomeres.push(50);

                                let new_s_idx = self.dna.helix.strands.len() - 1;
                                let genes_to_methylate: Vec<usize> = self
                                    .epigenome
                                    .iter()
                                    .filter(|(s, _)| *s == s_idx)
                                    .map(|(_, g)| *g)
                                    .collect();

                                for g_idx in genes_to_methylate {
                                    self.epigenome.insert((new_s_idx, g_idx));
                                }

                                self.energy -= 30; // Cost
                                self.output.push(format!(
                                    "MITOSIS: Cloned strand {} to {}",
                                    s_idx, new_s_idx
                                ));
                            } else {
                                self.output.push(
                                    "Error: Strand index out of bounds for mitosis".to_string(),
                                );
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for mitosis".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for mitosis".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "apoptosis" => {
                // stack: strand_idx
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(idx) => {
                            let s_idx = idx as usize;
                            if s_idx < self.dna.helix.strands.len() {
                                self.dna.helix.strands[s_idx].genes.clear();
                                self.epigenome.retain(|(s, _)| *s != s_idx);
                                self.energy -= 10;
                                self.output.push(format!("APOPTOSIS: Cleared strand {}", s_idx));
                            } else {
                                self.output.push(
                                    "Error: Strand index out of bounds for apoptosis".to_string(),
                                );
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for apoptosis".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for apoptosis".to_string());
                }
                None
            }
            _ => {
                self.output.push(format!("Unknown enzyme: {}", name));
                None
            }
        }
    }

    fn binary_op<F>(stack: &mut Vec<Value>, output: &mut Vec<String>, op: F)
    where
        F: Fn(i64, i64) -> i64,
    {
        if stack.len() < 2 {
            output.push("Error: Stack underflow".to_string());
            return;
        }
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();

        match (a, b) {
            (Value::Int(ia), Value::Int(ib)) => stack.push(Value::Int(op(ia, ib))),
            _ => output.push("Error: Type mismatch".to_string()),
        }
    }

    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let helix = &mut self.dna.helix;
        if helix.strands.is_empty() {
            return;
        }

        let strand_idx = rng.gen_range(0..helix.strands.len());
        let strand = &mut helix.strands[strand_idx];
        if strand.genes.is_empty() {
            return;
        }

        let gene_idx = rng.gen_range(0..strand.genes.len());
        let gene = &mut strand.genes[gene_idx];

        // 50% chance to change name, 50% to change arg
        if rng.gen_bool(0.5) {
            let enzymes = [
                "push",
                "add",
                "sub",
                "mul",
                "div",
                "dup",
                "print",
                "swap",
                "drop",
                "jump",
                "brz",
                "photosynthesize",
                "consume",
                "transcribe",
                "s_len",
                "helix_len",
                "gene_len",
                "g_read",
                "g_write",
                #[cfg(feature = "nova")]
                "telomerase",
                #[cfg(feature = "nova")]
                "t_len",
                #[cfg(feature = "nova")]
                "s_index",
                #[cfg(feature = "nova")]
                "mitosis",
                #[cfg(feature = "nova")]
                "apoptosis",
            ];
            let new_name = enzymes[rng.gen_range(0..enzymes.len())];
            // Add "Mutation" log
            self.output
                .push(format!("MUTATION: {} -> {}", gene.name, new_name));
            gene.name = new_name.to_string();
        } else if !gene.args.is_empty() {
            if let Some(Nucleotide::Number(n)) = gene.args.first_mut() {
                let old_n = *n;
                *n = rng.gen_range(0..100); // Random number
                self.output
                    .push(format!("MUTATION: arg {} -> {}", old_n, *n));
            }
        }
    }
}
