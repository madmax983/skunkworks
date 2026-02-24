use super::normalize_coords;
use crate::vm::{ChimeraVM, Value, MAX_STRANDS};

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
                vm.output.push("GENETICS: Hybridization failed (MAX_STRANDS limit reached)".to_string());
            }
        }
    }
}
