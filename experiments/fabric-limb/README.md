# Fabric Limb (The Code Dancer) ⚛️💃

**"Motion emerges from rules."**

This experiment explores the intersection of **Inverse Kinematics (IK)** and **Code Navigation**.
Instead of a static highlight cursor, a multi-jointed robotic limb physically reaches for the file you select.

## Concept
The project implements the **FABRIK** (Forward And Backward Reaching Inverse Kinematics) algorithm to control a 4-segment chain. As you navigate the file tree, the "target" of the IK solver updates, and the arm smoothly interpolates its position to touch the file.

## Controls
- **Up / Down**: Navigate the file list.
- **Q / Esc**: Quit.

## Running
```bash
cargo run -p fabric-limb
```

## Stack
- **ratatui**: TUI rendering.
- **crossterm**: Event handling.
- **walkdir**: File system traversal.
- **tui-shared**: Terminal lifecycle management.
