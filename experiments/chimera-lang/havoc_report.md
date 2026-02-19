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

# 👺 Havoc Report: Recursive Structure DoS in Chimera Prologue

## 🧨 The Trigger
The `Prologue` system allows for the construction of recursive data structures using the `[` (Collect) and `!` (Source) runes. By creating a feedback loop where a `Value` is wrapped in a `Junction` and written back to itself, the depth of the data structure grows linearly with execution ticks.

The `Value` enum supports arbitrarily deep nesting:
```rust
pub enum Value {
    Junction(JunctionType, Vec<Value>),
    // ...
}
```

Critically, the `std::fmt::Display` implementation for `Value` is recursive and does not check for recursion depth:
```rust
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Junction(t, vals) => {
                // ...
                for v in vals {
                    write!(f, "{}", v)?; // Unbounded recursion
                }
                // ...
            }
        }
    }
}
```

## 💥 The Explosion
By running a circuit that builds a deep `Junction` structure (or constructing one manually via API), and then triggering a print operation (e.g., via `?` Sink or CLI output), the program will encounter a **Stack Overflow**, crashing the entire VM process.

Alternatively, the massive memory consumption of the growing structure constitutes a **Memory Exhaustion DoS**.

## 🧪 Reproduction
Run the provided example:
```bash
cargo run --example havoc_recursive_bomb
```

The example attempts to build the bomb via Prologue logic. If the circuit fails to reach critical mass (due to nondeterministic execution ordering or resilience), it engages a "Manual Override" to construct the structure programmatically and detonate it.

## 📉 Stack Trace (Simulated)
```
thread 'main' has overflowed its stack
fatal runtime error: stack overflow
```

## 😈 Havoc's Comment
"You built a language that mimics life, but you forgot that life is recursive. You put `MAX_RECURSION_DEPTH` on your genes, but left your data structures naked. I fed the Ouroboros its own tail, and it choked."
