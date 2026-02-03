# CPU Chaos Monitor ⚛️⛈️

**Genesis Experiment: The Meteorologist**

> "Is your system load random noise, or deterministic chaos?"

## 🔬 Experiment Analysis

This experiment applies **Chaos Theory** to computer system metrics. Specifically, it uses **Time-Delay Embedding** (Takens' Theorem) to reconstruct the phase space of your CPU usage history and estimates the **Largest Lyapunov Exponent (LLE)** to determine if the system is chaotic.

### Concept

1.  **Data Source**: Global CPU usage is sampled at 20Hz.
2.  **Phase Space Reconstruction**:
    We take a single time series $x(t)$ (CPU load) and reconstruct a 3D trajectory:
    $$ \vec{v}(t) = [x(t), x(t-\tau), x(t-2\tau)] $$
    where $\tau$ is the time delay.
    If the system has a hidden attractor, this reconstruction preserves its topology.
3.  **Lyapunov Analysis**:
    We measure the rate at which initially close trajectories diverge over time.
    $$ |\delta Z(t)| \approx e^{\lambda t} |\delta Z_0| $$
    - $\lambda > 0$: **Chaotic** (Butterfly Effect).
    - $\lambda \le 0$: **Stable** (Periodic or Fixed Point).

## 🕹️ Controls

- **Q**: Quit
- **Space**: Pause/Resume Analysis
- **R**: Reset History
- **Up/Down**: Adjust Delay ($\tau$) parameter.

## 📊 Interpretation

- **Visuals**: You will see a 3D point cloud rotating.
    - A simple loop/circle indicates periodic load.
    - A fuzzy ball indicates random noise.
    - A structured, folded shape (like a butterfly wings) indicates a **Strange Attractor**.
- **Metrics**:
    - **Lyapunov Exponent**: Positive values indicate chaos.
    - **Status**: CHAOTIC / STABLE.

## Tech Stack
- `ratatui`: TUI Rendering
- `sysinfo`: System Metrics
- `chaos.rs`: Custom implementation of embedding and Rosenstein's algorithm.
