use crate::ast::{Dna, Nucleotide};
use crate::opcode::OpCode;
use rand::Rng;
#[cfg(feature = "nova")]
use std::collections::{HashMap, HashSet, VecDeque};

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
    pub waste_grid: Vec<Vec<i64>>,
    pub call_stack: Vec<(usize, usize)>,
    pub input_buffer: VecDeque<char>,
    pub receptors: HashMap<char, usize>,
    #[cfg(feature = "cortex")]
    pub synapse_map: Vec<Vec<usize>>,
    #[cfg(feature = "cortex")]
    pub activation_levels: Vec<i64>,
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
    pub waste_grid: Vec<Vec<i64>>,
    #[cfg(feature = "nova")]
    pub spores: Vec<Spore>,
    #[cfg(feature = "nova")]
    pub call_stack: Vec<(usize, usize)>,
    #[cfg(feature = "nova")]
    pub input_buffer: VecDeque<char>,
    #[cfg(feature = "nova")]
    pub receptors: HashMap<char, usize>,
    #[cfg(feature = "cortex")]
    pub synapse_map: Vec<Vec<usize>>,
    #[cfg(feature = "cortex")]
    pub activation_levels: Vec<i64>,
}

impl ChimeraVM {
    pub fn new(dna: Dna) -> Self {
        // Initialize 16x16 grid with 0s
        let grid = vec![vec![Value::Int(0); 16]; 16];
        #[cfg(any(feature = "nova", feature = "cortex"))]
        let strand_count = dna.helix.strands.len();
        #[cfg(feature = "nova")]
        let hormone_grid = vec![vec![[0, 0, 0]; 16]; 16];
        #[cfg(feature = "nova")]
        let waste_grid = vec![vec![0; 16]; 16];
        #[cfg(feature = "cortex")]
        let synapse_map = vec![vec![]; strand_count];
        #[cfg(feature = "cortex")]
        let activation_levels = vec![0; strand_count];

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
            waste_grid,
            #[cfg(feature = "nova")]
            spores: Vec::new(),
            #[cfg(feature = "nova")]
            call_stack: Vec::new(),
            #[cfg(feature = "nova")]
            input_buffer: VecDeque::new(),
            #[cfg(feature = "nova")]
            receptors: HashMap::new(),
            #[cfg(feature = "cortex")]
            synapse_map,
            #[cfg(feature = "cortex")]
            activation_levels,
        }
    }

    #[cfg(feature = "nova")]
    pub fn handle_input(&mut self, key: char) -> bool {
        if self.receptors.contains_key(&key) {
            self.input_buffer.push_back(key);
            true
        } else {
            false
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

    #[cfg(feature = "nova")]
    #[allow(clippy::needless_range_loop)]
    fn diffuse_waste(&mut self) {
        let mut new_grid = self.waste_grid.clone();
        for y in 0..16 {
            for x in 0..16 {
                let mut sum = self.waste_grid[y][x] * 4;
                let mut count = 4;

                if y > 0 {
                    sum += self.waste_grid[y - 1][x];
                    count += 1;
                }
                if y < 15 {
                    sum += self.waste_grid[y + 1][x];
                    count += 1;
                }
                if x > 0 {
                    sum += self.waste_grid[y][x - 1];
                    count += 1;
                }
                if x < 15 {
                    sum += self.waste_grid[y][x + 1];
                    count += 1;
                }

                new_grid[y][x] = sum / count;
            }
        }
        self.waste_grid = new_grid;
    }

    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        self.energy -= 1;

        #[cfg(feature = "nova")]
        {
            if let Some(key) = self.input_buffer.pop_front() {
                if let Some(&strand_idx) = self.receptors.get(&key) {
                    // Interrupt!
                    // Push current IP to call stack so we can return later (if we want)
                    // Note: We push the *current* IP. If we want to return to the *next* instruction
                    // when we are interrupted between instructions, it depends.
                    // Here we are at start of step(), so IP points to the instruction *to be executed*.
                    // So when we return, we want to execute *that* instruction.
                    self.call_stack.push(self.ip);

                    if strand_idx < self.dna.helix.strands.len() {
                        self.ip = (strand_idx, 0);
                        self.output.push(format!(
                            "INTERRUPT: Signal '{}' -> Strand {}",
                            key, strand_idx
                        ));
                    }
                }
            }
        }

        #[cfg(feature = "cortex")]
        {
            for level in self.activation_levels.iter_mut() {
                if *level > 0 {
                    *level -= 1;
                }
            }
        }

        #[cfg(feature = "nova")]
        {
            // Metabolic Waste Production
            let (cy, cx) = self.context_loc;
            self.waste_grid[cy][cx] += 10;

            self.diffuse_hormones();
            self.diffuse_waste();

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

            // Toxicity check
            if self.waste_grid[cy][cx] > 100 {
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.05) {
                    self.output
                        .push(format!("MUTATION: TOXICITY at {},{}", cx, cy));
                    self.mutate();
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
        let (gene_op, gene_args) = {
            let gene = &self.dna.helix.strands[self.ip.0].genes[self.ip.1];
            (gene.op.clone(), gene.args.clone())
        };

        let jump_target = self.execute_gene(gene_op, &gene_args);

        if let Some(target) = jump_target {
            self.ip = target;
        } else {
            // Move to next gene
            self.ip.1 += 1;
        }
    }

    fn execute_gene(&mut self, op: OpCode, args: &[Nucleotide]) -> Option<(usize, usize)> {
        if self.recursion_depth > 100 {
            self.output
                .push("Error: Recursion limit exceeded".to_string());
            return None;
        }
        self.recursion_depth += 1;
        let result = self.execute_gene_inner(op, args);
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
        op: OpCode,
        args: &[Nucleotide],
    ) -> Option<(usize, usize)> {
        match op {
            OpCode::Push => {
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
            OpCode::Sporulate => {
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
                    waste_grid: self.waste_grid.clone(),
                    call_stack: self.call_stack.clone(),
                    input_buffer: self.input_buffer.clone(),
                    receptors: self.receptors.clone(),
                    #[cfg(feature = "cortex")]
                    synapse_map: self.synapse_map.clone(),
                    #[cfg(feature = "cortex")]
                    activation_levels: self.activation_levels.clone(),
                };

                let id = self.spores.len();
                self.spores.push(spore);
                self.stack.push(Value::Int(id as i64));
                self.energy -= 50; // High cost for time travel
                self.output.push(format!("SPORULATE: Created Spore {}", id));
                None
            }
            #[cfg(feature = "nova")]
            OpCode::Germinate => {
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
                            self.waste_grid = spore.waste_grid.clone();
                            self.call_stack = spore.call_stack.clone();
                            self.input_buffer = spore.input_buffer.clone();
                            self.receptors = spore.receptors.clone();
                            #[cfg(feature = "cortex")]
                            {
                                self.synapse_map = spore.synapse_map.clone();
                                self.activation_levels = spore.activation_levels.clone();
                            }

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
            OpCode::Incubate => {
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
                                                op: OpCode::Push,
                                                args: vec![crate::ast::Nucleotide::Number(*n)],
                                            });
                                        }
                                        Value::Str(s) => {
                                            let op =
                                                s.parse().unwrap_or(OpCode::Unknown(s.clone()));
                                            genes.push(crate::ast::Gene { op, args: vec![] });
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
                                #[cfg(feature = "cortex")]
                                {
                                    self.activation_levels.push(0);
                                    self.synapse_map.push(Vec::new());
                                }
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
            OpCode::Add => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a + b);
                None
            }
            OpCode::Sub => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a - b);
                None
            }
            OpCode::Mul => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a * b);
                None
            }
            OpCode::Div => {
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
            OpCode::Dup => {
                if let Some(val) = self.stack.last() {
                    self.stack.push(val.clone());
                }
                None
            }
            OpCode::Swap => {
                let len = self.stack.len();
                if len >= 2 {
                    self.stack.swap(len - 1, len - 2);
                } else {
                    self.output
                        .push("Error: Stack underflow for swap".to_string());
                }
                None
            }
            OpCode::Drop => {
                self.stack.pop();
                None
            }
            OpCode::Print => {
                if let Some(val) = self.stack.pop() {
                    self.output.push(format!("{}", val));
                }
                None
            }
            OpCode::Jump => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    Some((*n as usize, 0))
                } else {
                    self.output.push("Error: Invalid arg for jump".to_string());
                    None
                }
            }
            OpCode::Brz => {
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
            OpCode::Photosynthesize => {
                self.energy += 5;
                None
            }
            OpCode::Consume => {
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
            OpCode::GRead => {
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
            OpCode::GWrite => {
                if self.stack.len() >= 3 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let val = self.stack.pop().unwrap();
                    if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                        if (0..16).contains(&y) && (0..16).contains(&x) {
                            self.grid[y as usize][x as usize] = val;
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
            OpCode::Radiate => {
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
                            self.grid[cy][cx] = val.clone();
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
            OpCode::Siphon => {
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
                            self.grid[cy][cx] = Value::Int(0);
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
            OpCode::Genome => {
                // Pushes genes of current strand to stack
                if self.ip.0 < self.dna.helix.strands.len() {
                    let strand = &self.dna.helix.strands[self.ip.0];
                    self.stack.push(Value::Int(strand.genes.len() as i64));
                    for gene in &strand.genes {
                        self.stack.push(Value::Str(gene.op.to_string()));
                    }
                }
                None
            }
            OpCode::Virus => {
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
                                let op = s.parse().unwrap_or(OpCode::Unknown(s.clone()));
                                let result = self.execute_gene(op, &[]);
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
            OpCode::Transcribe => {
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
            OpCode::JumpS => {
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
            OpCode::BrzS => {
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
            OpCode::SLen => {
                self.stack.push(Value::Int(self.stack.len() as i64));
                None
            }
            OpCode::HelixLen => {
                self.stack
                    .push(Value::Int(self.dna.helix.strands.len() as i64));
                None
            }
            OpCode::GeneLen => {
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
            OpCode::Methylate => {
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
            OpCode::Demethylate => {
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
            OpCode::Telomerase => {
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
            OpCode::TLen => {
                let idx = self.ip.0;
                if idx < self.telomeres.len() {
                    self.stack.push(Value::Int(self.telomeres[idx]));
                } else {
                    self.stack.push(Value::Int(0));
                }
                None
            }
            #[cfg(feature = "nova")]
            OpCode::Recombine => {
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
            OpCode::SIndex => {
                self.stack.push(Value::Int(self.ip.0 as i64));
                None
            }
            #[cfg(feature = "nova")]
            OpCode::CrisprScan => {
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
                            // Use op.to_string() for comparison
                            let guide_names: Vec<String> = guide_strand
                                .genes
                                .iter()
                                .map(|g| g.op.to_string())
                                .collect();
                            let target_names: Vec<String> = target_strand
                                .genes
                                .iter()
                                .map(|g| g.op.to_string())
                                .collect();

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
            OpCode::Cas9Cut => {
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

                                #[cfg(feature = "cortex")]
                                {
                                    self.activation_levels.push(0);
                                    self.synapse_map.push(Vec::new());
                                }

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
            OpCode::Ligase => {
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
            OpCode::Mitosis => {
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

                                #[cfg(feature = "cortex")]
                                {
                                    self.activation_levels.push(0);
                                    self.synapse_map.push(Vec::new());
                                }

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
            OpCode::Apoptosis => {
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
            OpCode::Integrase => {
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
                                        op: name.parse().unwrap_or(OpCode::Unknown(name.clone())),
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
            OpCode::Excision => {
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
            OpCode::Secrete => {
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
            OpCode::Detect => {
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
            OpCode::Absorb => {
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
            OpCode::Migrate => {
                // stack: dy, dx (top)
                if self.stack.len() >= 2 {
                    let dx_val = self.stack.pop().unwrap();
                    let dy_val = self.stack.pop().unwrap();
                    if let (Value::Int(dy), Value::Int(dx)) = (dy_val, dx_val) {
                        let (cy, cx) = self.context_loc;
                        let new_y = (cy as i64 + dy).rem_euclid(16) as usize;
                        let new_x = (cx as i64 + dx).rem_euclid(16) as usize;
                        self.context_loc = (new_y, new_x);
                        self.energy -= 5;
                        self.output
                            .push(format!("MIGRATE: moved to {},{}", new_x, new_y));
                    } else {
                        self.output
                            .push("Error: Type mismatch for migrate".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for migrate".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            OpCode::Detox => {
                // stack: radius (top)
                if let Some(val) = self.stack.pop() {
                    if let Value::Int(r) = val {
                        let (cy, cx) = self.context_loc;
                        let coords = self.get_circular_coords(cx as i64, cy as i64, r);
                        for (tx, ty) in coords {
                            self.waste_grid[ty][tx] = 0;
                        }
                        self.energy -= (r * r + 1).clamp(5, 50); // Cost proportional to area
                        self.output
                            .push(format!("DETOX: Cleansed radius {} at {},{}", r, cx, cy));
                    } else {
                        self.output
                            .push("Error: Type mismatch for detox".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for detox".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            OpCode::WRead => {
                let (cy, cx) = self.context_loc;
                let waste = self.waste_grid[cy][cx];
                self.stack.push(Value::Int(waste));
                None
            }
            #[cfg(feature = "nova")]
            OpCode::Call => {
                if let Some(Nucleotide::Number(idx)) = args.first() {
                    let strand_idx = *idx as usize;
                    if strand_idx < self.dna.helix.strands.len() {
                        // Push return address (current strand, next gene)
                        // ip points to Call instruction. Step loop will increment it.
                        // But if we jump, step loop sets ip to target.
                        // So we need to push (ip.0, ip.1 + 1).
                        self.call_stack.push((self.ip.0, self.ip.1 + 1));
                        return Some((strand_idx, 0));
                    } else {
                        self.output
                            .push("Error: Invalid strand index for call".to_string());
                    }
                } else {
                    self.output.push("Error: Invalid arg for call".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            OpCode::Ret => {
                if let Some(ret_addr) = self.call_stack.pop() {
                    return Some(ret_addr);
                } else {
                    self.output
                        .push("Warning: Return with empty stack".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            OpCode::Bind => {
                // stack: strand_idx (top), char_code
                if self.stack.len() >= 2 {
                    let s_val = self.stack.pop().unwrap();
                    let c_val = self.stack.pop().unwrap();
                    if let (Value::Int(s), Value::Int(c)) = (s_val, c_val) {
                        let strand_idx = s as usize;
                        let key = (c as u8) as char;
                        if strand_idx < self.dna.helix.strands.len() {
                            self.receptors.insert(key, strand_idx);
                            self.output
                                .push(format!("BIND: '{}' -> Strand {}", key, strand_idx));
                        } else {
                            self.output
                                .push("Error: Invalid strand index for bind".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for bind".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for bind".to_string());
                }
                None
            }
            #[cfg(feature = "nova")]
            OpCode::Unbind => {
                if let Some(val) = self.stack.pop() {
                    if let Value::Int(c) = val {
                        let key = (c as u8) as char;
                        self.receptors.remove(&key);
                        self.output.push(format!("UNBIND: '{}'", key));
                    } else {
                        self.output
                            .push("Error: Type mismatch for unbind".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for unbind".to_string());
                }
                None
            }
            #[cfg(feature = "cortex")]
            OpCode::Link => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(target) => {
                            let target_idx = target as usize;
                            let s_idx = self.ip.0;
                            // Check bounds using activation_levels as proxy for strand count
                            if target_idx < self.activation_levels.len()
                                && s_idx < self.synapse_map.len()
                            {
                                if !self.synapse_map[s_idx].contains(&target_idx) {
                                    self.synapse_map[s_idx].push(target_idx);
                                    self.output
                                        .push(format!("LINK: {} -> {}", s_idx, target_idx));
                                }
                            } else {
                                self.output
                                    .push("Error: Invalid strand index for link".to_string());
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for link".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for link".to_string());
                }
                None
            }
            #[cfg(feature = "cortex")]
            OpCode::Sever => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(target) => {
                            let target_idx = target as usize;
                            let s_idx = self.ip.0;
                            if s_idx < self.synapse_map.len() {
                                if let Some(pos) = self.synapse_map[s_idx]
                                    .iter()
                                    .position(|&x| x == target_idx)
                                {
                                    self.synapse_map[s_idx].remove(pos);
                                    self.output
                                        .push(format!("SEVER: {} -x {}", s_idx, target_idx));
                                }
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for sever".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for sever".to_string());
                }
                None
            }
            #[cfg(feature = "cortex")]
            OpCode::Spark => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(amount) => {
                            let s_idx = self.ip.0;
                            if s_idx < self.synapse_map.len() {
                                let targets = self.synapse_map[s_idx].clone();
                                let count = targets.len();
                                for target_idx in targets {
                                    if target_idx < self.activation_levels.len() {
                                        self.activation_levels[target_idx] += amount;
                                    }
                                }
                                self.energy -= (count as i64) + 1;
                                self.output
                                    .push(format!("SPARK: Fired {} to {} targets", amount, count));
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for spark".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for spark".to_string());
                }
                None
            }
            #[cfg(feature = "cortex")]
            OpCode::Sense => {
                let s_idx = self.ip.0;
                if s_idx < self.activation_levels.len() {
                    let level = self.activation_levels[s_idx];
                    self.stack.push(Value::Int(level));
                } else {
                    self.stack.push(Value::Int(0));
                }
                None
            }
            #[cfg(feature = "cortex")]
            OpCode::Gate => {
                if let Some(Nucleotide::Number(threshold)) = args.first() {
                    let s_idx = self.ip.0;
                    if s_idx < self.activation_levels.len() {
                        if self.activation_levels[s_idx] < *threshold {
                            // Skip next instruction
                            // VM increments ip.1 by 1 by default after execute_gene returns None.
                            // So we need to increment it by 1 here to make it skip one more.
                            self.ip.1 += 1;
                        }
                    }
                } else {
                    self.output.push("Error: Invalid arg for gate".to_string());
                }
                None
            }

            OpCode::Unknown(name) => {
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
                OpCode::Push,
                OpCode::Add,
                OpCode::Sub,
                OpCode::Mul,
                OpCode::Div,
                OpCode::Dup,
                OpCode::Print,
                OpCode::Swap,
                OpCode::Drop,
                OpCode::Jump,
                OpCode::Brz,
                OpCode::Photosynthesize,
                OpCode::Consume,
                OpCode::Transcribe,
                OpCode::SLen,
                OpCode::HelixLen,
                OpCode::GeneLen,
                OpCode::GRead,
                OpCode::GWrite,
                OpCode::Radiate,
                OpCode::Siphon,
                OpCode::Genome,
                #[cfg(feature = "nova")]
                OpCode::Telomerase,
                #[cfg(feature = "nova")]
                OpCode::TLen,
                #[cfg(feature = "nova")]
                OpCode::SIndex,
                #[cfg(feature = "nova")]
                OpCode::Mitosis,
                #[cfg(feature = "nova")]
                OpCode::Apoptosis,
                #[cfg(feature = "nova")]
                OpCode::CrisprScan,
                #[cfg(feature = "nova")]
                OpCode::Cas9Cut,
                #[cfg(feature = "nova")]
                OpCode::Ligase,
            ];
            let new_op = enzymes[rng.gen_range(0..enzymes.len())].clone();
            // Add "Mutation" log
            self.output
                .push(format!("MUTATION: {} -> {}", gene.op, new_op));
            gene.op = new_op;
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
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
            },
            Gene {
                op: OpCode::Add,
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
                    op: OpCode::Jump,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(100)],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
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
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::Brz,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(100)],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
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
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                }, // 0: strand idx
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(5)],
                }, // 1: gene idx
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                }, // 2: arg idx
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(99)],
                }, // 3: value
                Gene {
                    op: OpCode::Transcribe,
                    args: vec![],
                }, // 4: transcribe
                Gene {
                    op: OpCode::Push,
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
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Div,
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
                op: OpCode::Push,
                args: vec![Nucleotide::Number(i64::MIN)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(-1)],
            },
            Gene {
                op: OpCode::Div,
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
            op: OpCode::Add, // Requires 2 args, stack has 0
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
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("foo".to_string())],
            },
            Gene {
                op: OpCode::Add,
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
            op: OpCode::Jump,
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
                op: OpCode::Photosynthesize,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
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
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Consume,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
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
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // x
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // x
            Gene {
                op: OpCode::GRead,
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
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Virus,
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
                op: OpCode::Push,
                args: vec![Nucleotide::String("add".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(6)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(6)],
            },
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(6)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(6)],
            },
            Gene {
                op: OpCode::Virus,
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
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::JumpS,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(100)],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
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
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Radiate,
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
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Radiate,
                args: vec![],
            },
            // Siphon same area
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Siphon,
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
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Genome,
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
