# Neuro-Flock 🧠🐦

> "What if the brain could fly?"

**Neuro-Flock** is a hybrid experiment combining flocking boids with spiking neural networks.

## 🧬 Lineage

- **Parent A**: `experiments/luminous-flock` (TUI Flocking simulation)
- **Parent B**: `experiments/beat-cortex` (Izhikevich Spiking Neural Network + Audio)
- **Hybrid Vigor**: The "synapses" of this neural network are dynamic. Connections are formed only when "neurons" (boids) are physically close to each other.

## 🔬 How it Works

1. **Mobile Neurons**: Each agent in the flock carries an internal Izhikevich neuron model.
2. **Spatial Synapses**: When boids come within `coupling_radius`, they form a temporary synaptic connection.
3. **Spike Propagation**: If a boid spikes (fires), it injects current into its neighbors.
4. **Sonification**: Each boid has a unique `base_freq` (DNA). When it spikes, it plays a tone.
5. **Emergence**: The flock's movement determines the network topology. A dense swarm creates a highly connected, noisy brain. A dispersed flock creates silence.

## 🎮 Controls

- `q` or `Esc`: Quit
- `r`: Reset simulation

## 📦 Run

```bash
# Visuals only (Silent)
cargo run -p neuro-flock

# With Audio (Requires ALSA on Linux)
cargo run -p neuro-flock --features audio
```

## 🧠 Model Details

Each boid simulates:
$$v' = 0.04v^2 + 5v + 140 - u + I$$
$$u' = a(bv - u)$$

Where $I$ (Input Current) is the sum of:
- Background noise (random)
- Synaptic input from neighbors who spiked in the previous frame ($W \approx 20.0$)
