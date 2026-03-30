# 🧬 Chimera Diffusion

**Parents:** `experiments/chimera-lang` × `crates/gray-scott`

A hybrid experiment where ChimeraVM agents ("Morphogenetic Beings") inhabit a Reaction-Diffusion substrate, actively altering their environment.

## 🔬 Concept

In this simulation, agents exist inside a continuous, chemical grid (the Gray-Scott Reaction-Diffusion model). The world is not an empty void, but a dynamic, morphogenetic medium.

-   **Agents:** Execute genetic code (`Dna`) representing biological intent and pathfinding logic.
-   **Environment:** A chemical grid running the Gray-Scott model (chemicals U and V).
-   **Interaction:** Agents move across the chemical grid. As they navigate, they actively deposit the `V` (kill) chemical, altering the Turing patterns.

## 🧬 Emergent Trait: Morphogenetic Foraging

By excreting the "kill" chemical `V` as they move:
1.  **Sustenance/Decay loops:** The agents create active trails of the chemical that diffuse, turning their path into a morphogenetic catalyst that reshapes the entire Turing pattern surrounding them.
2.  **Environment Shapers:** The agents aren't just adapting to the environment, they are the primary architects of its structural changes, simulating an organism modifying its own niche.

## 🕹️ Controls

-   **Q**: Quit the simulation.

## 📦 Lineage

-   **Chimera Lang:** Provides the biological VM, DNA execution, and organism structure.
-   **Gray-Scott:** Provides the continuous, 2D reaction-diffusion mathematical substrate.
