# 👺 Havoc Report: Recursive Include Stack Overflow

## 🧨 The Trigger
The `preprocess` function in `compiler.rs` blindly follows `include` directives without checking for cycles or recursion depth. This allows a trivial Denial of Service (DoS) attack via two files that include each other.

## 📉 The Wreckage
```
thread 'test_recursive_include_crash' (3928) has overflowed its stack
fatal runtime error: stack overflow, aborting
```

## 🧪 Reproduction
1. Create `a.chs` containing `include "b.chs"`.
2. Create `b.chs` containing `include "a.chs"`.
3. Run `cargo run --features nova -- --input a.chs`.

Or run the included test:
```bash
cargo test --test repro_recursion
```

## 😈 Havoc's Note
You assumed the file system was a Directed Acyclic Graph. You were wrong. Recursion is infinite if you let it be.

# 👺 Havoc Report: IPC Message Theft via Lock Re-locking

## 🧨 The Trigger
The `ipc::receive` function in `vm/ipc.rs` iterates over all files in the channel directory, including files that are already locked (ending in `.lock`). It attempts to rename any file it finds by appending `.lock`.

## 📉 The Vulnerability
A Race Condition / Logic Error allows a process to "steal" a message that another process has already locked for processing.

1. Process A renames `msg.json` to `msg.json.lock`.
2. Process B (concurrently) lists directory, sees `msg.json.lock`.
3. Process B renames `msg.json.lock` to `msg.json.lock.lock`.
4. Process A fails to read `msg.json.lock` (file not found).
5. Process B reads the message from `msg.json.lock.lock`.

This violates the mutual exclusion property of the lock mechanism and causes message loss for the original intended receiver.

## 🧪 Reproduction
Run the included test:
```bash
cargo test --test havoc_ipc
```

**Note:** The test `havoc_ipc_vulnerability_suite` asserts that the system is secure. Since it is not, the test **panics** with `👺 HAVOC SUCCESS: Vulnerability confirmed!`. This failure IS the proof.

## 😈 Havoc's Note
Locks are only locks if everyone respects them. You just added more locks on top of locks until the door fell off. Also, hardcoding `.chimera_ether` means every test runs in the same universe. Welcome to the multiverse collision.
