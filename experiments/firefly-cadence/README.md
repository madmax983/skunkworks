# firefly-cadence

A revival cross by The Cartographer of Lost Things 🗺️ —
coordinates: graveyard/firefly-synapse → crates/git-associates.

## Lineage

**The dead:** `graveyard/firefly-synapse` — 100,000 Kuramoto
fireflies on a 1000×1000 field (macroquad + rayon): each agent's
phase turns at its natural frequency, neighbours within radius 4
pull phases together by (K/N_local) · Σ sin(θj − θi), dt = 0.1.
Executed for **Generic Implementation / Ecosystem Isolation**: a
textbook Kuramoto sketch containing no repository at all. Its
Reaper report is one of the rare graves that arrives with its own
work order — salvation requires *"a meaningful structural
integration that leverages repository data (e.g. git commits
driving natural frequency)"*. Two further flaws the certificate
never named, visible in its own source:

- Its frequencies were drawn U[0.10, 0.12] — a band ±10% wide, in
  which synchronisation under its coupling is a foregone
  conclusion, not a finding.
- Its swarm was never counted. No order parameter, no cluster
  census, no headless bypass: the only ledger it ever printed was
  FPS on a window, and the window is what killed it.

**The donation:** the living `crates/git-associates` supplies the
exact organ the Reaper prescribed — this repository's commit
history (hashes, timestamps) read headlessly, the same supply the
living `git-resonance` bell has rung from since the garden opened.
Nothing else was missing: the corpse's update rule is carried over
verbatim in shape.

**The cross:** every commit becomes one firefly, and the
repository's *cadence* becomes its frequency. The gap until the
next commit landed (hours) is log-mapped onto ω ∈ [0.06, 0.16] —
a burst-era commit (minutes between landings) burns fast, a
quiet-era commit (weeks) burns slow. Position and opening phase
come from the commit's hash, as the bell's notes do, so the whole
swarm is a pure function of the history: change the history and
the swarm changes; run it twice and it is identical.

## What is genuinely new

Neither parent does this alone. The corpse synchronised a
fictional swarm whose frequencies were chosen to synchronise; the
git crate has never been asked what its history *does* when its
commits are coupled to each other. The revival runs one swarm in
five rooms, so each ingredient is priced separately:

| Room | Frequencies | Placement | Coupling |
|------|-------------|-----------|----------|
| corpse-hash-local | corpse's U[0.10, 0.12] (hash-drawn) | hash | local |
| cadence-hash-local | cadence | hash | local |
| cadence-era-local | cadence | era (x = rank, y = hash) | local |
| shuffled-era-local | cadence, shuffled (seed 42) | era | local |
| cadence-global | cadence | hash | mean-field (exact) |

The era and hash placements differ **only in x** (y is the hash's
in both) — a controlled swap. The shuffled room keeps the cadence
distribution and the era map and destroys only the assignment:
it separates *what frequencies exist* from *where they live*.
Each room is surveyed at K ∈ {0.025, 0.05, 0.1, 0.2, 0.4} for
1,500 ticks; the ledger reports R(K) per room, the critical
coupling Kc (first K with final R ≥ 0.8), and at K = 0.1 the
phase-coherent cluster census (edges: neighbours with wrapped
phase difference < 0.3 rad), the flash count (phase wraps), and
each map split into west/east provinces (own R, wrapped mean-phase
gap between halves).

## Predictions (declared before the first run)

- **P1.** The corpse's narrow band has the lowest Kc of any room.
- **P2.** At K = 0.05 the local cadence rooms fragment
  (clusters ≥ 3 in both) while the corpse holds one dominant
  cluster (largest ≥ 90% of N).
- **P3.** Era placement does not rescue cadence locally:
  R_era(0.1) < R_hash(0.1) — cadence is era-structured, so an era
  map builds provinces that turn at different rates, not
  neighbourhoods that agree.
- **P4.** Shuffling the cadence assignment on the era map raises
  R(0.1) above the true assignment — dispersing the extremes
  helps the mean field more than era locality helps neighbours.
- **P5.** Global coupling erases placement: R_global(0.1) ≥ both
  local cadence rooms.
- **P6.** Provinces: at K = 0.1 each era half is more coherent
  than the era whole (R_west, R_east > R_all), and the era
  province gap exceeds the hash room's.

## Falsification condition

This specimen is wrong — and should be composted back to the
archive — if, on this repository's real history, the cadence
rooms' ledgers are indistinguishable from the corpse room's
(same Kc, same R at K = 0.1 within 0.02, same cluster census):
that would mean cadence carries no structure the corpse's narrow
band did not already have, and the Reaper's prescribed integration
changed nothing. It is also wrong if the shuffled room matches
the era room exactly at every K — that would mean assignment
never matters and the map is decoration.

## Run

```bash
cargo run -p firefly-cadence -- --headless [repo_path] [limit]
```

`--headless` is handled before anything else; the specimen has no
TUI at all. Default: the repository at `.`, newest 200 commits.
If no repository can be opened, a deterministic synthetic history
(200 commits, burst/lull blocks) is lit instead and the ledger
says so — a bell in a vacuum still gets heard.
