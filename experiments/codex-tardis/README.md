# Codex Tardis 🧬🌌

> "It's bigger on the inside, and full of stars."

**Codex Tardis** is a hybrid experiment visualizing a recursive file system where every directory is a room, and the walls are paved with Star Maps that encode the directory's data.

## 🧬 Lineage

This experiment is a cross between:

-   **[codex-void](../codex-void)**: Provides the `StarMap` steganography system (Radial Binary Encoding).
-   **[tardis-memory](../tardis-memory)**: Provides the recursive 3D world structure and portal navigation.

## ⚗️ The Hybrid

-   **Structure**: A procedurally generated galaxy of nested rooms.
-   **Visuals**: Rooms are textured with `StarMap`s generated from their simulated content.
-   **Navigation**:
    -   **Portals**: Cube-shaped Star Clusters that transport you to inner rooms (sub-directories).
    -   **Data Stars**: Spheres representing files or data points.
-   **Steganography**: The texture on the walls is not random noise; it is data encoded as stars.

## 🎮 Controls

-   **W/A/S/D**: Move
-   **Space/Shift**: Up/Down
-   **Mouse**: Look
-   **Esc**: Quit

## 🧪 Mutation Notes

-   Fixed a genetic defect in `codex-void` where `OFFSETS` was missing from `glyph.rs`.
-   Spliced `StarMap` generation directly into the `World` generation.
-   Replaced simple colored walls with generated textures.
