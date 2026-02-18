# Turbulent Rhythms

**Lineage:**
- **Parent A:** `experiments/sync-opation` (Polyrhythmic Thread Contention)
- **Parent B:** `experiments/system-turbulence` (Fluid Dynamics driven by System Stats)
- **Concept:** A fluid simulation where the viscosity is controlled by system load, and thread synchronization events create turbulence.

**Mechanics:**
- **Rhythm:** Multiple threads ("Musicians") contend for a shared `Mutex` ("The Beat").
- **Fluid:** A compute-shader based fluid simulation (Navier-Stokes-ish Advection).
- **Interaction:**
    - When a Musician acquires the lock (plays a note), they inject dye and velocity into the fluid at their specific location.
    - The global system CPU usage controls the `viscosity` and `decay` parameters of the fluid.
    - High Load = Thicker, slower fluid.
    - Low Load = Watery, chaotic fluid.

**Controls:**
- Run and watch/listen.
- Increase system load (external) to see fluid thicken.
