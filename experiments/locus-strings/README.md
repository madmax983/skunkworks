# Locus Strings

An interactive hybrid experiment blending continuous non-linear flocking behaviors (`crates/locus`) with discrete resonant magnetic strings (`experiments/ferrous-strings`).

## Phenotype

**Acoustic Swarming**

Boids swarm around magnetic strings. Their continuous flocking motion plucks the discrete resonant strings, generating a dynamic audio-visual feedback loop. The sound waves translate back into magnetic fields, physically perturbing the flock.

## Dependencies
Requires `libasound2-dev` on Linux for audio playback.

## Usage

```bash
cargo run -p locus-strings
```
