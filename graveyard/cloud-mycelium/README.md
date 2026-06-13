# Cloud Mycelium 🍄☁️

**"Mycelium nutrient sharing + Load balancer simulation"**

A visual simulation of a "Server Forest" where load balancing is performed by a fungal network.

## Concept
Servers are represented as Mushrooms.
- **Mushrooms**: Receive "Rain" (Requests/Load).
- **Network**: Mycelium connects roots.
- **Action**: When a mushroom is overloaded (Red), it shunts excess load into the mycelium as "Packets" (Spores) to be transported to underloaded neighbors.

## Visuals
- **Mushrooms**: Pulse and change color (Green -> Red) based on load.
- **Mycelium**: Glowing threads connecting the forest.
- **Packets**: Yellow particles moving along hyphae.
- **Rain**: Cyan particles falling from the cloud.

## Controls
- **Click**: Spawn a new Mushroom Server.
- **Space**: Trigger a "Storm" (High Load / DDoS).
- **R**: Reset the simulation.

## Tech Stack
- **Rust**
- **Macroquad** (Visualization)
- **Glam** (Vector Math)
