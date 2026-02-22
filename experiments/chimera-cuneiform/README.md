# Chimera Cuneiform 🏺

> "The ink is wet, but the clay hardens."

**A hybrid of `chimera-lang` (Genetic Agents) and `saros-cycle` (Babylonian Time Series).**

## Concept

**Paleographic Memory**: Agents inhabit a 2D grid ("Clay Tablet") where they can write values. Unlike RAM, this memory "hardens" over time.
- **Fresh Clay**: Mutable. Agents can overwrite it.
- **Hard Clay**: Immutable. Writes fail.
- **The Flood**: Periodically (based on the Saros Cycle), a flood washes away all "soft" clay. Only hardened, immutable knowledge survives.

Agents evolve to:
1. Write quickly before the clay dries.
2. Predict the flood and position themselves to preserve their history.
3. Use the hardened data of ancestors as a reference.

## Lineage

- **Parent A (Allele: `chimera-lang`)**:
  - `ChimeraVM`: The biological agent executing DNA.
  - `Dna`: Genetic code evolved via mutation.
  - `OpCodes`: Stack-based instructions for movement and writing.

- **Parent B (Allele: `saros-cycle`)**:
  - `Sexagesimal`: (Included module) numeric system logic.
  - `Cycle Logic`: Periodic events (Floods) driving selection.
  - `Tablet Metaphor`: Persistence vs. Erasure.

- **Novel Trait**:
  - **Immutable History**: Data permanence is a function of time (Age).
  - **Catastrophic Selection**: Survival depends on creating structures that outlast the cycle.

## Usage

```bash
cargo run -p chimera-cuneiform
```

## Controls

- `q` / `Esc`: Quit
