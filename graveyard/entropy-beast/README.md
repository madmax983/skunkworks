# Entropy Beast ☢️🦁

> "The beast lives, dies, and lives again." - GUESTBOOK

A visualization of data resilience and mutation. An "Archaeologist" algorithm attempts to reconstruct a `Creature` from a binary stream that is being actively corrupted by radiation (entropy).

## The Experiment

1.  **Life**: A `Creature` is generated (Head + Recursive Limbs).
2.  **Death**: The creature is serialized into a binary DNA format (`MAGIC` header, markers, checksums).
3.  **Entropy**: The binary data is subjected to random bit flips, byte swaps, drops, and insertions. The intensity is controlled by the user.
4.  **Resurrection**: The `Archaeologist` parser scans the corrupted stream for recognizable patterns (Headers, Markers). It attempts to rebuild the creature.
    - If a child limb's data is corrupted, it might be detached.
    - If a marker is found out of place, a limb might be attached to the wrong joint.
    - Parameter mutation (size, length) occurs due to bit flips in `f32` fields.

## Controls

-   **Left/Right Arrows (or A/D)**: Decrease/Increase Entropy (Radiation Level).
-   **R**: Regenerate a new original creature.
-   **H**: Toggle Hex Dump view.

## Emergent Behavior

-   **Chimera Limbs**: Limbs intended to be children often detach and become root limbs when the hierarchy structure is corrupted.
-   **Glitch Geometry**: Corrupted floats create impossible geometry.
-   **Resilience**: The creature rarely disappears completely; it just becomes more monstrous.
