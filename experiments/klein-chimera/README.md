# Klein Chimera 🧬🍩

**Parents**: `chimera-lang` (Parent A) + `klein-flock` (Parent B)

A hybrid experiment where `ChimeraVM` organisms inhabit a non-orientable surface (Klein Bottle).

## Concept

The world is a 2D rectangle where:
- The **X-axis** wraps normally (Cylinder).
- The **Y-axis** wraps with a twist (Mobius/Klein).

When an agent crosses the Y-boundary (Top/Bottom), its position is wrapped, its X-coordinate is inverted, and its **Chirality** (Left/Right Handedness) is flipped.

## Novel Trait: Chiral Inversion

Chimera organisms have an internal `Chirality` state (Left/Right). This affects how their enzymes (OpCodes) function (e.g., `Add` becomes `Sub`).
In this experiment, the topology of space forces this state to flip. An organism that relies on "Left" turns will suddenly find itself turning "Right" after traversing the world.

## Visuals

- **Cyan Agents (L)**: Left-Chiral (Normal)
- **Magenta Agents (R)**: Right-Chiral (Inverted)
- **Lines**: Velocity vectors. Note how they flip direction when crossing the twist.

## Controls

- `Q`: Quit
- `R`: Reset Population

## Lineage

- **From chimera-lang**: The genetic VM, evolution, and biological state.
- **From klein-flock**: The topological wrapping logic and TUI visualization structure.
