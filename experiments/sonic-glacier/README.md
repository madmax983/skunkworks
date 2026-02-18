# Sonic Glacier ⚛️❄️

**Hybrid Experiment**: `sonic-viscosity` × `heap-glacier`

A thermodynamic battle where memory allocations generate heat (melting the landscape) while audio frequencies freeze it back into crystalline structures.

## Lineage

- **Parent A**: `experiments/sonic-viscosity`
    - Contributed: Audio analysis engine (FFT), Spectrum data structure.
    - Allele: `SonicEngine`
- **Parent B**: `experiments/heap-glacier`
    - Contributed: 3D Heap Terrain visualization (Bedrock, Ice, Water).
    - Allele: `HeapTerrain`

## Novel Traits

- **Thermodynamic Sonification**: The music is the cooling agent. Without music, the world melts under the pressure of memory allocations.
- **Phase Transitions**: Water flows hydraulically but freezes into static ice when the bass drops (or the treble spikes).

## Controls

- **WASD + Arrows**: Move camera.
- **Space/Shift**: Up/Down.
- **H**: Inject Heat (Simulate Allocation / GC Pressure).
- **M**: Toggle Auto-Heat Mode.

## Run

```bash
cargo run -p sonic-glacier --features audio
```
(Without `--features audio`, it runs in simulation mode with dummy data)
