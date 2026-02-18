# Chimera Erosion 🧬💧

> "The river carries the seeds of its own reshaping."

A hybrid experiment combining hydraulic erosion simulation with biological evolution.

## 🧬 Lineage

This experiment is a cross between:

*   **Parent A:** `experiments/vascular-valley` (Hydraulic Erosion)
*   **Parent B:** `experiments/chimera-lang` (Genetic Programming VM)

## 🧪 Concept

Water droplets in the simulation not only erode the terrain but also transport genetic material ("Spores"). When sediment settles, these spores can germinate into `ChimeraVM` instances (Plants).

These plants execute genetic code (Chimera DNA) that allows them to:
*   **Sense:** Read local terrain height, water level, and soil quality.
*   **Act:** Grow roots (stabilize soil against erosion), grow leaves (collect water/energy), or release more spores.
*   **Evolve:** Successful plants spread their DNA downstream.

## 🕹️ Controls

*   `Space`: Toggle Erosion/Simulation.
*   `R`: Reset Terrain.
*   `Click`: Rain (Manual).
*   `D`: Toggle Debug View (Plants vs Terrain).

## 🏗️ Architecture

*   **Erosion Engine:** Ported from `vascular-valley`. Uses droplet-based hydraulic erosion.
*   **Bio Engine:** Sparse grid of `Plant` entities, each owning a `ChimeraVM`.
*   **Interaction:**
    *   `Erosion` -> `Bio`: Water flow provides energy and transport.
    *   `Bio` -> `Erosion`: Roots modify the hardness map, preventing erosion.
