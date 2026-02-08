#[cfg(feature = "nova")]
use super::{ChimeraVM, Value, GRID_SIZE};
#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use rand::Rng;

#[cfg(feature = "nova")]
fn char_to_val(c: char) -> Option<i64> {
    match c {
        '0'..='9' => Some(c as i64 - '0' as i64),
        'a'..='z' => Some(c as i64 - 'a' as i64 + 10),
        'A'..='Z' => Some(c as i64 - 'A' as i64 + 10),
        _ => None,
    }
}

#[cfg(feature = "nova")]
fn val_to_char(v: i64) -> char {
    let v = v.rem_euclid(36);
    if v < 10 {
        ((v as u8) + b'0') as char
    } else {
        ((v as u8 - 10) + b'a') as char
    }
}

#[cfg(feature = "nova")]
fn peek(vm: &ChimeraVM, y: usize, x: usize, dy: i64, dx: i64) -> Option<i64> {
    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
        match &vm.grid[ny][nx] {
            Value::Int(n) => Some(*n),
            Value::Str(s) => {
                if s.len() == 1 {
                    char_to_val(s.chars().next().unwrap())
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    }
}

#[cfg(feature = "nova")]
struct GridWrite {
    y: usize,
    x: usize,
    val: Value,
}

#[cfg(feature = "nova")]
struct DnaWrite {
    strand_idx: usize,
    gene_idx: usize,
    val: Nucleotide,
}

#[cfg(feature = "nova")]
struct ResonanceWrite {
    y: usize,
    x: usize,
    freq: f32,
    amp: f32,
}

#[cfg(feature = "nova")]
struct MutationRequest {
    strand_idx: usize,
}

#[cfg(feature = "nova")]
pub fn process_signals(vm: &mut ChimeraVM) {
    let size = GRID_SIZE;
    let mut next_signals = vec![vec![0u8; size]; size];
    let mut grid_writes: Vec<GridWrite> = Vec::new();
    let mut dna_writes: Vec<DnaWrite> = Vec::new();
    let mut resonance_writes: Vec<ResonanceWrite> = Vec::new();
    let mut mutation_requests: Vec<MutationRequest> = Vec::new();
    // Executions now store (OpCode, Args)
    let mut executions: Vec<(OpCode, Vec<Nucleotide>)> = Vec::new();

    // 1. Scan Phase
    for y in 0..size {
        for x in 0..size {
            let signal = vm.signal_grid[y][x];

            let val = &vm.grid[y][x];
            let c = match val {
                Value::Str(s) if s.len() == 1 => s.chars().next().unwrap(),
                Value::Str(s) => match s.as_str() {
                    "*" | ">" | "<" | "^" | "v" | "+" => s.chars().next().unwrap(),
                    _ => '\0',
                },
                _ => '\0',
            };

            let is_uppercase = c.is_ascii_uppercase();
            let is_bang = c == '*';
            let active = signal > 0 || is_uppercase || is_bang;

            if !active {
                continue;
            }

            match c {
                '*' => {
                    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                            next_signals[ny][nx] = next_signals[ny][nx].saturating_add(1);
                        }
                    }
                }
                '>' => propagate_directional(vm, y, x, 0, 1, 1, &mut next_signals),
                '<' => propagate_directional(vm, y, x, 0, -1, 1, &mut next_signals),
                '^' => propagate_directional(vm, y, x, -1, 0, 1, &mut next_signals),
                'v' => propagate_directional(vm, y, x, 1, 0, 1, &mut next_signals),
                '+' => {
                    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                            next_signals[ny][nx] = next_signals[ny][nx].saturating_add(signal);
                        }
                    }
                }
                'N' => read_write_directional(vm, y, x, -1, 0, 1, 0, &mut grid_writes),
                'S' => read_write_directional(vm, y, x, 1, 0, -1, 0, &mut grid_writes),
                'E' => read_write_directional(vm, y, x, 0, 1, 0, -1, &mut grid_writes),
                'W' => read_write_directional(vm, y, x, 0, -1, 0, 1, &mut grid_writes),
                'A' | 'a' => binary_op(vm, y, x, &mut grid_writes, |a, b| a.wrapping_add(b)),
                'B' | 'b' => binary_op(vm, y, x, &mut grid_writes, |a, b| a.wrapping_sub(b)),
                'D' | 'd' => binary_op(vm, y, x, &mut grid_writes, |a, b| {
                    if b != 0 {
                        a.wrapping_div(b)
                    } else {
                        0
                    }
                }),
                'M' | 'm' => {
                    // Mutate: West (Strand)
                    if let Some(s_idx) = peek(vm, y, x, 0, -1) {
                        if signal > 0 {
                            mutation_requests.push(MutationRequest {
                                strand_idx: s_idx as usize,
                            });
                        }
                    }
                }
                'T' | 't' => {
                    // Teleport: West(X), East(Y), North(Val) -> Write Val to (Y,X)
                    if let (Some(x_val), Some(y_val), Some(val)) = (
                        peek(vm, y, x, 0, -1),
                        peek(vm, y, x, 0, 1),
                        peek(vm, y, x, -1, 0),
                    ) {
                        if let Some((ty, tx)) = vm.normalize_coords(y_val, x_val) {
                            grid_writes.push(GridWrite {
                                y: ty,
                                x: tx,
                                val: Value::Str(val_to_char(val).to_string()),
                            });
                        }
                    }
                }
                'L' | 'l' => {
                    // Laser: West(Len), North(Dir)
                    // Dir: 0=N, 1=E, 2=S, 3=W (clock-wise)
                    if let (Some(len), Some(dir)) = (peek(vm, y, x, 0, -1), peek(vm, y, x, -1, 0)) {
                        let (dy, dx) = match dir % 4 {
                            0 => (-1, 0),
                            1 => (0, 1),
                            2 => (1, 0),
                            3 => (0, -1),
                            _ => (0, 0),
                        };
                        // Beam length up to 16
                        for i in 1..=len.min(16) {
                            if let Some((ny, nx)) =
                                vm.normalize_coords(y as i64 + dy * i, x as i64 + dx * i)
                            {
                                next_signals[ny][nx] = next_signals[ny][nx].saturating_add(1);
                            }
                        }
                    }
                }
                'Z' | 'z' => {
                    // Resonate: West(Freq), East(Amp)
                    if let (Some(freq), Some(amp)) = (peek(vm, y, x, 0, -1), peek(vm, y, x, 0, 1)) {
                        if signal > 0 {
                            resonance_writes.push(ResonanceWrite {
                                y,
                                x,
                                freq: freq as f32 * 10.0,
                                amp: amp as f32 * 2.0,
                            });
                        }
                    }
                }
                'I' | 'i' => {
                    if let Some(n) = peek(vm, y, x, -1, 0) {
                        let max = peek(vm, y, x, 0, 1).unwrap_or(35);
                        let res = if n >= max { 0 } else { n + 1 };
                        if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                            grid_writes.push(GridWrite {
                                y: sy,
                                x: sx,
                                val: Value::Str(val_to_char(res).to_string()),
                            });
                        }
                    }
                }
                'R' | 'r' => {
                    let min = peek(vm, y, x, -1, 0).unwrap_or(0);
                    let max = peek(vm, y, x, 0, 1).unwrap_or(35);
                    let mut rng = rand::thread_rng();
                    let range_min = min.min(max);
                    let range_max = min.max(max);
                    let res = rng.gen_range(range_min..=range_max);
                    if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                        grid_writes.push(GridWrite {
                            y: sy,
                            x: sx,
                            val: Value::Str(val_to_char(res).to_string()),
                        });
                    }
                }
                'C' | 'c' => {
                    let rate = peek(vm, y, x, 0, 1).unwrap_or(1).max(1);
                    let mod_val = peek(vm, y, x, -1, 0).unwrap_or(8).max(1);
                    let res = (vm.tick_counter as i64 / rate) % mod_val;
                    if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                        grid_writes.push(GridWrite {
                            y: sy,
                            x: sx,
                            val: Value::Str(val_to_char(res).to_string()),
                        });
                    }
                }
                'X' | 'x' => {
                    // Write: North (val) -> (West (x), East (y))
                    if let (Some(val), Some(x_off), Some(y_off)) = (
                        peek(vm, y, x, -1, 0),
                        peek(vm, y, x, 0, -1),
                        peek(vm, y, x, 0, 1),
                    ) {
                        if let Some((ty, tx)) =
                            vm.normalize_coords(y as i64 + y_off, x as i64 + x_off)
                        {
                            grid_writes.push(GridWrite {
                                y: ty,
                                x: tx,
                                val: Value::Str(val_to_char(val).to_string()),
                            });
                        }
                    }
                }
                'O' | 'o' => {
                    // Offset: Read (West (x), East (y)) -> South
                    if let (Some(x_off), Some(y_off)) =
                        (peek(vm, y, x, 0, -1), peek(vm, y, x, 0, 1))
                    {
                        if let Some(val) = peek(vm, y, x, y_off, x_off) {
                            if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                                grid_writes.push(GridWrite {
                                    y: sy,
                                    x: sx,
                                    val: Value::Str(val_to_char(val).to_string()),
                                });
                            }
                        }
                    }
                }
                'G' | 'g' => {
                    // Gene Read: West (Strand), East (Gene) -> South (OpCode)
                    if let (Some(s_idx), Some(g_idx)) = (peek(vm, y, x, 0, -1), peek(vm, y, x, 0, 1)) {
                        let s = s_idx as usize;
                        let g = g_idx as usize;
                        if s < vm.dna.helix.strands.len() {
                            let strand = &vm.dna.helix.strands[s];
                            if g < strand.genes.len() {
                                let op_str = strand.genes[g].op.to_string();
                                if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                                    grid_writes.push(GridWrite {
                                        y: sy,
                                        x: sx,
                                        val: Value::Str(op_str),
                                    });
                                }
                            }
                        }
                    }
                }
                'P' | 'p' => {
                    // Play: West (Strand) -> Execute Call(Strand)
                    if let Some(s_idx) = peek(vm, y, x, 0, -1) {
                        if signal > 0 {
                            executions.push((OpCode::Call, vec![Nucleotide::Number(s_idx)]));
                        }
                    }
                }
                'K' | 'k' => {
                    // Kill: West (Strand) -> Execute Push(Strand), Apoptosis
                    if let Some(s_idx) = peek(vm, y, x, 0, -1) {
                         if signal > 0 {
                             executions.push((OpCode::Push, vec![Nucleotide::Number(s_idx)]));
                             executions.push((OpCode::Apoptosis, vec![]));
                         }
                    }
                }
                'Y' | 'y' => {
                    // Synthesize: West (Strand), East (Gene), North (Value) -> DNA
                    if let (Some(s_idx), Some(g_idx), Some(val)) = (
                        peek(vm, y, x, 0, -1),
                        peek(vm, y, x, 0, 1),
                        peek(vm, y, x, -1, 0)
                    ) {
                        if signal > 0 {
                             dna_writes.push(DnaWrite {
                                 strand_idx: s_idx as usize,
                                 gene_idx: g_idx as usize,
                                 val: Nucleotide::Number(val)
                             });
                        }
                    }
                }
                _ => {
                    if let Value::Str(s) = val {
                        if let Ok(op) = s.parse::<OpCode>() {
                            if signal > 0 {
                                executions.push((op, vec![]));
                            }
                        }
                    }
                }
            }
        }
    }

    // 2. Apply Writes
    for w in grid_writes {
        vm.grid[w.y][w.x] = w.val;
    }

    // 2.5 Apply DNA Writes
    for w in dna_writes {
        if w.strand_idx < vm.dna.helix.strands.len() {
            let strand = &mut vm.dna.helix.strands[w.strand_idx];
            if w.gene_idx < strand.genes.len() {
                let gene = &mut strand.genes[w.gene_idx];
                if !gene.args.is_empty() {
                    gene.args[0] = w.val;
                } else {
                    gene.args.push(w.val);
                }
            }
        }
    }

    // 3. Update Signal State
    vm.signal_grid = next_signals;

    // 3.5 Apply Resonance & Mutations
    for w in resonance_writes {
        vm.resonance_grid[w.y][w.x] = (w.freq, w.amp);
    }

    for req in mutation_requests {
        if req.strand_idx < vm.dna.helix.strands.len() {
            let strand_len = vm.dna.helix.strands[req.strand_idx].genes.len();
            if strand_len > 0 {
                let mut rng = rand::thread_rng();
                let g_idx = rng.gen_range(0..strand_len);
                // Mutate Arg
                if !vm.dna.helix.strands[req.strand_idx].genes[g_idx]
                    .args
                    .is_empty()
                {
                    let val = rng.gen_range(0..100);
                    vm.dna.helix.strands[req.strand_idx].genes[g_idx].args[0] =
                        Nucleotide::Number(val);
                }
            }
        }
    }

    // 4. Execution Phase
    for (op, args) in executions {
        vm.execute_gene_inner(op.clone(), &args);
    }
}

