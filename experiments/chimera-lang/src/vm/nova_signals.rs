#![cfg(feature = "nova")]

use super::{ChimeraVM, Value, GRID_SIZE};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use rand::Rng;

fn char_to_val(c: char) -> Option<i64> {
    match c {
        '0'..='9' => Some(c as i64 - '0' as i64),
        'a'..='z' => Some(c as i64 - 'a' as i64 + 10),
        'A'..='Z' => Some(c as i64 - 'A' as i64 + 10),
        _ => None,
    }
}

fn val_to_char(v: i64) -> char {
    let v = v.rem_euclid(36);
    if v < 10 {
        ((v as u8) + b'0') as char
    } else {
        ((v as u8 - 10) + b'a') as char
    }
}

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

struct GridWrite {
    y: usize,
    x: usize,
    val: Value,
}

struct DnaWrite {
    strand_idx: usize,
    gene_idx: usize,
    val: Nucleotide,
}

struct DnaAppend {
    strand_idx: usize,
    gene: crate::ast::Gene,
}

struct ResonanceWrite {
    y: usize,
    x: usize,
    freq: f32,
    amp: f32,
}

struct MutationRequest {
    strand_idx: usize,
}

struct SignalContext {
    next_signals: Vec<Vec<u8>>,
    grid_writes: Vec<GridWrite>,
    dna_writes: Vec<DnaWrite>,
    dna_appends: Vec<DnaAppend>,
    resonance_writes: Vec<ResonanceWrite>,
    mutation_requests: Vec<MutationRequest>,
    executions: Vec<(OpCode, Vec<Nucleotide>)>,
}

pub fn process_signals(vm: &mut ChimeraVM) {
    let size = GRID_SIZE;
    let mut ctx = SignalContext {
        next_signals: vec![vec![0u8; size]; size],
        grid_writes: Vec::new(),
        dna_writes: Vec::new(),
        dna_appends: Vec::new(),
        resonance_writes: Vec::new(),
        mutation_requests: Vec::new(),
        executions: Vec::new(),
    };

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
                            ctx.next_signals[ny][nx] = ctx.next_signals[ny][nx].saturating_add(1);
                        }
                    }
                }
                '>' => propagate_directional(vm, y, x, 0, 1, 1, &mut ctx.next_signals),
                '<' => propagate_directional(vm, y, x, 0, -1, 1, &mut ctx.next_signals),
                '^' => propagate_directional(vm, y, x, -1, 0, 1, &mut ctx.next_signals),
                'v' => propagate_directional(vm, y, x, 1, 0, 1, &mut ctx.next_signals),
                '+' => {
                    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                            ctx.next_signals[ny][nx] =
                                ctx.next_signals[ny][nx].saturating_add(signal);
                        }
                    }
                }
                'N' => read_write_directional(vm, y, x, -1, 0, 1, 0, &mut ctx),
                'S' => read_write_directional(vm, y, x, 1, 0, -1, 0, &mut ctx),
                'E' => read_write_directional(vm, y, x, 0, 1, 0, -1, &mut ctx),
                'W' => read_write_directional(vm, y, x, 0, -1, 0, 1, &mut ctx),
                'A' | 'a' => binary_op(vm, y, x, &mut ctx.grid_writes, |a, b| a.wrapping_add(b)),
                'B' | 'b' => binary_op(vm, y, x, &mut ctx.grid_writes, |a, b| a.wrapping_sub(b)),
                'D' | 'd' => binary_op(vm, y, x, &mut ctx.grid_writes, |a, b| {
                    if b != 0 {
                        a.wrapping_div(b)
                    } else {
                        0
                    }
                }),
                'M' | 'm' => exec_mutate(vm, y, x, signal, &mut ctx),
                'T' | 't' => exec_teleport(vm, y, x, &mut ctx),
                'L' | 'l' => exec_laser(vm, y, x, &mut ctx),
                'Z' | 'z' => exec_resonate(vm, y, x, signal, &mut ctx),
                'I' | 'i' => exec_increment(vm, y, x, &mut ctx),
                'R' | 'r' => exec_random(vm, y, x, &mut ctx),
                'C' | 'c' => exec_clock(vm, y, x, &mut ctx),
                'X' | 'x' => exec_write(vm, y, x, &mut ctx),
                'O' | 'o' => exec_offset(vm, y, x, &mut ctx),
                'G' | 'g' => exec_gene_read(vm, y, x, &mut ctx),
                'P' | 'p' => exec_play(vm, y, x, signal, &mut ctx),
                'K' | 'k' => exec_kill(vm, y, x, signal, &mut ctx),
                'Y' | 'y' => exec_synthesize(vm, y, x, signal, &mut ctx),
                'Q' | 'q' => exec_query(vm, y, x, &mut ctx),
                'H' | 'h' => exec_harvest(vm, y, x, signal, &mut ctx),
                _ => {
                    if let Value::Str(s) = val {
                        if let Ok(op) = s.parse::<OpCode>() {
                            if signal > 0 {
                                ctx.executions.push((op, vec![]));
                            }
                        }
                    }
                }
            }
        }
    }

    // 2. Apply Writes
    for w in ctx.grid_writes {
        vm.grid[w.y][w.x] = w.val;
    }

    // 2.5 Apply DNA Writes
    for w in ctx.dna_writes {
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

    for w in ctx.dna_appends {
        if w.strand_idx < vm.dna.helix.strands.len() {
            vm.dna.helix.strands[w.strand_idx].genes.push(w.gene);
        }
    }

    // 3. Update Signal State
    vm.signal_grid = ctx.next_signals;

    // 3.5 Apply Resonance & Mutations
    for w in ctx.resonance_writes {
        vm.resonance_grid[w.y][w.x] = (w.freq, w.amp);
    }

    for req in ctx.mutation_requests {
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
    for (op, args) in ctx.executions {
        vm.execute_gene_inner(op.clone(), &args);
    }
}

fn exec_mutate(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if let Some(s_idx) = peek(vm, y, x, 0, -1) {
        if signal > 0 {
            ctx.mutation_requests.push(MutationRequest {
                strand_idx: s_idx as usize,
            });
        }
    }
}

fn exec_harvest(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if signal == 0 {
        return;
    }

    // Inputs: West (Strand), East (Length), North (Offset Y)
    let s_idx = peek(vm, y, x, 0, -1);
    let len = peek(vm, y, x, 0, 1);
    let off_y = peek(vm, y, x, -1, 0).unwrap_or(1); // Default offset 1

    if let (Some(s), Some(l)) = (s_idx, len) {
        let mut op_str = String::new();
        // Read l chars starting from (y + off_y, x)
        for i in 0..l {
            if let Some(val) = peek(vm, y, x, off_y, i) {
                op_str.push(val_to_char(val));
            } else {
                op_str.push(' ');
            }
        }

        // Trim
        let clean_op = op_str.trim();
        if let Ok(op) = clean_op.parse::<OpCode>() {
            ctx.dna_appends.push(DnaAppend {
                strand_idx: s as usize,
                gene: crate::ast::Gene {
                    op,
                    args: vec![], // No args support yet
                },
            });

            // Success Output
            if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                ctx.grid_writes.push(GridWrite {
                    y: sy,
                    x: sx,
                    val: Value::Str("1".to_string()),
                });
            }
        } else {
            // Failure Output
            if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                ctx.grid_writes.push(GridWrite {
                    y: sy,
                    x: sx,
                    val: Value::Str("0".to_string()),
                });
            }
        }
    }
}

