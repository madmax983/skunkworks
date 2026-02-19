# Tardis Memory ⚛️🌌

> "It's bigger on the inside."

A 3D visualization of memory (or a file system) where containers are physically smaller than their contents. Implements **Seamless Portal Rendering** to allow looking into and walking into these "Tardis" spaces.

## Concept
Standard memory visualizations show hierarchies as trees or nested boxes of decreasing size. `tardis-memory` breaks Euclidean geometry:
- **Outer View**: A struct is a small box.
- **Inner View**: Walking into the box reveals a massive room containing its fields.
- **Portals**: The face of the box is a real-time portal window into the inner room.

## Controls
- **WASD**: Move horizontally.
- **Space / Shift**: Fly Up / Down.
- **Mouse**: Look around.
- **ESC**: Exit.

## Tech Stack
- **Rust**
- **Macroquad** (0.4)
- **Portal Rendering**: Uses `render_target` to capture views of inner rooms and map them to the faces of outer blocks.
- **Teleportation**: Seamlessly transports the player coordinate system when crossing a portal threshold.

## Status
- [x] Basic World Generation (Recursive Heap)
- [x] Portal Rendering (Immediate Mode)
- [x] Seamless Teleportation
- [ ] Recursive Portals (Depth > 1)
- [ ] Live Memory Inspection (ptr scanning)
