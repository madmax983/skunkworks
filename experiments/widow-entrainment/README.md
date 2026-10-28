# Widow Entrainment 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-10-28. Headless ledger below.

## Lineage

- **`neuro-sim`** provides two genuinely different clocks in one
  crate: the Regular Spiking neuron (recovery constants a=0.02,
  d=8) and the Fast Spiking neuron (a=0.1, d=2). Same equations,
  different machinery — under constant current each keeps its own
  period, and neither period is written down anywhere in the parent.
  The Widow measured them.

## Concept

Two neurons, joined by exactly one synapse, its weight the only
thing the Widow is allowed to turn:

- **The driver** — Regular Spiking, constant current I=12. Solo
  measured period: **39.62 ticks** (113 spikes in the window).
- **The follower** — Fast Spiking, constant current I=4. Solo
  measured period: **44.52 ticks** (101 spikes). Ratio **1.124**:
  near the driver, and not equal. That gap is the drift.

The synapse is swept through three named teeth — uncoupled (0.0),
weak thread (1.0), strong thread (35.0) — and each regime is
counted in beats: driver cycles containing exactly one follower
spike (matched), cycles containing none (missed beats), extra
spikes, the follower's observed period, and the circular dispersion
of its phase inside the driver cycle (0 = one fixed phase, a lock;
→1 = phase smeared around the whole cycle, a wander).

## Novel Trait

**Entrainment with its boundary left in.** Most coupled systems in
this garden are shown agreeing. This one is shown *almost*
agreeing, and the almost is measured:

- Uncoupled, the follower slips a full beat every ~9 cycles:
  100/112 cycles matched, 12 missed, dispersion 0.9664.
- On the weak thread (weight 1.0) the driver's kick *pulls* —
  the follower's period is dragged from 44.52 toward the driver
  (41.43) and missed beats fall to 5 — but the phase still wanders
  (dispersion 0.3840). Partial entrainment: a tug, not a hold.
- On the strong thread (weight 35.0) the follower surrenders its
  period entirely: 112/112 cycles matched, 0 missed, 0 extra,
  period exactly 39.62, locked at mean phase 0.050 with dispersion
  0.0000.

During tuning the Widow also found the commensurate trap: a Fast
Spiking follower at I=6 keeps period 20.37 — a hair off a perfect
2:1 against the driver — and excitatory coupling only hurries it.
She set that clock aside; this instrument studies the 1:1 boundary.

## Falsification Condition

The claim — *synaptic coupling entrains the follower, across a
measurable boundary* — is falsified if any of these hold:

- the uncoupled follower shows no drift (no missed or extra beats,
  low phase dispersion) — then the clocks secretly agreed and there
  was nothing to entrain;
- the strong thread fails to lock (any missed or extra beat, or
  dispersion ≥ 0.05) — then the coupling does no work;
- tightening the coupling does not tighten the phase (dispersion
  fails to fall from weak to strong) — then there is no boundary,
  only a switch or a coincidence.

All three are pinned by unit tests.

## Usage

```bash
# Headless (handled before anything else; prints the metronome ledger)
cargo run -p widow-entrainment -- --headless

# Tests
cargo test -p widow-entrainment
```

## The Ledger

Headless runs report, per regime (6000 ticks, first 1500 discarded
as settling): both solo clocks (spikes and mean period), the
intrinsic ratio, and for each coupling the spike counts, observed
periods, matched/missed/extra beats, mean phase, phase dispersion,
and the verdict — LOCKED or DRIFTING. This run: drift 0.9664,
a tug at 0.3840, a lock at 0.0000.

*Count everything. Forgive nothing that drifts.*