fn exec_query(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    if let (Some(dir_code), Some(target_val)) = (peek(vm, y, x, -1, 0), peek(vm, y, x, 0, 1)) {
        let (dy, dx) = match dir_code % 4 {
            0 => (-1, 0), // N
            1 => (0, 1),  // E
            2 => (1, 0),  // S
            3 => (0, -1), // W
            _ => (0, 0),
        };

        let target_char = val_to_char(target_val);
        let actual_val = peek(vm, y, x, dy, dx);

        let is_match = if let Some(val) = actual_val {
            val_to_char(val) == target_char
        } else {
            false
        };

        let res = if is_match { 1 } else { 0 };

        if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
            ctx.grid_writes.push(GridWrite {
                y: sy,
                x: sx,
                val: Value::Str(val_to_char(res).to_string()),
            });
        }
    }
}

fn exec_teleport(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    if let (Some(x_val), Some(y_val), Some(val)) = (
        peek(vm, y, x, 0, -1),
        peek(vm, y, x, 0, 1),
        peek(vm, y, x, -1, 0),
    ) {
        if let Some((ty, tx)) = vm.normalize_coords(y_val, x_val) {
            ctx.grid_writes.push(GridWrite {
                y: ty,
                x: tx,
                val: Value::Str(val_to_char(val).to_string()),
            });
        }
    }
}

fn exec_laser(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    if let (Some(len), Some(dir)) = (peek(vm, y, x, 0, -1), peek(vm, y, x, -1, 0)) {
        let (dy, dx) = match dir % 4 {
            0 => (-1, 0),
            1 => (0, 1),
            2 => (1, 0),
            3 => (0, -1),
            _ => (0, 0),
        };
        for i in 1..=len.min(16) {
            if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy * i, x as i64 + dx * i) {
                ctx.next_signals[ny][nx] = ctx.next_signals[ny][nx].saturating_add(1);
            }
        }
    }
}

fn exec_resonate(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if let (Some(freq), Some(amp)) = (peek(vm, y, x, 0, -1), peek(vm, y, x, 0, 1)) {
        if signal > 0 {
            ctx.resonance_writes.push(ResonanceWrite {
                y,
                x,
                freq: freq as f32 * 10.0,
                amp: amp as f32 * 2.0,
            });
        }
    }
}

