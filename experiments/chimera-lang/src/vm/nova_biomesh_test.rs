#![cfg(test)]
#![cfg(feature = "nova")]

use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    ChimeraVM::new(dna)
}

#[test]
fn test_mesh_creation() {
    // [ push(1) mesh_net(1) ] at (8,8)
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }, // ID
        Gene {
            op: OpCode::MeshNet,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    vm.step(); // push
    vm.step(); // mesh_net

    assert!(vm.biomesh.nodes.contains_key(&(8, 8)));
    assert_eq!(vm.biomesh.nodes.get(&(8, 8)).unwrap().id, 1);
}

#[test]
fn test_mesh_grow_connect() {
    // 1. Create Node 1 at (8,8)
    // 2. Move to (8,9)
    // 3. Create Node 2
    // 4. MeshGrow

    // We can't move easily in one strand without Migrate which checks walls.
    // Let's use direct grid manipulation or just assume Migrate works.
    // Or just manually set up state.

    let mut vm = make_vm(vec![]);

    // Manual setup
    vm.context_loc = (8, 8);
    // MeshNet(1)
    crate::vm::nova_biomesh::exec_mesh_net(&mut vm, &[Nucleotide::Number(1)]);

    vm.context_loc = (8, 9);
    // MeshNet(2)
    crate::vm::nova_biomesh::exec_mesh_net(&mut vm, &[Nucleotide::Number(2)]);

    // MeshGrow at (8,9) should connect to (8,8)
    crate::vm::nova_biomesh::exec_mesh_grow(&mut vm);

    let node1 = vm.biomesh.nodes.get(&(8, 8)).unwrap();
    let node2 = vm.biomesh.nodes.get(&(8, 9)).unwrap();

    assert!(node1.connections.contains(&(8, 9)));
    assert!(node2.connections.contains(&(8, 8)));
}

#[test]
fn test_mesh_send_recv() {
    let mut vm = make_vm(vec![]);

    // Setup 2 connected nodes
    vm.context_loc = (8, 8);
    crate::vm::nova_biomesh::exec_mesh_net(&mut vm, &[Nucleotide::Number(1)]);

    vm.context_loc = (8, 9);
    crate::vm::nova_biomesh::exec_mesh_net(&mut vm, &[Nucleotide::Number(2)]);
    crate::vm::nova_biomesh::exec_mesh_grow(&mut vm);

    // Send 42 from Node 1 to Node 2
    vm.context_loc = (8, 8);
    vm.stack.push(Value::Int(2)); // Target ID
    vm.stack.push(Value::Int(42)); // Value
    crate::vm::nova_biomesh::exec_mesh_send(&mut vm);

    // Check Node 2 buffer
    let node2 = vm.biomesh.nodes.get(&(8, 9)).unwrap();
    assert_eq!(node2.buffer.len(), 1);
    assert_eq!(node2.buffer[0], Value::Int(42));

    // Recv at Node 2
    vm.context_loc = (8, 9);
    crate::vm::nova_biomesh::exec_mesh_recv(&mut vm);

    assert_eq!(vm.stack.pop(), Some(Value::Int(42)));
}
