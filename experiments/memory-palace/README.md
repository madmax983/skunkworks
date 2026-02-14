# Memory Palace ⚛️🧠🏛️

> "The door is small, but the room is infinite."

**Memory Palace** is a Moonshot experiment combining **Tardis Spaces** (Non-Euclidean geometry) with **Memory Visualization**.

It visualizes a mock memory heap where allocations are physical rooms. Pointers are portals (doors) connecting these rooms.

## Concept

- **Tardis Effect**: A pointer (8 bytes) is a small doorway. The allocation it points to (e.g., a large struct) is a huge room. When you look through the door, you see the full-sized room inside, defying Euclidean geometry.
- **Portals**: Seamless transitions between memory spaces. Walking through a pointer teleports you to the destination allocation.
- **Recursion**: Recursive structures (like linked lists or trees) form infinite hallways.
- **Cycles**: Circular references create loops in space.

## Controls

- **WASD**: Move around.
- **Mouse**: Look around.
- **Space**: Fly Up.
- **Shift**: Fly Down.

## Tech Stack

- **Rust**
- **Macroquad** (3D Rendering)
- **Portal Rendering**: Uses render-to-texture and coordinate transformation to simulate non-Euclidean connectivity.

## Status

- [x] Basic Portal Rendering (CCTV style)
- [x] Teleportation Logic
- [x] Mock Heap Generation
- [ ] Recursive Portal Rendering (Mirrors/Infinite Hallways)
- [ ] Real-time Heap Analysis (ptracing a running process)

## Notes

The "Tardis" effect is achieved by rendering the destination room from a "virtual camera" that matches the player's perspective relative to the portal. This allows small openings to reveal large spaces.
