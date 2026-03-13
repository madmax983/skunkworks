use super::normalize_coords;
use crate::ast::{Gene, JunctionType, Nucleotide};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value, MAX_STRANDS};
use std::str::FromStr;

/// Applies the logic for the Genesis Rune (G).
///
/// **Genesis (G)**:
/// *   **Inputs**:
///     *   **West**: OpCode String (e.g., "push").
///     *   **North**: Argument(s) (Int, String, or Junction).
/// *   **Output**:
///     *   **South**: Gene Tuple `[OpCode, Args]`.
///
/// **Behavior**:
/// Constructs a Gene Tuple from the inputs. This tuple can be used by Ligation (Z)
/// or other genetic runes to modify DNA.
pub fn apply_genesis_rune(
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut [Vec<Option<Value>>],
) -> bool {
    let mut changes = false;

    // Check outputs first to avoid re-triggering if already set
    if next_signals[y][x].is_some() {
        return false;
    }

    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        current_signals[ny][nx].clone()
    } else {
        None
    };

    if let (Some(Value::Str(op_str)), Some(args_val)) = (&w_sig, &n_sig) {
        // Parse OpCode
        if OpCode::from_str(op_str).is_ok() {
            // Normalize Args into a Junction List
            let args = match args_val {
                Value::Junction(JunctionType::All, list) => list.clone(),
                Value::Junction(JunctionType::Any, list) => list.clone(),
                v => vec![v.clone()],
            };

            // Construct Gene Tuple: [OpCodeStr, ArgsJunction]
            // We store OpCode as string in the tuple for easier inspection/serialization
            let gene_tuple = Value::Junction(
                JunctionType::All,
                vec![
                    Value::Str(op_str.clone()),
                    Value::Junction(JunctionType::All, args),
                ],
            );

            // Output to South
            if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                // We write to South cell in next_signals
                if next_signals[sy][sx] != Some(gene_tuple.clone()) {
                    next_signals[sy][sx] = Some(gene_tuple);
                    changes = true;
                }
            }
        } else {
            println!("DEBUG: Genesis failed to parse OpCode: {}", op_str);
        }
    } else {
        println!(
            "DEBUG: Genesis missing inputs at {},{}: W={:?} N={:?}",
            x, y, w_sig, n_sig
        );
    }

    changes
}

/// Applies the logic for the Ligation Rune (Z).
///
/// **Ligation (Z)**:
/// *   **Inputs**:
///     *   **West**: Gene Tuple `[OpCode, Args]`.
///     *   **North**: Strand Index (Int).
/// *   **Output**:
///     *   **East**: Success Signal (1).
///
/// **Behavior**:
/// Appends the Gene described by the tuple to the specified Strand.
pub fn apply_ligation_rune(vm: &mut ChimeraVM, y: usize, x: usize) {
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        vm.prologue_state.signal_grid[ny][nx].clone()
    } else {
        None
    };

    if let (Some(gene_tuple), Some(Value::Int(strand_idx))) = (&w_sig, &n_sig) {
        // Parse Gene Tuple
        if let Value::Junction(_, list) = gene_tuple {
            if list.len() == 2 {
                if let (Value::Str(op_str), Value::Junction(_, args_list)) = (&list[0], &list[1]) {
                    if let Ok(op) = OpCode::from_str(op_str) {
                        // Convert Value Args to Nucleotides
                        let nucleotides: Vec<Nucleotide> =
                            args_list.iter().map(value_to_nucleotide).collect();

                        let gene = Gene {
                            op,
                            args: nucleotides,
                        };

                        // Append to DNA
                        let s_idx = *strand_idx as usize;
                        if s_idx < vm.dna.helix.strands.len() {
                            vm.dna.helix.strands[s_idx].genes.push(gene);

                            // Success Output (East)
                            if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                                vm.prologue_state.signal_grid[ey][ex] = Some(Value::Int(1));
                                // Also light up self
                                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                            }

                            vm.output
                                .push(format!("GENETICS: Ligated {} to Strand {}", op_str, s_idx));
                        } else {
                            println!("DEBUG: Ligation failed: Invalid strand index {}", s_idx);
                        }
                    } else {
                        println!("DEBUG: Ligation failed to parse OpCode: {}", op_str);
                    }
                }
            }
        }
    } else {
        println!(
            "DEBUG: Ligation missing inputs at {},{}: W={:?} N={:?}",
            x, y, w_sig, n_sig
        );
    }
}

fn value_to_nucleotide(v: &Value) -> Nucleotide {
    match v {
        Value::Int(n) => Nucleotide::Number(*n),
        Value::Str(s) => Nucleotide::String(s.clone()),
        Value::Junction(t, list) => {
            let nucleos = list.iter().map(value_to_nucleotide).collect();
            Nucleotide::Junction(*t, nucleos)
        }
        _ => Nucleotide::Number(0),
    }
}

/// Applies the logic for the Hybridize Rune (⨁).
///
/// **Hybridize (⨁)**:
/// *   **Inputs**:
///     *   **West**: Strand Index A (Int).
///     *   **North**: Strand Index B (Int).
/// *   **Output**:
///     *   **East**: New Strand Index (Int).
///
/// **Behavior**:
/// It reads two strand indices. If both are valid, it creates a new strand by concatenating
/// the genes of Strand A and Strand B (A + B). The new strand is appended to the DNA helix,
/// and its index is emitted to the East.
///
/// If the helix is full (`MAX_STRANDS`), it fails silently or logs an error.
pub fn apply_hybridize_rune(vm: &mut ChimeraVM, y: usize, x: usize) {
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        vm.prologue_state.signal_grid[ny][nx].clone()
    } else {
        None
    };

    if let (Some(Value::Int(idx_a)), Some(Value::Int(idx_b))) = (w_sig, n_sig) {
        let len = vm.dna.helix.strands.len();
        if idx_a >= 0 && (idx_a as usize) < len && idx_b >= 0 && (idx_b as usize) < len {
            let strand_a = &vm.dna.helix.strands[idx_a as usize];
            let strand_b = &vm.dna.helix.strands[idx_b as usize];

            let mut new_genes = strand_a.genes.clone();
            new_genes.extend(strand_b.genes.clone());

            if vm.dna.helix.strands.len() < MAX_STRANDS {
                let new_strand = crate::ast::Strand { genes: new_genes };
                vm.dna.helix.strands.push(new_strand);
                let new_idx = vm.dna.helix.strands.len() - 1;

                if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                    vm.prologue_state.signal_grid[ey][ex] = Some(Value::Int(new_idx as i64));
                    // Also light up self to show activity
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));

                    vm.output.push(format!(
                        "GENETICS: Hybridized Strands {} + {} -> {}",
                        idx_a, idx_b, new_idx
                    ));
                }
            } else {
                vm.output
                    .push("GENETICS: Hybridization failed (MAX_STRANDS limit reached)".to_string());
            }
        }
    }
}
