use crate::ast::{Gene, Nucleotide};
use crate::opcode::OpCode;

/// Maps a Gene (OpCode + Args) to a character representation for the Grid.
/// This acts as the "Transmutation" step from DNA to Protein (Grid Operator).
pub fn transmute(gene: &Gene) -> char {
    match gene.op {
        OpCode::Add => 'A',
        OpCode::Sub => 's',
        OpCode::Mul => 'm', // M is Mutate
        OpCode::Div => 'D',
        OpCode::GWrite => 'X',
        OpCode::GRead => 'o', // 'o' used in Ribosome for read
        OpCode::Jump => 'J',  // Conflicts with Jam, but acceptable for now
        OpCode::Print => 'P', // Conflicts with Play
        OpCode::Push => {
            // If pushing a number, return the digit/char
            if let Some(arg) = gene.args.first() {
                match arg {
                    Nucleotide::Number(n) => val_to_char(*n),
                    Nucleotide::String(s) => {
                        if s.len() == 1 {
                            s.chars().next().unwrap()
                        } else {
                            '"'
                        }
                    }
                    _ => '#',
                }
            } else {
                '#'
            }
        }
        OpCode::Nop => '.',
        // OpCode::Random => 'R', // Random is not explicitly in OpCode enum in recent read, checking...
        // Checked opcode.rs: OpCode::Random does NOT exist. HavocRate? Or R is Random in signals but not a standalone OpCode?
        // nova_signals.rs: 'R' | 'r' => exec_random. But does OpCode::Random exist?
        // Let's check opcode.rs again. I don't see Random.
        // nova_signals creates ctx.grid_writes.

        // I'll stick to safe ones.
        _ => '?',
    }
}

/// Maps a Grid character back to an OpCode (Reverse Engineering).
pub fn reverse_transmute(c: char) -> Option<OpCode> {
    match c {
        'A' => Some(OpCode::Add),
        's' => Some(OpCode::Sub),
        'm' => Some(OpCode::Mul),
        'D' => Some(OpCode::Div),
        'X' => Some(OpCode::GWrite),
        'o' => Some(OpCode::GRead),
        'J' => Some(OpCode::Jump),
        'P' => Some(OpCode::Print),
        '0'..='9' | 'a'..='z' => Some(OpCode::Push), // Push literal
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

/// Compiles a sequence of Genes into a 2D Grid layout.
/// Currently implements a simple linear layout wrapping at width 16.
pub fn compile_to_grid(genes: &Vec<Gene>) -> Vec<String> {
    let mut grid = Vec::new();
    let width = 16;
    let mut current_row = String::new();

    for gene in genes {
        let c = transmute(gene);
        current_row.push(c);

        if current_row.len() >= width {
            grid.push(current_row);
            current_row = String::new();
        }
    }

    if !current_row.is_empty() {
        grid.push(current_row);
    }

    // Pad to at least one row
    if grid.is_empty() {
        grid.push(String::new());
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Nucleotide;

    #[test]
    fn test_transmute_basic() {
        let g_add = Gene { op: OpCode::Add, args: vec![] };
        assert_eq!(transmute(&g_add), 'A');

        let g_push_5 = Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] };
        assert_eq!(transmute(&g_push_5), '5');

        let g_push_10 = Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] };
        assert_eq!(transmute(&g_push_10), 'a');
    }

    #[test]
    fn test_compile() {
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
            Gene { op: OpCode::Add, args: vec![] },
        ];
        let grid = compile_to_grid(&genes);
        assert_eq!(grid.len(), 1);
        assert_eq!(grid[0], "12A");
    }
}
