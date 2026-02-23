#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::nova_biomesh::BioMeshNode;
    use crate::vm::nova_signals::process_signals;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_biomesh_signal_propagation() {
        let mut vm = make_vm();

        // 1. Setup BioMesh Nodes
        // Node A at (1,1)
        // Node B at (1,10)
        vm.biomesh.nodes.insert((1, 1), BioMeshNode::new(1));
        vm.biomesh.nodes.insert((1, 10), BioMeshNode::new(2));

        // 2. Connect them
        if let Some(node) = vm.biomesh.nodes.get_mut(&(1, 1)) {
            node.connections.push((1, 10));
        }
        if let Some(node) = vm.biomesh.nodes.get_mut(&(1, 10)) {
            node.connections.push((1, 1));
        }

        // 3. Place Bang at (1,1)
        vm.grid[1][1] = Value::Str("*".to_string());
        vm.signal_grid[1][1] = 1; // Ignite it

        // 4. Process Signals
        // Node A should activate.
        // It should propagate signal to Node B (1,10) for the NEXT tick.
        process_signals(&mut vm);

        // 5. Check Node B signal in next tick
        // Note: process_signals updates signal_grid to next_signals at the end.
        // So vm.signal_grid[1][10] should be > 0.

        assert!(
            vm.signal_grid[1][10] > 0,
            "Signal did not propagate to connected mesh node at (1,10)"
        );
    }
}
