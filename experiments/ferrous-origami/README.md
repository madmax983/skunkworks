# Ferrous-Origami 🧲📄

A hybrid experiment combining a continuous 2D magnetic substrate (`Platter`) with the dynamic 3D structural constraints of a procedural Miura-ori soft body mesh.

## 🧬 Lineage

This experiment is a cross between:
- **Parent A:** `crates/ferrous-core` (The continuous magnetic field `Platter` system)
- **Parent B:** `crates/origami` (The procedural Miura-ori tessellation and Position Based Dynamics logic)

## 🔬 Concept

**Magnetic Morphogenesis**

This experiment maps continuous fluid dynamics to discrete structural bounds. The 2D magnetic substrate dynamically warps and actuates the 3D structural constraints of the Miura-ori soft body mesh. As magnetic flux concentrates in an area, the physical constraints of the paper in that region expand or contract, causing the paper to bulge, warp, and deform dynamically based on the underlying field.

## 🕹️ Controls

- **Left Click:** Inject magnetic flux into the center of the substrate to trigger a wave of morphogenesis.

## 📦 Technical Details

- **Physics:** Position Based Dynamics (PBD) and Newtonian magnetic fields via `ferrous-core`.
- **Rendering:** Macroquad 3D.
