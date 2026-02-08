# Quipu Serializer ⚛️🏺

A Serde serializer and interactive TUI that uses **Incan Quipu Knots** as the data representation format.

> "Data is not stored in bits, but in knots." - Genesis

## Features

- **Ancient Arithmetic**: Implements `Add` and `Sub` traits for `Cord` (Quipu strings) using physical knot manipulation logic (carrying over by untying/tying knots).
- **Serde Integration**: Serialize any Rust struct into a Quipu (a collection of cords).
- **TUI Visualizer**:
  - **Calculator Mode**: Add two numbers and see the result as Quipu knots.
  - **Serializer Mode**: Type JSON and see it instantly translated into a Quipu.

## Usage

### Run TUI
```bash
cargo run -p quipu-serializer
```

### Use as Library
```rust
use quipu_serializer::ser::to_quipu;
use serde::Serialize;

#[derive(Serialize)]
struct MyData {
    id: u64,
    active: bool,
}

let data = MyData { id: 123, active: true };
let quipu = to_quipu(&data).unwrap();
println!("{}", quipu);
```

## Knot System

- **Figure-Eight Knot (∞)**: Value 1 (Units position only).
- **Long Knot (≡N)**: Value 2-9 (Units position only).
- **Simple Knot (●)**: Value 1 (Tens and higher positions).
- **Empty**: Value 0.

## Logic
When adding two cords, we simulate the physical process:
1. Align cords by position (power of 10).
2. Sum the knots.
3. If knots > 9, untye 10 knots and tie 1 Simple knot in the next position up.
