Title: 👺 Havoc: Capacity overflow panic in `generate_miura_grid`

🧨 **The Trigger:** Passing dimensions like `(0, usize::MAX - 1)` to `generate_miura_grid` passes the `.checked_mul` bounds checks but still results in `capacity = usize::MAX`, triggering a fatal capacity overflow panic when passed to `Vec::with_capacity()`.

📉 **The Stack Trace:**
```
thread 'havoc_origami_capacity_panic_inner' panicked at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/alloc/src/raw_vec/mod.rs:28:5:
capacity overflow
```

🧪 **Reproduction:** Run `cargo test -p origami --test havoc`.

😈 **Comment:** "You thought a few `checked_mul`s were enough? You assumed the matrix would never be larger than RAM. You were wrong."
