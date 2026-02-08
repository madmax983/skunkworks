# Hyperbolic Lexicon

**Genetic Lineage:**
- Parent A: `experiments/hyperbolic-raymarcher` (Non-Euclidean Geometry Engine)
- Parent B: `experiments/morph-physics` (Phonetic Particle System)
- Novel Trait: Linguistic Divergence in Infinite Space.

**Concept:**
Phonemes are particles drifting on the Poincaré Disk. The distance between sounds is measured using the hyperbolic metric. As languages evolve (drift), they move exponentially further apart in the linguistic space.

**Controls:**
- **WASD**: Pan the view (move the camera in hyperbolic space).
- **SPACE**: Trigger a phonetic shift (apply random drift vectors).

**Implementation Details:**
- Uses `poincare-disk` crate for Mobius transformations.
- Uses `macroquad` for rendering.
- A custom fragment shader renders the {4,5} tiling background.
- Phonemes are colored based on their features (Place, Manner, Voice).
