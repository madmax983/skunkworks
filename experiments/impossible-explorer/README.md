# Impossible Explorer ⚛️🗺️

> "The file system is bigger on the inside."

A non-Euclidean file system explorer where directories are rendered as rooms connected by portals. Features recursive rendering and "Tardis" geometry.

## Concept
-   **Portals**: Every subdirectory is a door. Looking through the door reveals the contents of that directory.
-   **Non-Euclidean**: Rooms can be larger than the space containing them. If you walk through a door, you are seamlessly transported to the new room.
-   **Recursion**: Explore the file system by walking through doors. The world behind the door is rendered using `glScissor` clipping to create a window into another dimension.

## Controls
-   **WASD**: Move
-   **Mouse**: Look
-   **Space / Shift**: Fly Up / Down
-   **Esc**: Quit

## Technical Details
-   **Stack**: `macroquad`, `glam`.
-   **Rendering**: Recursive portal rendering using `glScissor`.
    -   The target room is rendered relative to the portal frame.
    -   Recursion depth is limited to 2 levels for performance.
-   **Navigation**:
    -   Collision with a portal triggers a teleport.
    -   Camera Yaw is adjusted to align with the new room's coordinate system.
    -   "Tardis" effect: You walk into a box and are now inside a room.

## How to Run
```bash
cargo run --bin impossible-explorer
```
The experiment starts in the current working directory.
