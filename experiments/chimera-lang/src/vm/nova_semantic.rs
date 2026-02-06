#![cfg(feature = "nova")]

use crate::vm::{ChimeraVM, Value};
use tui_semantic::{Entity, Snapshot};

/// Generates a semantic snapshot of the Chimera VM state.
///
/// This snapshot captures the biological and physical state of the organism,
/// including energy levels, active organelles, and grid contents.
pub fn snapshot(vm: &ChimeraVM) -> Snapshot {
    let mut snap = Snapshot::new("chimera-vm")
        .with_metric("energy", vm.energy)
        .with_metric("phase", format!("{:?}", vm.phase))
        .with_metric("halted", vm.halted)
        .with_metric("chaos_mode", vm.chaos_mode)
        .with_state(if vm.halted { "dead" } else { "alive" })
        .with_viewport(16, 16);

    // Organism Entity (Self) representing the main execution context
    let organism = Entity::new("organism")
        .with_id("root")
        .at(vm.context_loc.1 as f64, vm.context_loc.0 as f64) // x, y
        .with_prop("energy", vm.energy)
        .with_prop("recursion_depth", vm.recursion_depth as i64)
        .with_prop("strand_idx", vm.ip.0 as i64)
        .with_prop("gene_idx", vm.ip.1 as i64)
        .with_prop("direction", vm.direction as i64);

    snap = snap.with_entity(organism);

    // Stack Context
    if let Some(top) = vm.stack.last() {
        snap = snap.with_entity(
            Entity::new("stack_top")
                .with_prop("value", top.to_string())
        );
    }
    snap = snap.with_metric("stack_depth", vm.stack.len());

    // Organelles
    for (i, org) in vm.organelles.iter().enumerate() {
        let kind_str = format!("{:?}", org.kind);
        let e = Entity::new("organelle")
            .with_id(format!("org_{}", i))
            .at(org.context_loc.1 as f64, org.context_loc.0 as f64)
            .with_prop("kind", kind_str)
            .with_prop("halted", org.halted)
            .with_prop("strand_idx", org.ip.0 as i64)
            .with_prop("gene_idx", org.ip.1 as i64);
        snap = snap.with_entity(e);
    }

    // Grid (Sparse Representation)
    // Only capture non-empty cells to save space/noise
    for y in 0..16 {
        for x in 0..16 {
            let val = &vm.grid[y][x];
            if !matches!(val, Value::Int(0)) {
                let e = Entity::new("cell")
                    .at(x as f64, y as f64)
                    .with_prop("value", val.to_string());
                snap = snap.with_entity(e);
            }
        }
    }

    // Capture Portals
    for ((y1, x1), (y2, x2)) in &vm.portals {
        let e = Entity::new("portal")
            .at(*x1 as f64, *y1 as f64)
            .with_prop("target_x", *x2 as i64)
            .with_prop("target_y", *y2 as i64);
        snap = snap.with_entity(e);
    }

    snap
}
