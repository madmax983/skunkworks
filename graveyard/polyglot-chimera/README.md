# Polyglot Chimera 🧬

> "In the beginning was the Word, and the Word was made flesh."

**Parents**: `experiments/geo-linguistics` × `experiments/chimera-lang`

A hybrid experiment where `ChimeraVM` agents inhabit a terrain generated from linguistic phonemes. The agents "read" the terrain (Height/Hardness) and "speak" back by modifying it (Eating/Digging).

## Concept

1.  **Linguistic Terrain**: Text is converted into a 2D terrain where phonemes determine height and hardness (inherited from `geo-linguistics`).
2.  **Polyglot Agents**: `ChimeraVM` instances roam this terrain.
3.  **Sensory Input**: Agents receive terrain data (Height, Hardness) on their stack.
4.  **Behavior**: Agents execute DNA to decide whether to Move or Eat (Erode) the terrain.
5.  **Emergence**: A swarm of agents that "consume" language, reshaping the landscape.

## DNA

The agents are spawned with a "Grazer" genome:
-   **Sense**: Check Height.
-   **Decide**: If High -> Eat. If Low -> Move.
-   **Act**: Modify the terrain or change position.

## Controls

-   **Type**: Add characters to the landscape.
-   **Backspace**: Remove characters.
-   **E**: Trigger natural erosion.
-   **R**: Respawn agents.
