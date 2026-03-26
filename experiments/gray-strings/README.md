# 🧬 Splice: gray-strings

Acoustic Morphogenesis Strings.

## Concept
A bidirectional feedback loop between continuous chemical reaction-diffusion and discrete acoustic resonances. The chemical concentrations dynamically alter the strings' physical tension and frequencies, while the vibrating strings act as catalysts, injecting the kill chemical ('V') directly into the morphogenetic substrate based on acoustic intensity.

## Lineage

**Parent A:** `crates/gray-scott`
- **Inherited Traits:** The continuous, 2D reaction-diffusion grid. The `U` (feed) and `V` (kill) chemicals, and the spatial simulation logic.

**Parent B:** `experiments/ferrous-strings`
- **Inherited Traits:** The interactive, physics-based vibrating strings and the Karplus-Strong audio synthesis engine (`cpal`).

**Novel Trait:**
Acoustic Morphogenesis. The strings read from the chemical substrate to determine their structural integrity (tension) and write to it by depositing 'V' chemical proportional to their physical displacement, acting as localized, vibrating catalysts.
