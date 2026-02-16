# Hyperbolic Rogue ⚛️🏰

**"The edge of the world is infinitely far away, yet you can reach it in a few steps."**

A roguelike engine set in the Poincaré Disk model of Hyperbolic Geometry.
This experiment combines **Hyperbolic Plane** geometry with **Infinite Dungeon Generation**.

## The Topology

- **Space**: Poincaré Disk Model (H²).
- **Tessellation**: {7,3} Heptagonal Tiling.
- **Movement**: Möbius Transformations (Hyperbolic Isometries).
- **Infinite**: The dungeon is generated procedurally as you move.

## Math

The world is infinite. The visible disk is just a projection.
As you move towards the edge, space expands.
To maintain numerical precision, the engine keeps the player at the origin (0,0) and moves the *universe* around them.
When you cross from one tile to another, the coordinate system "re-centers" on the new tile, allowing for truly infinite exploration without floating-point errors.

## Controls

- **WASD / Arrow Keys**: Move.
- **Movement is continuous**: You are applying a hyperbolic translation to your position.

## Implementation Details

- **Stack**: `macroquad`, `num-complex`.
- **Geometry**: Custom `Mobius` struct handling SU(1,1) transformations.
- **Generation**: A local patch of the {7,3} tiling is generated relative to the player. The "Identity" of each room is determined by hashing the cumulative transformation from the origin.

## Running

```bash
cargo run -p hyperbolic-rogue
```
