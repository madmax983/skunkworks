# Heap Arena 🏟️

**Lineage:** `func-arena` × `heap-hopper`

A hybrid experiment combining RPG code metrics with memory-management platforming.

## 🧬 Concept

You are the **Garbage Collector**. Your mission is to reclaim memory from complex, bloated functions.
The function is the level. Its lines of code form the ground you walk on.
The function is also the **Boss**, floating at the end of its memory allocation, throwing exceptions at you.

## 🎮 Mechanics

- **Terrain Generation**: The level layout is procedurally generated from the source code of a random function in the target directory.
  - `let` / `fn`: Solid ground.
  - `unsafe` / `panic!`: Hazard blocks (Red).
  - Loops: Bouncy blocks (Blue).
  - Empty lines: Gaps (Fall = Segfault).

- **Boss Stats**:
  - **HP**: Based on Lines of Code.
  - **Attack**: Based on Cyclomatic Complexity (determines projectile fire rate).
  - **Defense**: Based on Argument count.

- **Objective**:
  - Survive the traversal of the function's heap.
  - Dodge "Exception" projectiles.
  - Reach the Return Address (end of level) to "Collect" the function.

## 🕹️ Controls

- **WASD / Arrows**: Move and Jump.
- **Q / ESC**: Quit.

## 🚀 Usage

```bash
cargo run -p heap-arena -- [path_to_scan]
```

Example:
```bash
cargo run -p heap-arena -- experiments/heap-arena/src
```
