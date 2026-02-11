# Chimera Bridge

A hybrid experiment combining `biomimetic-bridge` and `chimera-lang`.

## 🧬 Lineage

- **Parent A**: `experiments/biomimetic-bridge` (Swarm Intelligence, Bridge Construction)
- **Parent B**: `experiments/chimera-lang` (Genetic Programming, Virtual Machine)

## 🧪 Concept

**Structural Computation.**

Army ants form bridges to span gaps, but in this hybrid, each ant is a Turing-complete computer (running a `ChimeraVM`). The bridge structure is not just physical but computational. Ants execute genetic code to decide when to link up, when to let go, and how to transfer data (pheromones/signals).

## 🔬 Phenotype

- Ants run a `ChimeraVM` instance with unique DNA.
- Ants traverse a gap, forming bridges dynamically.
- The VM state (Stack, Registers) influences bridging probability.
- Visualization via `macroquad`.

## 🚀 Usage

```bash
cargo run -p chimera-bridge
```
