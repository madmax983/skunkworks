# Storm Monitor ⛈️

> "The flutter of a butterfly's wing can cause a typhoon halfway across the world."

**Storm Monitor** is a chaos visualization experiment that maps your system's metabolic state (CPU, Memory, IO) to the parameters of a [Lorenz Attractor](https://en.wikipedia.org/wiki/Lorenz_system).

It attempts to answer the question: *What is the shape of your computer's chaos?*

## The Science

The Lorenz system is defined by:

$$
\begin{aligned}
\frac{dx}{dt} &= \sigma (y - x) \\
\frac{dy}{dt} &= x (\rho - z) - y \\
\frac{dz}{dt} &= xy - \beta z
\end{aligned}
$$

In **Storm Monitor**, these parameters are driven by your hardware:

*   **$\sigma$ (Sigma)**: The Prandtl number, representing viscosity/volatility.
    *   Mapped to **CPU Load**.
    *   High CPU = High Sigma = More chaotic, rapid mixing.
*   **$\rho$ (Rho)**: The Rayleigh number, representing instability/buoyancy.
    *   Mapped to **Memory Usage**.
    *   High Memory = High Rho = The system is "heavier", pushing the attractor into new regimes (bifurcation).
*   **$\beta$ (Beta)**: Geometric aspect ratio.
    *   Mapped to **Swap/IO**.
    *   Changes the "shape" or decay rate of the attractor.

## Running

```bash
cargo run -p storm-monitor
```

## Controls

*   **Q**: Quit the simulation.

## Architecture

*   **Core**: RK4 Numerical Integrator (`lorenz.rs`).
*   **Sensors**: `sysinfo` crate (`monitor.rs`).
*   **Visuals**: `ratatui` for TUI rendering.
*   **Parallelism**: `rayon` for updating thousands of particle trajectories simultaneously.
