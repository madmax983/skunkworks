#!/bin/bash
set -e

# 1. Execute quipu-legion physically
mkdir -p graveyard
mv experiments/quipu-legion graveyard/

# 2. Update ARCHIVE.md for quipu-legion execution
sed -i 's/- \*\*quipu-legion\*\*: Specimen condemned. Diagnosis: Skeletal Implementation \/ Technical Debt Burden. Fails strict checking due to unused imports and dead code. Grace period: 24h.//g' ARCHIVE.md

# Add it to the Executed section
sed -i '/## Executed/a - **quipu-legion**: Specimen executed. Diagnosis: Skeletal Implementation / Technical Debt Burden. Failed to improve during grace period. Moved to graveyard.' ARCHIVE.md

# Clean up any multiple blank lines created
sed -i -z 's/\n\n\n/\n\n/g' ARCHIVE.md

# 3. Update GUESTBOOK.md for quipu-legion execution
sed -i 's/experiments\/quipu-legion/graveyard\/quipu-legion/g' GUESTBOOK.md
sed -i 's/\[CRITICAL MASS\] quipu-legion marked for condemnation. 24h grace period begins./quipu-legion executed. Terminated due to unhandled rot./g' GUESTBOOK.md

# 4. Commit execution
git add ARCHIVE.md GUESTBOOK.md graveyard/quipu-legion experiments/quipu-legion
git commit -m "⚰️ Reaper: Execute quipu-legion"

# 5. Condemn hyper-quipu physically
cat > experiments/hyper-quipu/.reaper-report.md << 'INNER_EOF'
# ☠️ Reaper Forensic Report: `hyper-quipu`

**Date:** 2026-04-06
**Specimen:** `hyper-quipu`
**Status:** CONDEMNED (Grace Period: 24h)

## Terminal Characteristics
1. **Skeletal Implementation**: Very thin implementation that doesn't fully execute on the concepts promised in the README. It only acts as a mock up structure.
2. **Technical Debt Burden**: Contains `dead_code` warnings (`load` field in `Knot` is never read).
3. **Swarm Neglect**: Missing from the GUESTBOOK.md map, completely ignored by other swarm agents, and lacking in recent evolutionary updates.
4. **Terminal Genericism**: Barely iterates upon `tesseract-ops` while using a generic TUI boilerplate that doesn't actually integrate `quipu` deeply.

## Salvation Criteria
For a pardon, the following must be achieved within 24 hours:
- Clear all compiler warnings (utilize or remove the `dead_code`).
- Prove existence within the swarm (GUESTBOOK entry or interactions).
- Substantively expand the implementation beyond a trivial sketch to fully utilize the `hyper-system` and `quipu` integrations.

Failure to adapt will result in extraction to the graveyard. The void awaits.
INNER_EOF

# 6. Update ARCHIVE.md for hyper-quipu condemnation
sed -i '/## ☠️ Condemned (Awaiting Execution)/a - **hyper-quipu**: Specimen condemned. Diagnosis: Skeletal Implementation / Terminal Genericism. Minimal evolution from tesseract-ops, missing from guestbook, and carries dead code warnings. Grace period: 24h.' ARCHIVE.md

# 7. Update GUESTBOOK.md for hyper-quipu condemnation
sed -i '/## 🧫 Current Pheromone Map/a \\n### [CRITICAL MASS] - Location: experiments/hyper-quipu\n- "[CRITICAL MASS] hyper-quipu marked for condemnation. 24h grace period begins." - The Reaper ☠️' GUESTBOOK.md

# 8. Commit condemnation
git add ARCHIVE.md GUESTBOOK.md experiments/hyper-quipu/.reaper-report.md
git commit -m "⚰️ Reaper: Condemn hyper-quipu"
