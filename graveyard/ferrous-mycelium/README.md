# Ferrous Mycelium 🍄🧲

**Lineage:** `ferrous-genesis` × `chimera-mycelium`

A hybrid experiment simulating "Magnetotropic Fungi".

## Concept
Hyphae (fungal threads) are composed of magnetic particles. They grow through a medium that records magnetic field intensity.
- **Sensing:** Hyphae sense the local magnetic field and its gradient.
- **Processing:** Each hypha tip is a `ChimeraVM` agent that executes genetic code to decide movement (turn angle) and branching probability based on magnetic inputs.
- **Actuation:** Hyphae move and deposit magnetic trails ("Stalks") into the field.
- **Feedback:** The deposited trails alter the field, influencing the growth of future hyphae (Stigmergy).

## Novel Trait: Magnetotropism
The organism constructs its own magnetic lattice. The growth pattern is a result of the feedback loop between the biological agent and the physical field it creates.

## Visuals
- **Background:** Magnetic Field Intensity (Blue = North, Red = South, Black = Neutral).
- **Agents:** Hyphae tips (Yellow/Cyan/Magenta based on state).

## Controls
- **Automated**: The simulation runs itself.
- **Respawn**: If the colony dies out, it respawns automatically.

## Implementation Details
- **Physics**: Uses a grid-based magnetic field (scalar potential with polarity) instead of O(N^2) particle interactions for scalability.
- **Genetics**: `ChimeraVM` (Nova feature set) drives the decision making.
- **Rendering**: `macroquad` for real-time visualization.
