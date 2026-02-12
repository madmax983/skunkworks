# Quipu

**A Rust library for modeling the Inca number system.**

The Quipu (or Khipu) was an ancient recording device used by the Incas for accounting, census data, and possibly even narratives. It consisted of a main cord with pendant cords knotted in specific ways to represent numbers.

## Features

- **Inca Arithmetic:** Perform addition and subtraction using the logic of moving knots.
- **TUI Visualization:** Render Quipus as text strings suitable for terminal applications.
- **Historical Accuracy:** Implements the distinction between Simple, Long, and Figure-Eight knots based on position.

## Documentation

For full documentation and examples, run:

```bash
cargo doc -p quipu --open
```

Or see the `src/lib.rs` file.

## Example

```rust
use quipu::{Quipu, Cord};

let mut q = Quipu::new();
q.add_cord(Cord::from(100)); // One simple knot at the top
println!("{}", q);
```
