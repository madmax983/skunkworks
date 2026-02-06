# 🧬 Chimera Tardis

> "It's bigger on the inside." - The Splice Surgeon

**Chimera Tardis** is a hybrid experiment combining the evolutionary logic of `chimera-lang` with the recursive visualization of `trace-tardis`.

It visualizes the runtime call stack of a ChimeraVM organism as an infinite series of nested rooms. As the organism executes recursive genes, new rooms appear, allowing you to fly into the depths of the execution context.

## 🔬 Lineage

- **Parent A**: `experiments/chimera-lang` (Nova) - Provides the biological VM, stack, and genetic code.
- **Parent B**: `experiments/trace-tardis` - Provides the recursive rendering engine and "infinite zoom" camera logic.

## 🕹️ Controls

- **WASD / Arrows**: Move the camera (Pan).
- **+ / -**: Zoom In / Out.
- **Space**: Pause / Resume execution.
- **F**: Increase simulation speed (Faster).
- **S**: Decrease simulation speed (Slower).

## 🧬 Genetic Code

The default organism runs a recursive countdown program that builds a stack depth of 50 before returning, creating a deep tunnel of execution frames to explore.

```
Strand 0: Call(1)
Strand 1: Recursive Function (Decrements arg, calls itself)
```

## 🧪 Observations

- **Emergent Trait**: "Tangible Recursion". You can physically navigate the call stack.
- **Visuals**: Each room represents a stack frame, colored by the Strand Index to visualize thread context.
