# Logic Gears ⚛️⚙️

**Genesis: The Horologist**

A simulation of mechanical logic gates using gear differentials, built with Bevy and Rapier 2D.
This experiment combines **Mechanical Calculation** with **Physically Accurate Gear Trains**.

## The Mechanism
The core component is a **Differential Adder**, constructed from:
*   **Input Racks (A & B)**: Vertical sliding bars driven by user input.
*   **Pinion Output**: A central gear that meshes with both input racks.
*   **Floating Axle**: The pinion's axle is free to move vertically.

### The Math
The vertical position of the Pinion ($Y_{out}$) is the average of the Input positions ($Y_A, Y_B$):
$$ Y_{out} = \frac{Y_A + Y_B}{2} $$
This implements a continuous mechanical Adder.

## Running
```bash
cargo run -p logic-gears
```

### Controls
*   **Up/Down Arrows**: Move Input A (Left Rack)
*   **W/S Keys**: Move Input B (Right Rack)
*   **Observe**: The center Green Gear moves up if either input moves up. If both move up, it moves up twice as fast (relative to single input contribution).

## Verification
A headless test verifies the physical logic:
```bash
cargo test -p logic-gears
```
This test spawns the physics world, mechanically drives the input rack, and asserts that the output rack moves the correct distance via gear interaction.

## Genesis Notes
"If you can hear it tick in your mind, you've succeeded."
The gears are approximated with compound colliders (teeth) to ensure true mechanical interlocking, not just kinematic constraints.
