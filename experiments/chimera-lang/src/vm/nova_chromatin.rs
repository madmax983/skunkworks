use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::vm::ChimeraVM;

/// Applies Spectral Logic (Chromatin) to the current gene execution.
///
/// If the organism is located on a colored grid cell, the behavior of the gene
/// is modulated by the color.
///
/// # Spectral Rules
///
/// - **Red (R > G, B)**: Amplification & Aggression.
///   - `Push(n)` -> `Push(n * 2)`
///   - `Sub` -> `Add`
///
/// - **Green (G > R, B)**: Life & Growth.
///   - `Photosynthesize` -> +5 Extra Energy (Total 10)
///   - `Add` -> `Mul`
///
/// - **Blue (B > R, G)**: Logic & Time.
///   - `Push(n)` -> `Push(n / 2)`
///   - `Add` -> `Sub`
///
pub fn apply_chromatin_effect(
    vm: &mut ChimeraVM,
    op: OpCode,
    args: &[Nucleotide],
) -> Option<(OpCode, Vec<Nucleotide>)> {
    let (cy, cx) = vm.context_loc;
    if cy >= vm.chroma_grid.len() || cx >= vm.chroma_grid[0].len() {
        return None;
    }

    let cell = vm.chroma_grid[cy][cx];
    let (r, g, b) = cell.fg?; // Return None if no color

    let max = r.max(g).max(b);
    if max == 0 {
        return None;
    }

    let is_red = r == max;
    let is_green = g == max && !is_red;
    let is_blue = b == max && !is_red && !is_green;

    if is_red {
        match op {
            OpCode::Push => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    let mut new_args = args.to_vec();
                    new_args[0] = Nucleotide::Number(n.saturating_mul(2));
                    return Some((op, new_args));
                }
            }
            OpCode::Sub => return Some((OpCode::Add, args.to_vec())),
            _ => {}
        }
    } else if is_green {
        match op {
            OpCode::Photosynthesize => {
                // Boost energy by 5 immediately. The OpCode execution will add another 5.
                vm.energy = vm.energy.saturating_add(5);
                return None;
            }
            OpCode::Add => return Some((OpCode::Mul, args.to_vec())),
            _ => {}
        }
    } else if is_blue {
        match op {
            OpCode::Push => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    let mut new_args = args.to_vec();
                    new_args[0] = Nucleotide::Number(n / 2);
                    return Some((op, new_args));
                }
            }
            OpCode::Add => return Some((OpCode::Sub, args.to_vec())),
            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dna, Gene, Helix, Strand};
    use crate::vm::ChromaCell;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_red_amplification() {
        let mut vm = make_vm();
        vm.chroma_grid[8][8] = ChromaCell {
            fg: Some((255, 0, 0)),
            char: None,
        };
        vm.context_loc = (8, 8);

        let op = OpCode::Push;
        let args = vec![Nucleotide::Number(10)];

        let result = apply_chromatin_effect(&mut vm, op.clone(), &args);
        assert!(result.is_some());
        let (new_op, new_args) = result.unwrap();
        assert_eq!(new_op, OpCode::Push);
        assert_eq!(new_args[0], Nucleotide::Number(20));
    }

    #[test]
    fn test_green_mutation() {
        let mut vm = make_vm();
        vm.chroma_grid[8][8] = ChromaCell {
            fg: Some((0, 255, 0)),
            char: None,
        };
        vm.context_loc = (8, 8);

        let op = OpCode::Add;
        let args = vec![];

        let result = apply_chromatin_effect(&mut vm, op, &args);
        assert!(result.is_some());
        let (new_op, _) = result.unwrap();
        assert_eq!(new_op, OpCode::Mul);
    }

    #[test]
    fn test_green_photosynthesis() {
        let mut vm = make_vm();
        vm.chroma_grid[8][8] = ChromaCell {
            fg: Some((0, 255, 0)),
            char: None,
        };
        vm.context_loc = (8, 8);
        vm.energy = 50;

        let op = OpCode::Photosynthesize;
        let args = vec![];

        let result = apply_chromatin_effect(&mut vm, op, &args);
        assert!(result.is_none());
        assert_eq!(vm.energy, 55); // +5 immediately
    }

    #[test]
    fn test_blue_dampening() {
        let mut vm = make_vm();
        vm.chroma_grid[8][8] = ChromaCell {
            fg: Some((0, 0, 255)),
            char: None,
        };
        vm.context_loc = (8, 8);

        let op = OpCode::Push;
        let args = vec![Nucleotide::Number(10)];

        let result = apply_chromatin_effect(&mut vm, op, &args);
        assert!(result.is_some());
        let (_, new_args) = result.unwrap();
        assert_eq!(new_args[0], Nucleotide::Number(5));
    }
}
