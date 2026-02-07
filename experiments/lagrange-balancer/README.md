# Lagrange Balancer ⚛️🔭⚖️

> "If the load is too high, just spin the servers faster until the requests get stuck in the L4/L5 points." — Genesis (The Astronomer)

**Lagrange Balancer** is a Moonshot experiment visualizing server load balancing as a gravitational N-body problem in a rotating reference frame.

## Concept

Imagine a load balancer that doesn't use queues or round-robin logic, but instead uses **Orbital Mechanics**.
*   **Requests** are massless particles spawned at the Gateway (center).
*   **Servers** are massive bodies that attract requests.
*   **The System Rotates**, creating Centrifugal and Coriolis forces.

At the right rotation speed ($\Omega$), the interplay of Gravity and Centrifugal force creates **Lagrange Points** (L1, L2, L3, L4, L5) — stable equilibria where particles can get trapped.

By tuning $\Omega$, you can potentially buffer requests in these gravitational eddies before they fall into a server, effectively creating "Orbital Queues".

## Physics

The simulation uses a symplectic integrator (Velocity Verlet) in a rotating frame:

$$ \vec{F} = \vec{F}_{gravity} + \vec{F}_{centrifugal} + \vec{F}_{coriolis} $$

*   **Gravity:** $\vec{F}_g = -G \sum \frac{M_i}{|\vec{r}_i|^3} \vec{r}_i$ (Attraction to servers)
*   **Centrifugal:** $\vec{F}_{cen} = m \Omega^2 \vec{r}$ (Outward push)
*   **Coriolis:** $\vec{F}_{cor} = -2m \vec{\Omega} \times \vec{v}$ (Deflection based on velocity)

## Controls

*   **Left / Right Arrow**: Decrease / Increase Rotation Speed ($\Omega$).
*   **Click (Left)**: Spawn a new Server (massive body) at the mouse position.
*   **Requests**: Automatically spawned at the center.

## Running

```bash
cargo run -p lagrange-balancer
```

## Observations

*   At $\Omega = 0$, requests just fall into the nearest server (Gravity only).
*   As $\Omega$ increases, requests spiral out.
*   At high $\Omega$, requests might be flung out to infinity before hitting a server.
*   At "Critical $\Omega$", stable orbits or Lagrange traps may form.
