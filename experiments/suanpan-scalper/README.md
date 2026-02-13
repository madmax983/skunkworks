# Suanpan Scalper 🧮💸

> "The abacus is faster than the calculator, provided the operator is sufficiently skilled."

**Suanpan Scalper** is a High-Frequency Trading (HFT) simulation that rejects modern floating-point units in favor of the **Suanpan** (traditional Chinese Abacus).

## 🏺 The Concept

In the fast-paced world of algorithmic trading, every microsecond counts. Modern CPUs are fast, but are they *soulful*? This project explores an alternative history where quantitative finance evolved from the bead-arithmetic of the Ming Dynasty.

The core logic of this trading bot relies entirely on a simulated **2-5 Suanpan** (2 Heaven beads, 5 Earth beads). All price calculations, moving averages, and order executions are performed by physically shifting virtual beads.

## 🕹️ Controls

- **Q / Esc**: Quit the simulation.
- **R**: Reset the simulation.

## 🚀 Running

```bash
cargo run --release
```

## 🧮 The System

The simulation uses a `Suanpan` struct that maintains state via bead positions.
- **Heaven Beads**: Value 5 (2 per rod)
- **Earth Beads**: Value 1 (5 per rod)
- **Base**: Decimal (standard), but handled via bead logic.

The TUI visualizes the frantic movement of beads as the market moves.

## 📚 Archaeology

This project resurrects the `Suanpan` arithmetic algorithms:
- **Addition**: Complementary numbers and carry propagation.
- **Subtraction**: Borrowing logic.

No `f64` arithmetic is used for the core trading logic. Only integer math on the abacus.