fn exec_increment(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    if let Some(n) = peek(vm, y, x, -1, 0) {
        let max = peek(vm, y, x, 0, 1).unwrap_or(35);
        let res = if n >= max { 0 } else { n + 1 };
        if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
            ctx.grid_writes.push(GridWrite {
                y: sy,
                x: sx,
                val: Value::Str(val_to_char(res).to_string()),
            });
        }
    }
}

fn exec_random(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    let min = peek(vm, y, x, -1, 0).unwrap_or(0);
    let max = peek(vm, y, x, 0, 1).unwrap_or(35);
    let mut rng = rand::thread_rng();
    let range_min = min.min(max);
    let range_max = min.max(max);
    let res = rng.gen_range(range_min..=range_max);
    if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
        ctx.grid_writes.push(GridWrite {
            y: sy,
            x: sx,
            val: Value::Str(val_to_char(res).to_string()),
        });
    }
}

fn exec_clock(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    let rate = peek(vm, y, x, 0, 1).unwrap_or(1).max(1);
    let mod_val = peek(vm, y, x, -1, 0).unwrap_or(8).max(1);
    let res = (vm.tick_counter as i64 / rate) % mod_val;
    if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
        ctx.grid_writes.push(GridWrite {
            y: sy,
            x: sx,
            val: Value::Str(val_to_char(res).to_string()),
        });
    }
}

fn exec_write(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    if let (Some(val), Some(x_off), Some(y_off)) = (
        peek(vm, y, x, -1, 0),
        peek(vm, y, x, 0, -1),
        peek(vm, y, x, 0, 1),
    ) {
        if let Some((ty, tx)) = vm.normalize_coords(y as i64 + y_off, x as i64 + x_off) {
            ctx.grid_writes.push(GridWrite {
                y: ty,
                x: tx,
                val: Value::Str(val_to_char(val).to_string()),
            });
        }
    }
}

fn exec_offset(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    if let (Some(x_off), Some(y_off)) = (peek(vm, y, x, 0, -1), peek(vm, y, x, 0, 1)) {
        if let Some(val) = peek(vm, y, x, y_off, x_off) {
            if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                ctx.grid_writes.push(GridWrite {
                    y: sy,
                    x: sx,
                    val: Value::Str(val_to_char(val).to_string()),
                });
            }
        }
    }
}

fn exec_gene_read(vm: &ChimeraVM, y: usize, x: usize, ctx: &mut SignalContext) {
    if let (Some(s_idx), Some(g_idx)) = (peek(vm, y, x, 0, -1), peek(vm, y, x, 0, 1)) {
        let s = s_idx as usize;
        let g = g_idx as usize;
        if s < vm.dna.helix.strands.len() {
            let strand = &vm.dna.helix.strands[s];
            if g < strand.genes.len() {
                let op_str = strand.genes[g].op.to_string();
                if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                    ctx.grid_writes.push(GridWrite {
                        y: sy,
                        x: sx,
                        val: Value::Str(op_str),
                    });
                }
            }
        }
    }
}

fn exec_play(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if let Some(s_idx) = peek(vm, y, x, 0, -1) {
        if signal > 0 {
            ctx.executions
                .push((OpCode::Call, vec![Nucleotide::Number(s_idx)]));
        }
    }
}

fn exec_kill(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if let Some(s_idx) = peek(vm, y, x, 0, -1) {
        if signal > 0 {
            ctx.executions
                .push((OpCode::Push, vec![Nucleotide::Number(s_idx)]));
            ctx.executions.push((OpCode::Apoptosis, vec![]));
        }
    }
}

fn exec_synthesize(vm: &ChimeraVM, y: usize, x: usize, signal: u8, ctx: &mut SignalContext) {
    if let (Some(s_idx), Some(g_idx), Some(val)) = (
        peek(vm, y, x, 0, -1),
        peek(vm, y, x, 0, 1),
        peek(vm, y, x, -1, 0),
    ) {
        if signal > 0 {
            ctx.dna_writes.push(DnaWrite {
                strand_idx: s_idx as usize,
                gene_idx: g_idx as usize,
                val: Nucleotide::Number(val),
            });
        }
    }
}

fn propagate_directional(
    vm: &ChimeraVM,
    y: usize,
    x: usize,
    dy: i64,
    dx: i64,
    signal: u8,
    next_signals: &mut [Vec<u8>],
) {
    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
        next_signals[ny][nx] = next_signals[ny][nx].saturating_add(signal);
    }
}

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

fn read_write_directional(
    vm: &ChimeraVM,
    y: usize,
    x: usize,
    read_dy: i64,
    read_dx: i64,
    write_dy: i64,
    write_dx: i64,
    ctx: &mut SignalContext,
) {
    if let Some(val) = peek(vm, y, x, read_dy, read_dx) {
        if let Some((wy, wx)) = vm.normalize_coords(y as i64 + write_dy, x as i64 + write_dx) {
            ctx.grid_writes.push(GridWrite {
                y: wy,
                x: wx,
                val: Value::Str(val_to_char(val).to_string()),
            });
        }
    }
}
