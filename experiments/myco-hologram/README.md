# 🍄 Myco-Hologram: Pheromone Spectral Interference

A hybrid experiment combining `myco-transit` and `hologram-text`.

## 🧬 Concept

Slime mold agents forage for food in a 2D space, depositing and following pheromone trails. However, rather than simply visualizing the physical pheromone grid, `myco-hologram` treats the entire pheromone map as an optical interference pattern. It computes the 2D Fast Fourier Transform (FFT) of the grid to generate a "holographic" projection.

You are watching the Fourier transform of a biological system in real time. The resulting visualization exposes the spectral/resonant frequencies of the organism's foraging pathways.

## 🔬 Lineage

- From `myco-transit`: Pheromone grid, agent foraging, decay logic, and `World` structures.
- From `hologram-text`: The `Hologram` component, 2D FFT, frequency domain reconstruction, and interference simulation.

## 🎮 Interaction

- Arrow Keys: Change reconstruction angle. This simulates viewing the hologram from different physical angles, shifting the reconstructed frequency domain.
- Space: Reset reconstruction angle to default.
- Q/Esc: Quit.
