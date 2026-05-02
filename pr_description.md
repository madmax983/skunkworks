👺 Havoc: Segfault/Capacity Overflow in `flocking` and `poincare-disk`

🧨 **The Trigger:**
Passing `usize::MAX` into the unvalidated `count` parameter in `crates/flocking` (during test bench allocation setup for `compute_force`) and `crates/poincare-disk` (during `pseudo_random_points` generation for properties testing) directly reaches `Vec::with_capacity(count)` causing a catastrophic integer overflow logic leading to a panic.

📉 **The Stack Trace:**
```
thread 'havoc_flocking_alloc_panic_inner' panicked at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/alloc/src/raw_vec/mod.rs:28:5:
capacity overflow

thread 'havoc_poincare_disk_alloc_panic_inner' panicked at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/alloc/src/raw_vec/mod.rs:28:5:
capacity overflow
```

🧪 **Reproduction:**
Run `cargo test -p flocking --test havoc` and `cargo test -p poincare-disk --test havoc`.

😈 **Comment:**
You assumed `count` would never be larger than RAM. You were wrong.
