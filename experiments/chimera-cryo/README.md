# Chimera Cryo ❄️🧬

**Lineage:** `chimera-lang` × `cryo-colony`

A hybrid experiment combining **Genetic Programming** (ChimeraVM) with **Thermodynamic Phase Transitions** (Cryo-Colony).

## Concept

Agents (Termites/Bots) are driven by a **ChimeraVM** brain. They inhabit a 3D world where matter can shift between Solid and Liquid states based on temperature. Agents have a genome (DNA) that evolves over time.

- **Inputs**: Agents sense their local environment (Temperature, Position, Energy).
- **Outputs**: Agents execute genes to Move, Heat, or Cool their surroundings.
- **Metabolism**: Actions cost energy. Agents must maintain energy levels to survive and reproduce.
- **Phase Change**: The environment reacts to agent actions (melting/freezing), which in turn affects agent movement (viscosity/spring forces).

## Evolution

Agents that survive long enough and gather sufficient energy will reproduce, passing on their DNA with mutations. Natural selection should favor agents that can effectively manipulate their environment or navigate to optimal conditions.

## Visualization

- **Particles**:
  - **Blue**: Solid (Ice)
  - **Red**: Liquid (Magma/Water)
- **Agents**:
  - **Orange**: Heating
  - **Cyan**: Cooling
  - **Green**: Moving
  - **Purple**: Reproducing
  - **White**: Just Born

## Controls

- **WASD/Space/Shift**: Move Camera
- **Arrow Up/Down**: Adjust Global Ambient Temperature
- **R**: Reset Simulation
