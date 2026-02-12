# Harmonic Engine

> "A mechanical computer that sings the differential equations it solves."

**Genesis: The Horologist**

This experiment simulates a **Mechanical Differential Analyzer** using `rapier2d` physics.
It constructs a system of **Ball-and-Disk Integrators** coupled together to solve differential equations physically.
The output of the integrators drives a **Music Box** mechanism, where the rotation of the solution cylinder plucks teeth to generate sound.

## Mechanism

### The Ball-and-Disk Integrator
The core component is the Ball-and-Disk integrator, a mechanical device used in early analog computers (like the Bush Differential Analyzer).
1.  **Input Disk**: Rotates at a constant speed (representing Time $t$).
2.  **Ball**: Positioned at distance $r$ from the center. This represents the value of the integrand $y$.
3.  **Output Cylinder**: Driven by the ball via friction. Its angular velocity $\omega_{out}$ is proportional to the disk speed $\omega_{in}$ and the ball position $r$.
    $$ \omega_{out} = k \cdot \omega_{in} \cdot r $$
    Since $\omega_{in}$ is constant, the total rotation $\theta_{out} = \int \omega_{out} dt \propto \int r dt$.
    Thus, the cylinder integrates the ball's position.

### The Harmonic Oscillator
We couple two integrators to solve the harmonic oscillator equation:
$$ y'' = -y $$
Or as a system of first-order equations:
$$ y' = v $$
$$ v' = -y $$

-   **Integrator 1 (y)**: Ball position is driven by $v$ (output of Integrator 2). Output is $y$.
-   **Integrator 2 (v)**: Ball position is driven by $-y$ (output of Integrator 1). Output is $v$.

The result is a sinusoidal oscillation of the ball positions and cylinder rotations.

### The Music Box
The Output Cylinders are equipped with virtual "pins". As they rotate, these pins strike the teeth of a comb, generating notes.
The melody produced is a direct sonification of the solution curve.

## Controls
-   **Run**: `cargo run -p harmonic-engine`
-   **Quit**: Press `q`.

## Visualization
The TUI displays the rotating disks and the sliding balls.
-   **White Circle**: Input Disk.
-   **Red Dot**: The Ball (Value).
-   **Yellow Line**: The Output Cylinder (Rotation).
-   **Green Line**: The Mechanical Coupling (Data Flow).

## Stack
-   **Physics**: `rapier2d` (Rigid Body Dynamics + Kinematic Constraints).
-   **Rendering**: `ratatui` (Terminal UI).
-   **Input**: `crossterm`.
