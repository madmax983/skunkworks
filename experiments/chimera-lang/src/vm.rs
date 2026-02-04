use crate::ast::{Dna, Nucleotide};
use rand::Rng;
#[cfg(feature = "nova")]
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Str(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Str(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[cfg(feature = "nova")]
#[derive(Clone)]
pub struct Spore {
    pub dna: Dna,
    pub stack: Vec<Value>,
    pub ip: (usize, usize),
    pub output: Vec<String>,
    pub halted: bool,
    pub energy: i64,
    pub grid: Vec<Vec<Value>>,
    pub chaos_mode: bool,
    pub recursion_depth: usize,
    pub context_loc: (usize, usize),
    pub epigenome: HashSet<(usize, usize)>,
    pub telomeres: Vec<i64>,
    pub hormone_grid: Vec<Vec<[i64; 3]>>,
    pub quantum_entanglement: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

pub struct ChimeraVM {
    pub dna: Dna,
    pub stack: Vec<Value>,
    pub ip: (usize, usize), // (strand_idx, gene_idx)
    pub output: Vec<String>,
    pub halted: bool,
    pub energy: i64,
    pub grid: Vec<Vec<Value>>,
    pub chaos_mode: bool,
    pub recursion_depth: usize,
    pub context_loc: (usize, usize),
    #[cfg(feature = "nova")]
    pub epigenome: HashSet<(usize, usize)>,
    #[cfg(feature = "nova")]
    pub telomeres: Vec<i64>,
    #[cfg(feature = "nova")]
    pub hormone_grid: Vec<Vec<[i64; 3]>>,
    #[cfg(feature = "nova")]
    pub quantum_entanglement: HashMap<(usize, usize), Vec<(usize, usize)>>,
    #[cfg(feature = "nova")]
    pub spores: Vec<Spore>,
}

impl ChimeraVM {
    pub fn new(dna: Dna) -> Self {
        // Initialize 16x16 grid with 0s
        let grid = vec![vec![Value::Int(0); 16]; 16];
        #[cfg(feature = "nova")]
        let strand_count = dna.helix.strands.len();
        #[cfg(feature = "nova")]
        let hormone_grid = vec![vec![[0, 0, 0]; 16]; 16];

        Self {
            dna,
            stack: Vec::new(),
            ip: (0, 0),
            output: Vec::new(),
            halted: false,
            energy: 50,
            grid,
            chaos_mode: false,
            recursion_depth: 0,
            context_loc: (8, 8),
            #[cfg(feature = "nova")]
            epigenome: HashSet::new(),
            #[cfg(feature = "nova")]
            telomeres: vec![50; strand_count],
            #[cfg(feature = "nova")]
            hormone_grid,
            #[cfg(feature = "nova")]
            quantum_entanglement: HashMap::new(),
            #[cfg(feature = "nova")]
            spores: Vec::new(),
        }
    }

    #[cfg(feature = "nova")]
    #[allow(clippy::needless_range_loop)]
    fn diffuse_hormones(&mut self) {
        let mut new_grid = self.hormone_grid.clone();
        for y in 0..16 {
            for x in 0..16 {
                for c in 0..3 {
                    let mut sum = self.hormone_grid[y][x][c] * 4;
                    let mut count = 4;

                    if y > 0 {
                        sum += self.hormone_grid[y - 1][x][c];
                        count += 1;
                    }
                    if y < 15 {
                        sum += self.hormone_grid[y + 1][x][c];
                        count += 1;
                    }
                    if x > 0 {
                        sum += self.hormone_grid[y][x - 1][c];
                        count += 1;
                    }
                    if x < 15 {
                        sum += self.hormone_grid[y][x + 1][c];
                        count += 1;
                    }

                    new_grid[y][x][c] = sum / count;
                }
            }
        }
        self.hormone_grid = new_grid;
    }

    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        self.energy -= 1;

        #[cfg(feature = "nova")]
        {
            self.diffuse_hormones();
            // Decay hormones: reduce intensity by 1 per step
            for row in self.hormone_grid.iter_mut() {
                for cell in row.iter_mut() {
                    for val in cell.iter_mut() {
                        if *val > 0 {
                            *val -= 1;
                        }
                    }
                }
            }
        }

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
                    self.output
                        .push(format!("SENESCENCE: Strand {} decayed", self.ip.0));
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

    fn write_grid(&mut self, start_x: usize, start_y: usize, val: Value) {
        if start_x >= 16 || start_y >= 16 {
            return;
        }

        #[cfg(feature = "nova")]
        {
            let mut stack = Vec::new();
            let mut visited = HashSet::new();

            stack.push((start_x, start_y));
            visited.insert((start_x, start_y));

            while let Some((x, y)) = stack.pop() {
                if x < 16 && y < 16 {
                    self.grid[y][x] = val.clone();
                }

                if let Some(links) = self.quantum_entanglement.get(&(x, y)) {
                    for &(lx, ly) in links {
                        if !visited.contains(&(lx, ly)) {
                            visited.insert((lx, ly));
                            stack.push((lx, ly));
                        }
                    }
                }
            }
        }

        #[cfg(not(feature = "nova"))]
        {
            self.grid[start_y][start_x] = val;
        }
    }

    fn execute_gene(&mut self, name: &str, args: &[Nucleotide]) -> Option<(usize, usize)> {
        if self.recursion_depth > 100 {
            self.output
                .push("Error: Recursion limit exceeded".to_string());
            return None;
        }
        self.recursion_depth += 1;
        let result = self.execute_gene_inner(name, args);
        self.recursion_depth -= 1;
        result
    }

    fn get_circular_coords(&self, cx: i64, cy: i64, r: i64) -> Vec<(usize, usize)> {
        let mut coords = Vec::new();
        let r_sq = r * r;
        for y in 0..16 {
            for x in 0..16 {
                let dx = x as i64 - cx;
                let dy = y as i64 - cy;
                if dx * dx + dy * dy <= r_sq {
                    coords.push((x, y));
                }
            }
        }
        coords
    }

    pub(crate) fn execute_gene_inner(
        &mut self,
        name: &str,
        args: &[Nucleotide],
    ) -> Option<(usize, usize)> {
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
            #[cfg(feature = "nova")]
            "sporulate" => {
                // Create snapshot
                let spore = Spore {
                    dna: self.dna.clone(),
                    stack: self.stack.clone(),
                    ip: self.ip,
                    output: self.output.clone(),
                    halted: self.halted,
                    energy: self.energy,
                    grid: self.grid.clone(),
                    chaos_mode: self.chaos_mode,
                    recursion_depth: self.recursion_depth,
                    context_loc: self.context_loc,
                    epigenome: self.epigenome.clone(),
                    telomeres: self.telomeres.clone(),
                    hormone_grid: self.hormone_grid.clone(),
                    quantum_entanglement: self.quantum_entanglement.clone(),
                };

                let id = self.spores.len();
                self.spores.push(spore);
                self.stack.push(Value::Int(id as i64));
                self.energy -= 50; // High cost for time travel
                self.output.push(format!("SPORULATE: Created Spore {}", id));
                None
            }
            #[cfg(feature = "nova")]
            "germinate" => {
                // stack: spore_id
                if let Some(val) = self.stack.pop() {
                    if let Value::Int(id) = val {
                        let idx = id as usize;
                        if idx < self.spores.len() {
                            let spore = &self.spores[idx];
                            // Restore state
                            self.dna = spore.dna.clone();
                            self.stack = spore.stack.clone();
                            self.ip = spore.ip;
                            self.output = spore.output.clone();
                            self.halted = spore.halted;
                            self.energy = spore.energy;
                            self.grid = spore.grid.clone();
                            self.chaos_mode = spore.chaos_mode;
                            self.recursion_depth = spore.recursion_depth;
                            self.context_loc = spore.context_loc;
                            self.epigenome = spore.epigenome.clone();
                            self.telomeres = spore.telomeres.clone();
                            self.hormone_grid = spore.hormone_grid.clone();
                            self.quantum_entanglement = spore.quantum_entanglement.clone();

                            self.output
                                .push(format!("GERMINATE: Restored Spore {}", idx));
                        } else {
                            self.output
                                .push("Error: Spore index out of bounds".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for germinate".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for germinate".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "incubate" => {
                // stack: len, y, x (top)
                if self.stack.len() >= 3 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let len_val = self.stack.pop().unwrap();

                    if let (Value::Int(x), Value::Int(y), Value::Int(len)) = (x_val, y_val, len_val)
                    {
                        if len > 0 {
                            let mut genes = Vec::new();
                            let mut valid = true;
                            for i in 0..len {
                                let curr_x = x + i;
                                if (0..16).contains(&y) && (0..16).contains(&curr_x) {
                                    let val = &self.grid[y as usize][curr_x as usize];
                                    match val {
                                        Value::Int(n) => {
                                            genes.push(crate::ast::Gene {
                                                name: "push".to_string(),
                                                args: vec![crate::ast::Nucleotide::Number(*n)],
                                            });
                                        }
                                        Value::Str(s) => {
                                            genes.push(crate::ast::Gene {
                                                name: s.clone(),
                                                args: vec![],
                                            });
                                        }
                                    }
                                } else {
                                    valid = false;
                                    self.output
                                        .push("Error: Incubate range out of bounds".to_string());
                                    break;
                                }
                            }

                            if valid {
                                self.dna.helix.strands.push(crate::ast::Strand { genes });
                                self.telomeres.push(50);
                                self.energy -= 20; // Cost
                                self.output.push(format!(
                                    "INCUBATE: Created new strand {} from grid",
                                    self.dna.helix.strands.len() - 1
                                ));
                            }
                        } else {
                            self.output
                                .push("Error: Invalid length for incubate".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for incubate".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for incubate".to_string());
                }
                None
            }
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
                        if (0..16).contains(&y) && (0..16).contains(&x) {
                            self.stack.push(self.grid[y as usize][x as usize].clone());
                        } else {
                            self.output
                                .push("Error: Grid index out of bounds".to_string());
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
                        if (0..16).contains(&y) && (0..16).contains(&x) {
                            self.write_grid(x as usize, y as usize, val);
                        } else {
                            self.output
                                .push("Error: Grid index out of bounds".to_string());
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
            "radiate" => {
                // stack: val, radius, y, x (top)
                if self.stack.len() >= 4 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let r_val = self.stack.pop().unwrap();
                    let val = self.stack.pop().unwrap();

                    if let (Value::Int(x), Value::Int(y), Value::Int(r)) = (x_val, y_val, r_val) {
                        let coords = self.get_circular_coords(x, y, r);
                        let count = coords.len();
                        for (cx, cy) in coords {
                            self.write_grid(cx, cy, val.clone());
                        }
                        self.energy -= (count / 2) as i64; // Cost based on area
                        self.output.push(format!(
                            "RADIATE: Affected {} cells at {},{} r={}",
                            count, x, y, r
                        ));
                    } else {
                        self.output
                            .push("Error: Type mismatch for radiate".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for radiate".to_string());
                }
                None
            }
            "siphon" => {
                // stack: radius, y, x (top)
                if self.stack.len() >= 3 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let r_val = self.stack.pop().unwrap();

                    if let (Value::Int(x), Value::Int(y), Value::Int(r)) = (x_val, y_val, r_val) {
                        let coords = self.get_circular_coords(x, y, r);
                        let count = coords.len();
                        let mut sum = 0;
                        for (cx, cy) in coords {
                            if let Value::Int(n) = self.grid[cy][cx] {
                                sum += n;
                            }
                            self.write_grid(cx, cy, Value::Int(0));
                        }
                        self.stack.push(Value::Int(sum));
                        self.energy -= 5;
                        self.output
                            .push(format!("SIPHON: Absorbed {} from {} cells", sum, count));
                    } else {
                        self.output
                            .push("Error: Type mismatch for siphon".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for siphon".to_string());
                }
                None
            }
            "genome" => {
                // Pushes genes of current strand to stack
                if self.ip.0 < self.dna.helix.strands.len() {
                    let strand = &self.dna.helix.strands[self.ip.0];
                    self.stack.push(Value::Int(strand.genes.len() as i64));
                    for gene in &strand.genes {
                        self.stack.push(Value::Str(gene.name.clone()));
                    }
                }
                None
            }
            "virus" => {
                if self.stack.len() >= 2 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();

                    let coords = if let (Value::Int(y), Value::Int(x)) = (&y_val, &x_val) {
                        if (0..16).contains(y) && (0..16).contains(x) {
                            Some((*y, *x))
                        } else {
                            self.output
                                .push("Error: Grid index out of bounds".to_string());
                            None
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for virus".to_string());
                        None
                    };

                    if let Some((y, x)) = coords {
                        let val = self.grid[y as usize][x as usize].clone();
                        match val {
                            Value::Int(n) => self.stack.push(Value::Int(n)),
                            Value::Str(s) => {
                                // Execute enzyme recursively
                                // We pass empty args because grid enzymes don't carry args
                                let old_loc = self.context_loc;
                                self.context_loc = (y as usize, x as usize);
                                let result = self.execute_gene(&s, &[]);
                                self.context_loc = old_loc;
                                return result;
                            }
                        }
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for virus".to_string());
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
                        _ => self
                            .output
                            .push("Error: Type mismatch for jump_s".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for jump_s".to_string());
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
                        _ => self
                            .output
                            .push("Error: Type mismatch for brz_s".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for brz_s".to_string());
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
                        _ => self
                            .output
                            .push("Error: Invalid arg for telomerase".to_string()),
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
                                // We need to check split bounds for both strands
                                let len_a = self.dna.helix.strands[sa_idx].genes.len();
                                let len_b = self.dna.helix.strands[sb_idx].genes.len();

                                if split_idx <= len_a && split_idx <= len_b {
                                    // Perform recombination
                                    // We need to borrow strands mutably.
                                    // Since they are in the same Vec, we need split_at_mut or similar trickery,
                                    // or just use indices if we can modify the Vec safely.
                                    // We can't get two mutable references to the same Vec at different indices directly.
                                    // So we'll use `split_at_mut` if they are different indices, or just do nothing if same.

                                    if sa_idx == sb_idx {
                                        // Recombining same strand with itself at same point is a no-op.
                                        self.output.push(
                                            "Warning: Recombining strand with itself".to_string(),
                                        );
                                    } else {
                                        // Ensure ordered access to avoid panic
                                        let (lower, upper) = if sa_idx < sb_idx {
                                            (sa_idx, sb_idx)
                                        } else {
                                            (sb_idx, sa_idx)
                                        };

                                        let (first_slice, second_slice) =
                                            self.dna.helix.strands.split_at_mut(upper);
                                        let strand_low = &mut first_slice[lower];
                                        let strand_high = &mut second_slice[0]; // relative index 0 is absolute 'upper'

                                        // Identify which is A and B
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
            "crispr_scan" => {
                // stack: guide_idx, target_idx (bottom)
                if self.stack.len() >= 2 {
                    let guide_val = self.stack.pop().unwrap();
                    let target_val = self.stack.pop().unwrap();

                    if let (Value::Int(g_idx), Value::Int(t_idx)) = (guide_val, target_val) {
                        let g_idx = g_idx as usize;
                        let t_idx = t_idx as usize;
                        let helix_len = self.dna.helix.strands.len();

                        if g_idx < helix_len && t_idx < helix_len {
                            let guide_strand = &self.dna.helix.strands[g_idx];
                            let target_strand = &self.dna.helix.strands[t_idx];

                            // We need to match sequence of gene names
                            let guide_names: Vec<String> =
                                guide_strand.genes.iter().map(|g| g.name.clone()).collect();
                            let target_names: Vec<String> =
                                target_strand.genes.iter().map(|g| g.name.clone()).collect();

                            let mut found_idx: i64 = -1;

                            if !guide_names.is_empty() && guide_names.len() <= target_names.len() {
                                for i in 0..=(target_names.len() - guide_names.len()) {
                                    if target_names[i..i + guide_names.len()] == guide_names[..] {
                                        found_idx = i as i64;
                                        break;
                                    }
                                }
                            }

                            self.stack.push(Value::Int(found_idx));
                            self.energy -= 5;
                            self.output.push(format!(
                                "CRISPR_SCAN: Scanned strand {} for pattern from {} -> {}",
                                t_idx, g_idx, found_idx
                            ));
                        } else {
                            self.output.push(
                                "Error: Strand index out of bounds for crispr_scan".to_string(),
                            );
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for crispr_scan".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for crispr_scan".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "cas9_cut" => {
                // stack: cut_index, strand_idx (bottom)
                if self.stack.len() >= 2 {
                    let cut_val = self.stack.pop().unwrap();
                    let strand_val = self.stack.pop().unwrap();

                    if let (Value::Int(cut), Value::Int(s_idx)) = (cut_val, strand_val) {
                        let s_idx = s_idx as usize;
                        let cut_idx = cut as usize;
                        let helix_len = self.dna.helix.strands.len();

                        if s_idx < helix_len {
                            let strand_len = self.dna.helix.strands[s_idx].genes.len();
                            if cut >= 0 && cut_idx <= strand_len {
                                // Perform split
                                // We need to mutate the strand.
                                let strand = &mut self.dna.helix.strands[s_idx];
                                let tail_genes = strand.genes.split_off(cut_idx);

                                // Create new strand
                                self.dna
                                    .helix
                                    .strands
                                    .push(crate::ast::Strand { genes: tail_genes });
                                self.telomeres.push(50);

                                let new_strand_idx = self.dna.helix.strands.len() - 1;

                                self.stack.push(Value::Int(new_strand_idx as i64));
                                self.energy -= 10;

                                self.output.push(format!(
                                    "CAS9_CUT: Cut strand {} at {}, created strand {}",
                                    s_idx, cut_idx, new_strand_idx
                                ));

                                // Handling Self-Modification Safety:
                                // If we cut the strand we are currently executing,
                                // and the cut point is BEFORE or AT our current IP, execution context moves.
                                if s_idx == self.ip.0 && self.ip.1 >= cut_idx {
                                    let new_gene_idx = self.ip.1 - cut_idx;
                                    // We jump to the next instruction in the NEW strand
                                    return Some((new_strand_idx, new_gene_idx + 1));
                                }
                            } else {
                                self.output
                                    .push("Error: Cut index out of bounds".to_string());
                            }
                        } else {
                            self.output
                                .push("Error: Strand index out of bounds for cas9_cut".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for cas9_cut".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for cas9_cut".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "ligase" => {
                // stack: donor_idx, recipient_idx (bottom)
                if self.stack.len() >= 2 {
                    let donor_val = self.stack.pop().unwrap();
                    let recipient_val = self.stack.pop().unwrap();

                    if let (Value::Int(d_idx), Value::Int(r_idx)) = (donor_val, recipient_val) {
                        let d_idx = d_idx as usize;
                        let r_idx = r_idx as usize;
                        let helix_len = self.dna.helix.strands.len();

                        if d_idx < helix_len && r_idx < helix_len {
                            if d_idx == r_idx {
                                self.output
                                    .push("Warning: Ligase on same strand is no-op".to_string());
                            } else {
                                // We need to move genes from donor to recipient.
                                let (lower, upper) = if d_idx < r_idx {
                                    (d_idx, r_idx)
                                } else {
                                    (r_idx, d_idx)
                                };

                                let (first_slice, second_slice) =
                                    self.dna.helix.strands.split_at_mut(upper);
                                let strand_low = &mut first_slice[lower];
                                let strand_high = &mut second_slice[0];

                                let (strand_d, strand_r) = if d_idx < r_idx {
                                    (strand_low, strand_high)
                                } else {
                                    (strand_high, strand_low)
                                };

                                strand_r.genes.append(&mut strand_d.genes);
                                // donor genes are now empty.

                                self.energy -= 10;
                                self.output.push(format!(
                                    "LIGASE: Appended strand {} to {}",
                                    d_idx, r_idx
                                ));
                            }
                        } else {
                            self.output
                                .push("Error: Strand index out of bounds for ligase".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for ligase".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for ligase".to_string());
                }
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
                                // Clone the strand
                                let new_strand = self.dna.helix.strands[s_idx].clone();
                                self.dna.helix.strands.push(new_strand);
                                self.telomeres.push(50); // Default life

                                // Inherit epigenetics
                                // We need to find all keys (s_idx, g_idx) and insert (new_idx, g_idx)
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

                                // Remove associated epigenetics
                                self.epigenome.retain(|(s, _)| *s != s_idx);

                                self.energy -= 10;
                                self.output
                                    .push(format!("APOPTOSIS: Cleared strand {}", s_idx));
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
            #[cfg(feature = "nova")]
            "integrase" => {
                // stack: arg, name, gene_idx, strand_idx (bottom)
                if self.stack.len() >= 4 {
                    let arg_val = self.stack.pop().unwrap();
                    let name_val = self.stack.pop().unwrap();
                    let gene_idx_val = self.stack.pop().unwrap();
                    let strand_idx_val = self.stack.pop().unwrap();

                    match (strand_idx_val, gene_idx_val, name_val, arg_val) {
                        (Value::Int(s), Value::Int(g), Value::Str(name), arg) => {
                            let s_idx = s as usize;
                            let g_idx = g as usize;
                            let helix_len = self.dna.helix.strands.len();

                            if s >= 0 && s_idx < helix_len {
                                let strand_len = self.dna.helix.strands[s_idx].genes.len();
                                if g >= 0 && g_idx <= strand_len {
                                    // Create Gene
                                    let new_gene = crate::ast::Gene {
                                        name: name.clone(),
                                        args: match arg {
                                            Value::Int(n) => {
                                                vec![crate::ast::Nucleotide::Number(n)]
                                            }
                                            Value::Str(s) => {
                                                vec![crate::ast::Nucleotide::String(s)]
                                            }
                                        },
                                    };

                                    // Insert
                                    self.dna.helix.strands[s_idx].genes.insert(g_idx, new_gene);

                                    // Update Epigenome: Shift all markers at (s_idx, k >= g_idx) to k+1
                                    let mut new_markers = Vec::new();
                                    let mut to_remove = Vec::new();
                                    for &(ms, mg) in self.epigenome.iter() {
                                        if ms == s_idx && mg >= g_idx {
                                            to_remove.push((ms, mg));
                                            new_markers.push((ms, mg + 1));
                                        }
                                    }
                                    for marker in to_remove {
                                        self.epigenome.remove(&marker);
                                    }
                                    for marker in new_markers {
                                        self.epigenome.insert(marker);
                                    }

                                    self.energy -= 20;
                                    self.output.push(format!(
                                        "INTEGRASE: Inserted {} at {}:{}",
                                        name, s, g
                                    ));

                                    // Update IP if we inserted before or at current execution
                                    if self.ip.0 == s_idx && self.ip.1 >= g_idx {
                                        self.ip.1 += 1;
                                    }
                                    // Default None means step() will increment IP +1.
                                    // If we shifted IP +1 here, total is +2.
                                    // This skips the inserted gene (if at g_idx) and the current gene (now at g_idx+1).
                                    // Correct.
                                } else {
                                    self.output.push(
                                        "Error: Gene index out of bounds for integrase".to_string(),
                                    );
                                }
                            } else {
                                self.output.push(
                                    "Error: Strand index out of bounds for integrase".to_string(),
                                );
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for integrase".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for integrase".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "excision" => {
                // stack: gene_idx, strand_idx (bottom)
                if self.stack.len() >= 2 {
                    let gene_idx_val = self.stack.pop().unwrap();
                    let strand_idx_val = self.stack.pop().unwrap();

                    match (strand_idx_val, gene_idx_val) {
                        (Value::Int(s), Value::Int(g)) => {
                            let s_idx = s as usize;
                            let g_idx = g as usize;
                            let helix_len = self.dna.helix.strands.len();

                            if s >= 0 && s_idx < helix_len {
                                let strand_len = self.dna.helix.strands[s_idx].genes.len();
                                if g >= 0 && g_idx < strand_len {
                                    // Remove
                                    self.dna.helix.strands[s_idx].genes.remove(g_idx);

                                    // Update Epigenome: Remove marker at g_idx, shift k > g_idx to k-1
                                    self.epigenome.remove(&(s_idx, g_idx));
                                    let mut new_markers = Vec::new();
                                    let mut to_remove = Vec::new();
                                    for &(ms, mg) in self.epigenome.iter() {
                                        if ms == s_idx && mg > g_idx {
                                            to_remove.push((ms, mg));
                                            new_markers.push((ms, mg - 1));
                                        }
                                    }
                                    for marker in to_remove {
                                        self.epigenome.remove(&marker);
                                    }
                                    for marker in new_markers {
                                        self.epigenome.insert(marker);
                                    }

                                    self.energy -= 15;
                                    self.output.push(format!("EXCISION: Removed {}:{}", s, g));

                                    // Update IP
                                    if self.ip.0 == s_idx {
                                        if g_idx < self.ip.1 {
                                            // Removed before current. Shift IP left.
                                            self.ip.1 -= 1;
                                            return None; // step() increments +1. Net 0 change (but content moved left, so we execute next).
                                        } else if g_idx == self.ip.1 {
                                            // Removed current.
                                            // Next gene slid into current slot.
                                            // We want to execute it.
                                            // So return IP as is, prevent step() increment.
                                            return Some(self.ip);
                                        }
                                    }
                                } else {
                                    self.output.push(
                                        "Error: Gene index out of bounds for excision".to_string(),
                                    );
                                }
                            } else {
                                self.output.push(
                                    "Error: Strand index out of bounds for excision".to_string(),
                                );
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for excision".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for excision".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "secrete" => {
                // stack: channel, amount (top)
                if self.stack.len() >= 2 {
                    let amount_val = self.stack.pop().unwrap();
                    let channel_val = self.stack.pop().unwrap();
                    if let (Value::Int(c), Value::Int(a)) = (channel_val, amount_val) {
                        if a > 0 {
                            let (cy, cx) = self.context_loc;
                            let channel_idx = (c.unsigned_abs() as usize) % 3;
                            self.hormone_grid[cy][cx][channel_idx] += a;
                            self.output.push(format!(
                                "SECRETE: Added {} to channel {} at {},{}",
                                a, c, cx, cy
                            ));
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for secrete".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for secrete".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "detect" => {
                // stack: channel
                if let Some(val) = self.stack.pop() {
                    if let Value::Int(c) = val {
                        let (cy, cx) = self.context_loc;
                        let channel_idx = (c.unsigned_abs() as usize) % 3;
                        let intensity = self.hormone_grid[cy][cx][channel_idx];
                        self.stack.push(Value::Int(intensity));
                    } else {
                        self.output
                            .push("Error: Type mismatch for detect".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for detect".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "absorb" => {
                // stack: channel, amount (top)
                if self.stack.len() >= 2 {
                    let amount_val = self.stack.pop().unwrap();
                    let channel_val = self.stack.pop().unwrap();
                    if let (Value::Int(c), Value::Int(a)) = (channel_val, amount_val) {
                        let (cy, cx) = self.context_loc;
                        let channel_idx = (c.unsigned_abs() as usize) % 3;
                        let intensity = &mut self.hormone_grid[cy][cx][channel_idx];
                        let absorbed = if *intensity >= a {
                            *intensity -= a;
                            a
                        } else {
                            let v = *intensity;
                            *intensity = 0;
                            v
                        };
                        self.stack.push(Value::Int(absorbed));
                        self.output.push(format!(
                            "ABSORB: Consumed {} from channel {} at {},{}",
                            absorbed, c, cx, cy
                        ));
                    } else {
                        self.output
                            .push("Error: Type mismatch for absorb".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for absorb".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "entangle" => {
                // stack: y, x (top)
                if self.stack.len() >= 2 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();

                    if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
                        if (0..16).contains(&x) && (0..16).contains(&y) {
                            let target = (x as usize, y as usize);
                            let source = self.context_loc;

                            if target != source {
                                // Add link source -> target
                                self.quantum_entanglement
                                    .entry(source)
                                    .or_default()
                                    .push(target);
                                // Add link target -> source
                                self.quantum_entanglement
                                    .entry(target)
                                    .or_default()
                                    .push(source);

                                self.energy -= 10;
                                self.output.push(format!(
                                    "ENTANGLE: Linked {},{} <-> {},{}",
                                    source.0, source.1, target.0, target.1
                                ));
                            }
                        } else {
                            self.output
                                .push("Error: Grid index out of bounds for entangle".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for entangle".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for entangle".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            "decohere" => {
                // Decohere current context_loc
                let source = self.context_loc;

                // We need to remove source from all its neighbors' lists
                if let Some(neighbors) = self.quantum_entanglement.remove(&source) {
                    for neighbor in neighbors {
                        if let Some(n_neighbors) = self.quantum_entanglement.get_mut(&neighbor) {
                            n_neighbors.retain(|&x| x != source);
                        }
                    }
                    self.energy -= 5;
                    self.output
                        .push(format!("DECOHERE: Unlinked {},{}", source.0, source.1));
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
                "radiate",
                "siphon",
                "genome",
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
                #[cfg(feature = "nova")]
                "crispr_scan",
                #[cfg(feature = "nova")]
                "cas9_cut",
                #[cfg(feature = "nova")]
                "ligase",
                #[cfg(feature = "nova")]
                "entangle",
                #[cfg(feature = "nova")]
                "decohere",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_add() {
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(20)],
            },
            Gene {
                name: "add".to_string(),
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }
        assert_eq!(vm.stack.len(), 1);
        match vm.stack[0] {
            Value::Int(30) => (),
            _ => panic!("Expected 30"),
        }
    }

    #[test]
    fn test_jump() {
        // [ jump(1) push(100) ] [ push(200) ]
        let strand0 = Strand {
            genes: vec![
                Gene {
                    name: "jump".to_string(),
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(100)],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(200)],
            }],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Step 1: jump(1)
        vm.step();
        assert_eq!(vm.ip, (1, 0));

        // Step 2: push(200)
        vm.step();
        assert_eq!(vm.stack.len(), 1);
        if let Value::Int(i) = vm.stack[0] {
            assert_eq!(i, 200);
        } else {
            panic!("Expected 200");
        }
    }

    #[test]
    fn test_brz() {
        // [ push(0) brz(1) push(100) ] [ push(200) ]
        let strand0 = Strand {
            genes: vec![
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    name: "brz".to_string(),
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(100)],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(200)],
            }],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Step 1: push(0)
        vm.step();

        // Step 2: brz(1) -> pops 0, jumps to (1, 0)
        vm.step();
        assert_eq!(vm.ip, (1, 0));
        assert_eq!(vm.stack.len(), 0);

        // Step 3: push(200)
        vm.step();
        if let Value::Int(i) = vm.stack[0] {
            assert_eq!(i, 200);
        } else {
            panic!("Expected 200");
        }
    }

    #[test]
    fn test_transcribe() {
        // [ push(0) push(0) push(0) push(99) transcribe() push(0) ]
        // The last push(0) should be modified to push(99)
        // stack order: strand, gene, arg, value
        // strand 0, gene 5 (the last push), arg 0 -> 99
        let strand0 = Strand {
            genes: vec![
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(0)],
                }, // 0: strand idx
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(5)],
                }, // 1: gene idx
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(0)],
                }, // 2: arg idx
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(99)],
                }, // 3: value
                Gene {
                    name: "transcribe".to_string(),
                    args: vec![],
                }, // 4: transcribe
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(0)],
                }, // 5: target to be modified
            ],
        };
        let dna = Dna {
            helix: Helix {
                strands: vec![strand0],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        for _ in 0..5 {
            vm.step();
        }

        // After transcribe, check if the last gene is modified
        if let Nucleotide::Number(n) = vm.dna.helix.strands[0].genes[5].args[0] {
            assert_eq!(n, 99);
        } else {
            panic!("Gene not modified");
        }

        // Execute the modified gene
        vm.step();
        assert_eq!(vm.stack.last().unwrap(), &Value::Int(99));
    }

    #[test]
    fn test_div_by_zero() {
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                name: "div".to_string(),
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        // We expect an error message instead of a panic
        assert!(
            vm.output.iter().any(|s| s.contains("Division by zero")),
            "Expected 'Division by zero' error, got: {:?}",
            vm.output
        );
    }

    #[test]
    fn test_div_overflow() {
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(i64::MIN)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(-1)],
            },
            Gene {
                name: "div".to_string(),
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        assert!(
            vm.output.iter().any(|s| s.contains("Division overflow")),
            "Expected 'Division overflow' error, got: {:?}",
            vm.output
        );
    }

    #[test]
    fn test_stack_underflow() {
        let genes = vec![Gene {
            name: "add".to_string(), // Requires 2 args, stack has 0
            args: vec![],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        assert!(
            vm.output.iter().any(|s| s.contains("Stack underflow")),
            "Expected 'Stack underflow' error, got: {:?}",
            vm.output
        );
    }

    #[test]
    fn test_type_mismatch() {
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::String("foo".to_string())],
            },
            Gene {
                name: "add".to_string(),
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        assert!(
            vm.output.iter().any(|s| s.contains("Type mismatch")),
            "Expected 'Type mismatch' error, got: {:?}",
            vm.output
        );
    }

    #[test]
    fn test_starvation() {
        // [ jump(0) ] - infinite loop, no food
        let genes = vec![Gene {
            name: "jump".to_string(),
            args: vec![Nucleotide::Number(0)],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));
        // Start energy is 50. Should die after 50 steps.
        for _ in 0..60 {
            vm.step();
        }
        assert!(vm.halted);
        assert!(vm.output.contains(&"DEATH: STARVATION".to_string()));
    }

    #[test]
    fn test_metabolism() {
        // [ photosynthesize() jump(0) ]
        // Cost: 2 per loop. Gain: 5 per loop. Net +3.
        let genes = vec![
            Gene {
                name: "photosynthesize".to_string(),
                args: vec![],
            },
            Gene {
                name: "jump".to_string(),
                args: vec![Nucleotide::Number(0)],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        #[cfg(feature = "nova")]
        {
            // Give enough telomeres for the test
            vm.telomeres[0] = 1000;
        }

        for _ in 0..100 {
            vm.step();
        }
        assert!(!vm.halted);
        assert!(vm.energy > 50);
    }

    #[test]
    fn test_consume_survival() {
        // [ push(10) consume() jump(0) ]
        // Cost: 3 per loop. Gain: 10 per loop. Net +7.
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                name: "consume".to_string(),
                args: vec![],
            },
            Gene {
                name: "jump".to_string(),
                args: vec![Nucleotide::Number(0)],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        for _ in 0..100 {
            vm.step();
        }
        assert!(!vm.halted);
        assert!(vm.energy > 50);
    }

    #[test]
    fn test_grid_ops() {
        // [ push(42) push(10) push(10) g_write() push(10) push(10) g_read() ]
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(10)],
            }, // y
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(10)],
            }, // x
            Gene {
                name: "g_write".to_string(),
                args: vec![],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(10)],
            }, // y
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(10)],
            }, // x
            Gene {
                name: "g_read".to_string(),
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }
        assert_eq!(vm.stack.len(), 1);
        match vm.stack[0] {
            Value::Int(42) => (),
            _ => panic!("Expected 42"),
        }

        // Verify grid state directly
        match vm.grid[10][10] {
            Value::Int(42) => (),
            _ => panic!("Grid not updated"),
        }
    }

    #[test]
    fn test_virus_int() {
        // [ push(42) push(5) push(5) g_write() push(5) push(5) virus() ]
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "g_write".to_string(),
                args: vec![],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "virus".to_string(),
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }
        assert_eq!(vm.stack.last().unwrap(), &Value::Int(42));
    }

    #[test]
    fn test_virus_enzyme() {
        // [ push("add") push(6) push(6) g_write() push(10) push(20) push(6) push(6) virus() ]
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::String("add".to_string())],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(6)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(6)],
            },
            Gene {
                name: "g_write".to_string(),
                args: vec![],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(20)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(6)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(6)],
            },
            Gene {
                name: "virus".to_string(),
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(30));
    }

    #[test]
    fn test_jump_s() {
        // [ push(1) jump_s() push(100) ] [ push(200) ]
        let strand0 = Strand {
            genes: vec![
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    name: "jump_s".to_string(),
                    args: vec![],
                },
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(100)],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(200)],
            }],
        };
        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        vm.step(); // push(1)
        vm.step(); // jump_s
        assert_eq!(vm.ip, (1, 0));

        vm.step(); // push(200)
        assert_eq!(vm.stack.pop(), Some(Value::Int(200)));
    }

    #[test]
    fn test_radiate() {
        // [ push(100) push(2) push(8) push(8) radiate() ]
        // Writes 100 to circle radius 2 at 8,8
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                name: "radiate".to_string(),
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        // Center should be 100
        assert_eq!(vm.grid[8][8], Value::Int(100));
        // (8+1, 8) should be 100
        assert_eq!(vm.grid[8][9], Value::Int(100));
        // (8+3, 8) should be 0 (out of radius 2)
        assert_eq!(vm.grid[8][11], Value::Int(0));
    }

    #[test]
    fn test_siphon() {
        // First radiate 10s
        // [ push(10) push(1) push(5) push(5) radiate() push(1) push(5) push(5) siphon() ]
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "radiate".to_string(),
                args: vec![],
            },
            // Siphon same area
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                name: "siphon".to_string(),
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        // Center (5,5), (5,4), (5,6), (4,5), (6,5) = 5 cells. 5 * 10 = 50.

        if let Some(Value::Int(sum)) = vm.stack.pop() {
            assert_eq!(sum, 50);
        } else {
            panic!("Expected sum on stack");
        }

        // Grid should be cleared
        assert_eq!(vm.grid[5][5], Value::Int(0));
    }

    #[test]
    fn test_genome() {
        // [ genome() ]
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                name: "genome".to_string(),
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        // Stack should contain:
        // 1 (from push)
        // 2 (length)
        // "push"
        // "genome"

        let last = vm.stack.pop().unwrap();
        assert_eq!(last, Value::Str("genome".to_string()));
        let second = vm.stack.pop().unwrap();
        assert_eq!(second, Value::Str("push".to_string()));
        let len = vm.stack.pop().unwrap();
        assert_eq!(len, Value::Int(2));
    }
}
