# System Choreography ⚛️💃

**Genesis: The Choreographer**

> *The machine is dancing. Its muscles are memory pages, its heartbeat is the clock cycle.*

This experiment visualizes the real-time performance of your computer as a procedurally animated dancer.
The movement is driven by **Laban Movement Analysis** parameters derived from system metrics.

## The Language of Motion

We map the computer's internal state to the dancer's effort qualities:

- **Weight (RAM Usage)**:
  - **Light (Low RAM)**: The dancer is buoyant, upright, and bouncy.
  - **Heavy (High RAM)**: The dancer is grounded, knees bent, movements are labored.

- **Time (CPU Load)**:
  - **Sudden (High CPU)**: Movements are fast, jittery, and energetic.
  - **Sustained (Low CPU)**: Movements are slow, fluid, and relaxed.

- **Space (Activity)**:
  - **Direct (Idle)**: Movements are small and focused.
  - **Indirect (Busy)**: Arms sweep in wide arcs, claiming space.

- **Flow (System Stability)**:
  - **Free (Stable)**: Interpolation is smooth and continuous.
  - **Bound (Volatile)**: Interpolation is snappy and rigid.

## Implementation

- **Inverse Kinematics**: A custom analytical 2-bone solver positions the limbs based on procedural targets.
- **Procedural Choreography**: A sequencer generates dance moves (oscillations, steps) that are modulated by the Laban parameters.
- **System Monitoring**: `sysinfo` polls the OS kernel for performance data.

## Usage

Run the visualization:

```bash
cargo run -p sys-dance
```

Make your computer dance by opening Chrome tabs (Weight) or compiling Rust code (Time).
