# Chimera Current 🧬🌊

**"Evolutionary Hydrodynamics"**

A hybrid experiment combining fluid dynamics (`typographic-turbulence`) with evolutionary computation (`chimera-lang`).

## Lineage

- **Parent A**: `typographic-turbulence` (LBM D2Q9 Fluid Simulation with ASCII rendering)
- **Parent B**: `chimera-lang` (ChimeraVM for genetic evolution)

## Concept

ChimeraVM agents drift in a fluid simulation made of text. They consume the "ink" (fluid density) to survive and evolve. Their movement is a mix of fluid dynamics (being pushed by the current) and their own propulsion driven by their genetic code.

- **Agents**: Each agent is a `ChimeraVM` instance executing a DNA program.
- **Environment**: A Lattice Boltzmann Method (LBM) fluid simulation where density is rendered as ASCII characters.
- **Interaction**: Agents sense local density and fluid velocity. They output acceleration vectors to swim. They "eat" fluid density to gain energy.

## Controls

- **Mouse Left**: Add fluid density at cursor.
- **Visuals**:
  - Blue text: Fluid density.
  - `@`: Agents. Color indicates energy (Green > Yellow > Red).

## Status

Compiles and runs. Agents exhibit basic swimming and eating behavior.
