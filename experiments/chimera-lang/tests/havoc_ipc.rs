#![cfg(feature = "nova")]

use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::{ChimeraVM, Value};
use std::fs;
use std::path::Path;

fn make_empty_dna() -> Dna {
    Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    }
}

#[test]
fn havoc_ipc_vulnerability_suite() {
    // We run scenarios sequentially because the IPC system uses a hardcoded
    // shared directory `.chimera_ether`, making parallel tests impossible.
    // This in itself is a finding (Lack of Isolation).

    test_ipc_lock_stealing();
}

fn test_ipc_lock_stealing() {
    let channel = 666;
    let _ = fs::remove_dir_all(".chimera_ether");
    let channel_dir = Path::new(".chimera_ether").join(channel.to_string());
    fs::create_dir_all(&channel_dir).expect("Failed to create dir");

    // 1. Simulate Process A: Locks "msg.json" -> "msg.json.lock"
    let msg = Value::Str("StealMe".to_string());
    let content = serde_json::to_string(&msg).unwrap();
    let lock_file = channel_dir.join("msg.json.lock");
    fs::write(&lock_file, content).expect("Failed to write lock file");

    // 2. Simulate Process B (us): Calls receive().
    // Robust behavior: Should ignore "msg.json.lock".
    // Vulnerable behavior: Renames to "msg.json.lock.lock" and reads it.
    let mut vm = ChimeraVM::new(make_empty_dna());
    vm.stack.push(Value::Int(channel));
    chimera_lang::vm::ipc::receive(&mut vm);

    // 3. Verify
    let val = vm.stack.pop();

    // RED PHASE: We assert that the system behaves CORRECTLY (Securely).
    // Because the bug exists, this assertion MUST FAIL.
    // If this test passes, the bug is fixed.
    // If this test panics, we have proven the vulnerability.
    assert_ne!(
        val,
        Some(Value::Str("StealMe".to_string())),
        "👺 HAVOC SUCCESS: Vulnerability confirmed! The IPC system stole a locked file."
    );
}
