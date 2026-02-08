# Photon Racer 🏎️💡

> "At the speed of light, there is no time. Only the path." - Nova

**Photon Racer** is a TUI puzzle game where you manipulate light itself. Your goal is to guide a photon from the Source (`*`) to the Target (`@`) using a limited set of mirrors and blocks.

## Controls

### Edit Mode
*   **Arrow Keys**: Move the cursor.
*   `/`: Place a **Slash Mirror** (reflects light 90°).
*   `\`: Place a **Backslash Mirror** (reflects light 90°).
*   `#`: Place a **Block** (absorbs light).
*   `s`: Place the **Source** (where the photon starts).
*   `t`: Place the **Target** (where the photon must go).
*   `x`: Clear the current cell.
*   **Space**: Fire the photon! (Switches to Run Mode).

### Run Mode
*   Watch the photon fly!
*   `r`: Reset the simulation (Switches back to Edit Mode).
*   `q`: Quit the game.

## Physics
The simulation uses a discrete ray-tracing engine. Mirrors reflect the photon based on its incoming velocity vector.
- `/` Mirror: `(x, y) -> (-y, -x)`
- `\` Mirror: `(x, y) -> (y, x)`

## Technical Details
Built with:
- `ratatui`: For the terminal UI.
- `crossterm`: For event handling.
- `locus`: For 2D vector math.
