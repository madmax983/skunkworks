# 🧲 Chimera Magnetron 🧬

> "The code is eating the disk." - The Splice Surgeon

**Chimera Magnetron** is a hybrid experiment combining the physical decay simulation of **Magnetron Decay** with the biological evolution of **Chimera Lang**.

## Concept

This experiment visualizes **Data Extremophiles**: ChimeraVM organisms that live on the tracks of a decaying hard drive platter.

- **The Platter**: A physical medium (16x16 grid) with Magnetization and Coercivity. It decays over time.
- **The Organisms**: Chimera agents execution logic on the grid. They consume Magnetization (metabolism) to survive.
- **The Head**: Controlled by you (The Controller). You can Scrub tracks to restore Magnetization, but this also wipes the memory at that location (killing agents).

## Dynamics

1.  **Bit Rot -> Mutation**: As sector magnetization drops, the probability of bit flips increases. These bit flips directly mutate the ChimeraVM memory (Grid Values), causing the organisms to glitch or evolve.
2.  **Metabolism -> Decay**: Active agents (those executing code) consume the local magnetization faster than natural decay. A heavy compute load will strip the disk bare.
3.  **Scrubbing -> Genocide**: The only way to save the drive's health is to Scrub it with the Head. But scrubbing overwrites the sector, killing any organism living there.

## Controls

- **Arrows**: Move the Read/Write Head.
- **Space**: Scrub/Refresh sector (Restores Magnetization, Wipes VM Memory).
- **+/-**: Adjust Time Scale.
- **Q**: Quit.

## Lineage

- **Parent A**: `experiments/magnetron-decay` (Physics: Magnetization, Coercivity, Decay).
- **Parent B**: `experiments/chimera-lang` (Biology: Genetic Code, VM, Metabolism).
- **Novel Trait**: **Data Parasitism**. Organisms that feed on the physical integrity of their storage medium.

## Implementation Details

- **Grid Size**: 16x16 (Matching ChimeraVM default).
- **Interaction**:
    - `Platter::update()` drives physical decay.
    - `ChimeraVM::step()` drives biological execution.
    - `Mutation`: Low Magnetization -> Random Bit Flips in VM Grid.
    - `Consumption`: High Execution Trail -> Reduced Magnetization.
