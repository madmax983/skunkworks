use super::ChimeraVM;

use crate::vm::Value;

pub fn exec_echo(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(r) = val {
            if r > 0 {
                let (cy, cx) = vm.context_loc;
                let mut best_dist_sq = (r * r) + 1;
                let mut target_gene = None;

                // Scan census
                for info in &vm.census {
                    // Skip self (approximate check: if loc is same, likely self or overlapping)
                    // But Echoing self is fine? "Loop".
                    // Let's exclude if distance is 0 to avoid trivial echo.

                    let dy = (info.loc.0 as i64).saturating_sub(cy as i64);
                    let dx = (info.loc.1 as i64).saturating_sub(cx as i64);
                    let dist_sq = dy * dy + dx * dx;

                    if dist_sq > 0 && dist_sq <= (r * r) {
                        if dist_sq < best_dist_sq {
                            best_dist_sq = dist_sq;
                            target_gene = info.last_gene.clone();
                        }
                    }
                }

                if let Some(gene) = target_gene {
                    vm.energy = vm.energy.saturating_sub(best_dist_sq); // Cost
                    vm.output.push(format!("ECHO: Mimicked {}", gene.op));
                    return vm.execute_gene_inner(gene.op, &gene.args);
                } else {
                    vm.output.push("ECHO: No neighbor found".to_string());
                }
            } else {
                vm.output.push("ECHO: Radius must be positive".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for echo".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for echo".to_string());
    }
    None
}
