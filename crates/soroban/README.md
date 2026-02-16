# Soroban 🧮

**A Rust library for simulating the Japanese Abacus.**

The Soroban (算盤) is an ancient calculation tool, perfected in Japan, that allows for rapid arithmetic operations. Unlike electronic calculators, the Soroban represents numbers physically using beads on rods.

## Features

- **Accurate Simulation:** Models the 1 Heaven / 4 Earth bead configuration of the modern Soroban.
- **Visual Output:** Renders the state of the abacus as ASCII art (via `Display`).
- **Arithmetic Logic:** Implements addition and subtraction using the mechanical logic of carry and borrow.
- **Educational:** Learn how the bead movements correspond to numerical values.

## How to Read a Soroban

Each column represents a decimal digit (Units, Tens, Hundreds, etc.).

- **The Beam (Horizontal Bar):** The divider between the upper and lower deck. Beads only have value when they are pushed *towards* the beam.
- **Heaven Bead (Top):** Worth **5**. It is active when pushed *down* to the beam.
- **Earth Beads (Bottom):** Worth **1** each. They are active when pushed *up* to the beam.

### Bead Values

| Digit | Heaven (5) | Earth (1s) | Visual |
|-------|------------|------------|--------|
| **0** | Up (Off)   | 0 Up       | `   |   ` |
| **1** | Up (Off)   | 1 Up       | `   |*  ` |
| **5** | Down (On)  | 0 Up       | ` * |   ` |
| **7** | Down (On)  | 2 Up       | ` * |** ` |

## Usage

```rust
use soroban::Soroban;

fn main() {
    let mut s = Soroban::new();

    // Add 123
    s.add(123);
    println!("{}", s);
    // Output (Visual representation of 123)

    // Add 50
    s += 50; // Uses AddAssign trait
    assert_eq!(s.value(), 173);
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
soroban = "0.1.0"
```
