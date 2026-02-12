# Biomorphic Strings 🧬🎻

A hybrid experiment combining 3D string physics with reaction-diffusion biology.

## 🧬 Concept

What if the fundamental strings of the universe were alive?
This simulation models a "Cosmic String" as a physical mass-spring system, but coats it in a biological reaction-diffusion medium (Gray-Scott model).

### Lineage
- **Parent A**: `experiments/cosmic-strings` (3D Physics)
- **Parent B**: `experiments/biomorphic-clock` (Reaction-Diffusion)

### Emergent Traits
- **Heavy Metal Biology**: The concentration of the chemical "activator" (`v`) increases the physical mass of the string nodes.
- **Metabolic Tension**: The physical stretching of the string accelerates the metabolic rate (feed rate) of the reaction.

## 🎮 Controls

- **Space**: Pluck the string (inject physical energy).
- **R**: Reset the simulation.
- **Q**: Quit.

## 🔬 Physics & Chemistry

The string is simulated using Semi-Implicit Euler integration.
The chemistry is a 1D Gray-Scott system:
$$ \frac{\partial u}{\partial t} = D_u \nabla^2 u - uv^2 + f(1-u) $$
$$ \frac{\partial v}{\partial t} = D_v \nabla^2 v + uv^2 - (f+k)v $$

Where $f$ (feed rate) is modulated by the local tension of the string.
And the node mass $m$ is modulated by $v$: $m = m_0 (1 + 5v)$.
