# Clockwork CPU ⚛️⏱️

A "Moonshot" experiment by Genesis: The Horologist.

## Concept
This experiment visualizes a CPU clock frequency not as a quartz crystal oscillation, but as a mechanical **Verge Escapement**—the earliest known mechanical escapement, used in clockwork from the 13th to the 17th century.

It simulates the physical interaction between a **Crown Wheel** (driven by a constant torque, representing "Voltage") and a **Verge & Foliot** (the regulator, representing the clock crystal).

The "Tick" of the CPU is physically determined by the moment of inertia of the foliot and the torque of the crown wheel.

## Controls
*   **UP Arrow**: Increase Voltage (Torque). Makes the clock run faster (higher Hz).
*   **DOWN Arrow**: Decrease Voltage. Slows down the clock.

## The Physics
The simulation uses a custom rigid body physics engine (`src/physics.rs`) to model:
*   **Crown Wheel**: Driven by torque, has inertia.
*   **Verge**: Has pallets that collide with the crown wheel teeth.
*   **Collision**: Impulse-based resolution. The crown wheel pushes the verge out of the way, transferring energy (Voltage -> Frequency).

## Visualization
*   **Yellow Wheel**: The Crown Wheel.
*   **Gray Bar**: The Foliot (Oscillator).
*   **Green/Gray Dots**: The Pallet interaction points (Top and Bottom).
*   **Green Oscilloscope**: Real-time graph of the Verge's angular velocity, simulating the CPU clock signal.
