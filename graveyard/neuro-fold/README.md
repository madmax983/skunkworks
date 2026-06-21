# Neuro-Fold 🧠📄

A hybrid experiment combining Spiking Neural Networks (SNN) with Position Based Dynamics (PBD) to create "living" origami structures.

## 🧬 Lineage

This experiment is a cross between:
- **Parent A:** `experiments/neuro-crab` (Spiking Neural Networks, Izhikevich Model)
- **Parent B:** `experiments/origami-constellation` (Rigid Origami Simulation, PBD)

## 🔬 Concept

The simulation creates a **Miura-ori** folding sheet where the creases are actuated by "muscles". These muscles are controlled by a **Central Pattern Generator (CPG)** formed by a chain of spiking neurons.

- **Actuation:** Neurons are mapped to vertical creases. When a neuron spikes/activates, it contracts the crease (folds it).
- **Proprioception:** The physical strain on the sheet (difference between current length and target length) is fed back into the neural network as sensory input.
- **Emergence:** The system exhibits rhythmic "breathing" or "peristaltic" motions driven by the CPG, which adapts to the physical constraints of the mesh.

## 🕹️ Controls

- **Mouse Drag:** Rotate camera.
- **Scroll:** Zoom.
- **Left Click:** Inject current (noise) into random neurons to excite the system.

## 📦 Technical Details

- **Physics:** Position Based Dynamics (PBD) with Distance and Actuator constraints.
- **Brain:** Izhikevich Neurons with synaptic delays (simplified) and STDP (potential extension).
- **Rendering:** Macroquad 3D.
