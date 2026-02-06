# Laban Rover 💃🚙

**Lineage:** `repo-rover` × `laban-machine`

A hybrid experiment combining space exploration with Laban Movement Analysis (LMA). The rover is not just a vehicle; it is a dancer, and the filesystem is its stage.

## Concept

The rover navigates your filesystem, but its physics are determined by the "atmosphere" of the current directory, mapped to Laban Effort parameters:

*   **Weight (Heavy vs Light):** Determined by **File Size**.
    *   *Heavy:* High friction, slow acceleration. Hard to move.
    *   *Light:* Low friction, gliding movement.
*   **Time (Sudden vs Sustained):** Determined by **File Age**.
    *   *Sudden (New Files):* Jerky, high-impulse acceleration.
    *   *Sustained (Old Files):* Smooth, consistent speed.
*   **Space (Direct vs Indirect):** Determined by **Directory Depth**.
    *   *Direct (Shallow):* Precise turning.
    *   *Indirect (Deep):* Wandering, jittery turning.
*   **Flow (Bound vs Free):** Random variation / Life.
    *   *Bound:* Rigid control.
    *   *Free:* Loose, fluid control (higher max speed).

## Controls

*   **Arrow Keys / WASD:** Move and Rotate.
*   **Enter:** Dive into a directory.
*   **Space:** Scan nearby objects (HUD message).
*   **Esc / Q:** Quit.
*   **+/-:** Adjust zoom.

## The Hybrid Vigor

This experiment demonstrates how abstract movement theory can be applied to generic UI navigation physics to create a "texture" for data. Navigating a folder of large, old ISOs feels physically different from navigating a source folder of tiny, new scripts.
