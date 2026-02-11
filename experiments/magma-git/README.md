# Magma Git 🌋

**"Magmatic Codeflow"**

A hybrid experiment splicing `tectonic-git` (Geological Code Analysis) with `fluid-rain` (Fluid Simulation).

## Concept

Git commits erupt from the top of the screen as active magma (fluid particles). As they flow down and cool, they solidify into geological strata, forming a history of the codebase.

- **Eruption**: New commits trigger a burst of particles.
- **Flow**: Particles move according to gravity and fluid dynamics.
- **Deposition**: Cooling magma solidifies into static blocks, building the history layer by layer.

## Lineage

- **Parent A**: `experiments/tectonic-git` (Strata visualization)
- **Parent B**: `experiments/fluid-rain` (SPH Fluid physics)

## Controls

- `q`: Quit

## Implementation

- **Physics**: Simple particle physics with temperature-based state changes (Liquid -> Solid).
- **Grid**: A dynamic "Strata Grid" that accumulates solidified particles.
- **Git**: Extracts commit history using `git-associates`.

## Phenotype

The experiment visualizes the "heat" of active development turning into the "cold hard rock" of history. Large commits create massive flows that can bury previous structures. Refactoring (simulated) could remelt strata (future feature).
