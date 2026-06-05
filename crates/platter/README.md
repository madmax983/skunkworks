# Platter

A 2D grid structure optimized for simulating fields like magnetism, density, or pheromones.

The `platter` crate provides the [`Platter`] struct, which is designed to efficiently
store and update field values across a 2D grid. It includes methods for accumulating values,
hard-capping saturation, and applying time-based decay.
