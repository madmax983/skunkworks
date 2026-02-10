use chimera_lang::vm::{ChimeraVM, Value};
use chimera_lang::opcode::OpCode;
use chimera_lang::ast::{Dna, Helix};
use std::thread;
use std::sync::{Arc, Barrier};

#[cfg(feature = "nova")]
#[test]
fn test_akashic_race_condition() {
    // Generate unique file paths for test isolation
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let db_file = format!(".chimera_akashic_{}.json", now);
    let lock_file = format!(".chimera_akashic_{}.lock", now);

    // Set env vars (Safe since integration tests run in their own process)
    std::env::set_var("CHIMERA_AKASHIC_FILE", &db_file);
    std::env::set_var("CHIMERA_LOCK_FILE", &lock_file);

    // 1. Clean up any existing akashic file
    let _ = std::fs::remove_file(&db_file);
    let _ = std::fs::remove_file(&lock_file);

    fn empty_dna() -> Dna {
        Dna { helix: Helix { strands: vec![] } }
    }

    // Initialize with 0
    {
        let mut vm = ChimeraVM::new(empty_dna());
        vm.stack.push(Value::Str("counter".to_string()));
        vm.stack.push(Value::Int(0));
        chimera_lang::vm::akashic::exec_akashic_op(&mut vm, OpCode::AkashicWrite, &[]);
    }

    let num_threads = 10;
    let increments_per_thread = 20;
    let barrier = Arc::new(Barrier::new(num_threads));

    let mut handles = vec![];

    for _ in 0..num_threads {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            b.wait();
            let mut vm = ChimeraVM::new(empty_dna());
            for _ in 0..increments_per_thread {

                // Atomic Add
                vm.stack.push(Value::Str("counter".to_string()));
                vm.stack.push(Value::Int(1));
                chimera_lang::vm::akashic::exec_akashic_op(&mut vm, OpCode::AkashicAdd, &[]);

            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // Verify final result
    let mut vm = ChimeraVM::new(empty_dna());
    vm.stack.push(Value::Str("counter".to_string()));
    chimera_lang::vm::akashic::exec_akashic_op(&mut vm, OpCode::AkashicRead, &[]);
    let val = vm.stack.pop().unwrap();
    let final_count = match val {
        Value::Int(n) => n,
        _ => -1,
    };

    let expected = (num_threads * increments_per_thread) as i64;
    println!("Final Count: {}, Expected: {}", final_count, expected);

    // Cleanup
    let _ = std::fs::remove_file(&db_file);
    let _ = std::fs::remove_file(&lock_file);

    assert_eq!(final_count, expected, "Race condition detected! Expected {}, got {}", expected, final_count);
}
