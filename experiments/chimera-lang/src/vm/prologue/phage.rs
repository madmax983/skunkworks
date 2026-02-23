use super::{normalize_coords, PrologueAgent};
use crate::ast::{Nucleotide};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;

pub fn process_phage_agent(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let current_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);

    // 1. Check neighbors for Strand Index (Integer) to mutate
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut rng = rand::thread_rng();

    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if let Value::Int(idx) = &grid_snapshot[ny][nx] {
                let s_idx = *idx as usize;
                // Only mutate if index is within bounds
                if s_idx < vm.dna.helix.strands.len() {
                    // Valid Strand Found -> Mutate
                    mutate_strand(vm, s_idx, &mut rng);
                    // Light up self to indicate activity
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                }
            }
        }
    }

    // 2. Move Logic: Random Walk
    use rand::seq::SliceRandom;
    let target_pos = neighbors
        .choose(&mut rng)
        .and_then(|(dy, dx)| normalize_coords(y as i64 + dy, x as i64 + dx))
        .and_then(|(ny, nx)| match &grid_snapshot[ny][nx] {
            Value::Int(0) => Some((ny, nx)),
            _ => None,
        });

    Some((current_agent, target_pos))
}

fn mutate_strand(vm: &mut ChimeraVM, s_idx: usize, rng: &mut impl Rng) {
    let strand = &mut vm.dna.helix.strands[s_idx];
    if strand.genes.is_empty() {
        return;
    }

    // Pick a random gene to mutate
    let g_idx = rng.gen_range(0..strand.genes.len());
    let gene = &mut strand.genes[g_idx];

    // Mutation Types:
    // 0: Flip OpCode
    // 1: Change Argument
    // 2: Shuffle Genes (Whole Strand)

    match rng.gen_range(0..3) {
        0 => {
            // Flip OpCode
            let ops = [
                OpCode::Add, OpCode::Sub, OpCode::Mul, OpCode::Div,
                OpCode::Push, OpCode::Dup, OpCode::Swap, OpCode::Drop,
                OpCode::Print, OpCode::Jump, OpCode::Brz,
                OpCode::Incubate, OpCode::Mitosis, OpCode::Apoptosis,
                OpCode::Photosynthesize, OpCode::Consume,
            ];
            let new_op = ops[rng.gen_range(0..ops.len())].clone();
            vm.output.push(format!("PHAGE: Mutated Strand {} Gene {} ({} -> {})", s_idx, g_idx, gene.op, new_op));
            gene.op = new_op;
        }
        1 => {
            // Change Argument
            if !gene.args.is_empty() {
                let a_idx = rng.gen_range(0..gene.args.len());
                let arg = &mut gene.args[a_idx];
                match arg {
                    Nucleotide::Number(n) => {
                        let old_n = *n;
                        *n = rng.gen_range(0..100);
                        vm.output.push(format!("PHAGE: Mutated Strand {} Gene {} Arg {} ({} -> {})", s_idx, g_idx, a_idx, old_n, n));
                    }
                    Nucleotide::String(s) => {
                         // Rotate string
                         if !s.is_empty() {
                             let c = s.remove(0);
                             s.push(c);
                             vm.output.push(format!("PHAGE: Mutated Strand {} Gene {} Arg {} (Rotated to '{}')", s_idx, g_idx, a_idx, s));
                         }
                    }
                    _ => {}
                }
            } else {
                // If no args, maybe add one? Or switch to Shuffle.
                vm.output.push(format!("PHAGE: No args to mutate in Strand {} Gene {}", s_idx, g_idx));
            }
        }
        2 => {
            // Shuffle Genes
            use rand::seq::SliceRandom;
            strand.genes.shuffle(rng);
            vm.output.push(format!("PHAGE: Shuffled Strand {}", s_idx));
        }
        _ => {}
    }
}
