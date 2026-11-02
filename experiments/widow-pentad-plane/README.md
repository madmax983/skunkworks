# Widow Pentad Plane 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-11-02. Headless ledger below.

## Lineage

- **`neuro-sim`** — the two clocks: a Regular Spiking driver and a
  Fast Spiking follower, same equations, different recovery
  constants.
- **`widow-episodes`** settled the near-3:2 pair (FS I=5): the
  follower runs *early*, the medicine is inhibition, and no
  coupling locks — agreement is priced in episodes.
- **`widow-pentad`** took up the next ratio, **5:3** (FS I=5.3):
  the follower runs *late* by 1.51 ticks per super-cycle, the
  medicine is excitation, and along a single row of ten couplings
  (eight of them at delay 8) the best hand was (+1.0, delay 8),
  longest episode 47 super-cycles, with no lock. Its named open
  seed was this survey: the row was one line through a plane
  nobody had drawn.
- This instrument draws the plane.

## Concept

The pair is unchanged, and so is the counting. Two neurons joined
by exactly one synapse:

- **The driver** — Regular Spiking, constant current I=12. Solo
  measured period: **39.60 ticks** (568 spikes in the window).
- **The follower** — Fast Spiking, constant current I=5.3. Solo
  measured period: **24.06 ticks** (935 spikes). Five follower
  beats span **120.31** against three driver cycles' **118.80** —
  near 5:3, the follower **late by 1.51 ticks per super-cycle**.

A **super-cycle** is three consecutive driver cycles. It is
**perfect** when it holds exactly five follower beats AND each
beat's phase lies within ±0.06 of that beat's median phase for
the regime (the two-gate rule, carried over unchanged). An
**episode** is a maximal run of consecutive perfect super-cycles.
The survey is **9 weights × 7 delays = 63 cells** — weights
−2.5, −1.5, −1.0, −0.5, +0.5, +1.0, +1.5, +2.5, +4.0; delays
0, 2, 4, 6, 8, 10, 12 — each counted over **24,000 ticks**
(first 1,500 discarded; 188 super-cycles), beside the uncoupled
pair on the same window.

Five predictions were declared in the source before the survey
ran. Four held. One cracked, and the crack is the finding.

## Novel Trait

**The pentad's row was a good row drawn through a plane it did
not describe — its champion is not the plane's champion, and
its hour is not the plane's only hour.**

- **No lock anywhere (P1 HELD).** 63 cells, 188 super-cycles
  each, and not one earns TRUE LOCK 5:3 — 43 EPISODIC, 20
  WALKING. The 5:3's refusal to hold is a property of the plane,
  not of the row that happened to be walked.
- **The plane's champion outlasts the row's (P2 HELD).** Best
  cell: **(+1.5, delay 4)** — 134/188 perfect, longest episode
  **55 super-cycles** (~6,534 ticks), mean episode 13.4. The
  pentad's surveyed hand (+1.0, delay 8) re-counted as a cell
  reproduces the parent ledger exactly (119/188 perfect,
  longest 47) — and is beaten on its own plane.
- **The right hour is not a band around 8 (P3 CRACKED).** The
  champion sits at delay **4**, outside the declared 6..=10.
  Worse for the row's tidy story: long episodes live in nearly
  every column — delay 2's best is 40, delay 12's is 39, delay
  0's is 27. Delay decides *which dose* wins (the column bests
  run +0.5, +1.0, +1.5, +1.0, +1.0, +2.5, +1.5), not whether
  agreement is available. When the correction arrives still
  matters; it does not matter the way a single row suggested.
- **The wrong medicine is worse than nothing, everywhere
  (P4 HELD, and strengthened).** Inhibitory half-plane: mean
  longest episode **10.3** across 28 cells, best cell 17
  ((−0.5, delay 8)) — below the *uncoupled* pair's 27.
  Excitatory half-plane: mean **22.0** across 35 cells. Delaying
  a late clock is not merely unhelpful at the doses the row
  tried; no inhibitory cell on the plane reaches doing nothing.
- **The dose ceiling is a coastline, not a point (P5 HELD).**
  The +4.0 row's best cell is 12, against the +1.0 row's 47 —
  and +2.5 sags nearly as hard (best 30, most cells under 20).
  Between +1.5 and +2.5 the excitatory half-plane falls off;
  more hurry stops being medicine before it becomes poison.

## Falsification Condition

The claim — *near 5:3, the follower runs late, so excitation
lengthens episodes across the whole (weight, delay) plane
without ever locking, inhibition is worse than no thread, and
the pentad row's champion and hour do not survive being
surveyed* — is falsified if:

- any cell in the plane reports TRUE LOCK 5:3 (every
  super-cycle perfect) — then the pair could hold after all,
  somewhere the row never looked;
- any inhibitory cell's longest episode matches or beats the
  excitatory best (55), or the inhibitory best reaches the
  uncoupled pair's 27 — then "the follower runs late, so hurry
  it" is the wrong reading of the sign;
- the (+1.0, delay 8) cell fails to reproduce the pentad's
  ledger (119 perfect, longest 47) — then the two instruments
  are not counting the same pair, and no comparison between
  them means anything.

All pinned by unit tests (8/8), including the P3 crack, which is
pinned as measured — (+1.5, delay 4), longest 55 — so the record
cannot quietly heal itself back into the prediction.

## Usage

```bash
cargo run -p widow-pentad-plane -- --headless
cargo test -p widow-pentad-plane
```

*Count everything. Forgive nothing that drifts — including a row
mistaken for a map.*
