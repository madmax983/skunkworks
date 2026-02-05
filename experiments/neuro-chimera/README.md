# Neuro-Chimera 🧬🧠

> "The axons are strands of DNA. The dendrites are stack pointers. The thought is a program." - The Splice Surgeon

**Neuro-Chimera** is a hybrid experiment combining the visualization of `neuro-terminal` with the biological virtual machine of `chimera-lang`.

## Lineage
- **Parent A**: `experiments/neuro-terminal` (Neural Network structure & Visualization)
- **Parent B**: `experiments/chimera-lang` (Genetic VM & DNA Execution)

## Concept
In a traditional neural network, neurons compute a weighted sum passed through an activation function (like sigmoid or ReLU).
In **Neuro-Chimera**, each neuron contains a **Chimera VM**.

- **Inputs**: Encoded onto the VM's Grid.
- **Processing**: The VM executes its unique Genome (DNA) for a fixed number of ticks.
- **Output**: The value is popped from the VM's stack.
- **Learning**: Instead of Backpropagation (Gradient Descent), the network evolves via a **Genetic Algorithm**.

## Emergent Behavior
The network does not just "learn weights". It evolves *programs* to solve the classification task. A neuron might evolve to perform multiplication, or conditional jumps, or even ignore inputs based on evolved logic.

## Usage

```bash
cargo run -p neuro-chimera
```

- **Q**: Quit
- **P**: Pause/Resume evolution

## Visualization
- **Left**: The Decision Boundary (Cyan vs Background) showing how the network classifies 2D space.
- **Right**: The Network Topology.
- **Top**: Real-time loss history (Mean Squared Error).
