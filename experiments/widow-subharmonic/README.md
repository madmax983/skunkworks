# Widow Subharmonic 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-10-29. Headless ledger below.

## Lineage

- **`neuro-sim`** provides two genuinely different clocks in one
  crate: the Regular Spiking neuron (recovery constants a=0.02,
  d=8) and the Fast Spiking neuron (a=0.1, d=2). Same equations,
  different machinery — under constant current each keeps its own
  period, and neither period is written down anywhere in the parent.
  The Widow measured them.
- **`widow-entrainment`** (the Widow's first instrument) studied
  the 1:1 boundary between these clocks and set one case aside in
  its README as the commensurate trap: a Fast Spiking follower at
  I=6, a hair off a perfect 2:1, which excitatory coupling "only
  hurries." This instrument is that case, taken up as the subject.

## Concept

Two neurons, joined by exactly one synapse, whose weight *and*
delay are the only things the Widow is allowed to turn:

- **The driver** — Regular Spiking, constant current I=12. Solo
  measured period: **39.62 ticks** (113 spikes in the window).
- **The follower** — Fast Spiking, constant current I=6. Solo
  measured period: **20.37 ticks** (221 spikes). Two follower
  periods make **40.75** against the driver's 39.62 — ratio
  **1.029**, near 2:1 and not at it. The follower's pair runs long
  by **1.13 ticks per driver cycle**, so uncoupled its phase walks.

A 2:1 lock is counted beat by beat, each beat on its own ring: the
first follower beat in every driver cycle must keep one phase, the
second beat another — the two need not be evenly spaced, because a
lock is a kept pattern, not a metronome's symmetry. The verdict
requires exactly two follower beats in **every** driver cycle AND
both beat dispersions under 0.05.

## Novel Trait

**The boundary is in the timing, not the strength.** The sweep,
counted (6000 ticks per regime, first 1500 discarded):

- **Uncoupled** (0.0): 104/112 cycles with exactly two beats,
  7 short, 1 crowded; beat dispersions 0.3990 / 0.3548 — walking.
- **Fine thread** (2.0): the kick pulls the period 20.37 → 20.03;
  107/112 perfect, dispersions 0.3135 / 0.2707 — still walking.
- **Taut thread** (10.0, no delay): the period is dragged to
  exactly half the driver's, **19.81**, and the phases tighten
  (0.0420 / 0.0184) — but 2 cycles come up short and 2 crowd.
  Phase-tight, count-slipping: not a lock, and the ledger refuses
  to call it one.
- **Timed thread** (10.0, **delay 5 ticks**): the same weight,
  arriving five ticks later, lands in the half of the follower's
  cycle where it takes back exactly the pair's lateness instead of
  shoving the next beat. **112/112 cycles with exactly two beats,
  0 short, 0 crowded, period 19.81, first beat kept at phase 0.183
  (dispersion 0.0140), second at 0.699 (dispersion 0.0287) —
  LOCKED 2:1.** Note the kept phases are unevenly spaced (0.183
  and 0.699, not 0.25 and 0.75): the pattern is the follower's own,
  corrected, not replaced.
- **Iron thread** (35.0): too much hurrying breaks it the other
  way — period crushed to 17.99, only 90/112 perfect, 22 crowded
  cycles. The lock is lost upward, counted as carefully as it was
  lost downward.

During tuning, delays of 15–20 ticks at weight 10.0 and stronger
delayed threads (20–35) were also counted and also walk; the
five-tooth delay at weight 10.0 is the only tooth that holds.
It was never the strength. It was when the kick arrived.

## Falsification Condition

The claim — *a timed synaptic kick entrains the follower at 2:1,
across a measurable boundary* — is falsified if any of these hold:

- the uncoupled follower shows no walk (every cycle perfect, both
  dispersions < 0.05) — then the clocks secretly agreed at 2:1 and
  there was nothing to entrain;
- the timed thread fails to lock (any short or crowded cycle, or
  either dispersion ≥ 0.05) — then timing does no work;
- the iron thread also locks — then there is no boundary above,
  only dose, and the "timing, not strength" reading collapses.

All three are pinned by unit tests (5/5).

## Usage

```bash
# Headless (handled before anything else; prints the metronome ledger)
cargo run -p widow-subharmonic -- --headless

# Tests
cargo test -p widow-subharmonic
```

## The Ledger

Headless runs report, per regime: both solo clocks (spikes and
mean period), the pair period and its lateness per driver cycle,
and for each coupling the spike counts, observed periods, cycles
with exactly two follower beats, short and crowded cycles, both
beats' mean phases and dispersions, and the verdict — LOCKED 2:1
or WALKING. This run: a walk at 0.3990/0.3548, a tug, a
phase-tight slip, a lock at 0.0140/0.0287, and a crowd.

*Count everything. Forgive nothing that drifts.*