#[cfg(feature = "nova")]
fn propagate_directional(
    vm: &ChimeraVM,
    y: usize,
    x: usize,
    dy: i64,
    dx: i64,
    signal: u8,
    next_signals: &mut Vec<Vec<u8>>,
) {
    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
        next_signals[ny][nx] = next_signals[ny][nx].saturating_add(signal);
    }
}

#[cfg(feature = "nova")]
fn binary_op<F>(vm: &ChimeraVM, y: usize, x: usize, grid_writes: &mut Vec<GridWrite>, op: F)
where
    F: Fn(i64, i64) -> i64,
{
    if let (Some(n), Some(e)) = (peek(vm, y, x, -1, 0), peek(vm, y, x, 0, 1)) {
        let res = op(n, e);
        if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
            grid_writes.push(GridWrite {
                y: sy,
                x: sx,
                val: Value::Str(val_to_char(res).to_string()),
            });
        }
    }
}

#[cfg(feature = "nova")]
fn read_write_directional(
    vm: &ChimeraVM,
    y: usize,
    x: usize,
    read_dy: i64,
    read_dx: i64,
    write_dy: i64,
    write_dx: i64,
    grid_writes: &mut Vec<GridWrite>,
) {
    if let Some(val) = peek(vm, y, x, read_dy, read_dx) {
        if let Some((wy, wx)) = vm.normalize_coords(y as i64 + write_dy, x as i64 + write_dx) {
            grid_writes.push(GridWrite {
                y: wy,
                x: wx,
                val: Value::Str(val_to_char(val).to_string()),
            });
        }
    }
}
