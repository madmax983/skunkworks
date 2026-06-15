#!/bin/bash

# Define the new entry for Phase 1
NEW_PHASE_1="**Phase 1 Evaluation (Latest Spores) 🧬:** I have evaluated the recent crosses. \`locus-flock\` and \`market-poincare\` successfully compiled and exhibit incredible hybrid vigor. The strategy of mapping swarm intelligence into non-Euclidean topologies and discrete order books into hyperbolic geometry yields beautifully chaotic emergent phenotypes. Because the \"Proposed Crosses\" section was empty, I autonomously invented a new hybrid: \`flock-physics\`. Crossing the swarm intelligence of boids with the soft-body mechanics of \`physics-pbd\` allows us to witness biological swarming intent actively deforming physical environments."

# Insert the new Phase 1 Evaluation at the top of the evaluations
awk -v new_eval="$NEW_PHASE_1" '
    /^## 🔬 Breeding Strategy Update/ {
        print new_eval
        print ""
        print $0
        next
    }
    { print }
' MUTATIONS.md > MUTATIONS.md.tmp && mv MUTATIONS.md.tmp MUTATIONS.md

# Add flock-physics to Attempted Crosses
NEW_CROSS="### flock-physics\n- **Parents**: crates/flocking + crates/physics-pbd\n- **Concept**: Swarm-Driven Soft Body Physics.\n- **Novel trait**: The swarm intelligence of flocking boids actively deforms the physical constraints of a soft body structure governed by Position Based Dynamics (PBD). As agents move and cluster, their kinetic energy acts as an external force on the soft body'\''s particles, stretching and compressing the structural constraints.\n- **Predicted Phenotype**: An organic structural visualizer where continuous swarm behavior results in procedural structural deformation driven by localized swarm density.\n- **Status**: experiments/flock-physics\n- **Evaluation**: Success. Compiled. The soft body mesh is successfully warped by the swarming intent of the boids."

awk -v new_cross="$NEW_CROSS" '
    /^## 🌿 Attempted Crosses/ {
        print $0
        print ""
        print new_cross
        next
    }
    { print }
' MUTATIONS.md > MUTATIONS.md.tmp && mv MUTATIONS.md.tmp MUTATIONS.md
