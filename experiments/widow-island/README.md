# Widow Island 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Subject:** the 2:1 lock found by `experiments/widow-subharmonic`
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-10-29. Headless ledger below.

## Lineage

- **`neuro-sim`** provides the two clocks, unchanged from the
  parent instrument: a Regular Spiking driver at I=12 (solo
  period **39.62 ticks**) and a Fast Spiking follower at I=6
  (solo period **20.37 ticks**; its pair makes 40.75 against the
  driver's 39.62 — ratio **1.029**, near 2:1 and not at it, the
  pair running long by 1.13 ticks per driver cycle).
- **`widow-subharmonic`** (the Widow's second instrument) found
  that one timed thread — weight 10.0, delay 5 ticks — holds
  this pair at 2:1 (112/112 cycles, dispersions 0.0140 / 0.0287),
  while the undelayed and iron threads walk. Its README called
  that thread "the only tooth that holds," on the strength of a
  sparse sweep. This instrument is that sentence, audited.

## Concept

Sweep the whole plane the parent instrument sampled: **12
weights (0.0–35.0) × 13 delays (0–20 ticks) = 156 regimes**,
each run for the parent's full window (6000 ticks, first 1500
discarded) and judged by the parent's exact definition of a
lock — exactly two follower beats in **every** driver cycle
**and** both beat dispersions under 0.05. No new clocks, no new
verdict: a survey, so the two ledgers can be read side by side.

## Novel Trait

**The tooth was not a point. It was an island.**

- **The plateau:** weights **5.0, 7.5, and 10.0** lock at
  **every** swept delay from 2 to 12 ticks — **27 of 27 cells**,
  a rectangular plateau in the middle of the plane. The
  parent's tooth (10.0, delay 5) re-counts locked inside it
  (112/112, dispersions 0.0140 / 0.0287), one cell among many.
- **The fringe:** 7 locking cells outside the plateau. Weight
  10.0 also locks at delay 1. Weight 3.0 locks at five scattered
  delays (3, 5, 6, 8, 10) with holes between them — it locks at
  delay 5 (dispersions 0.0291 / 0.0399) and slips at delay 4,
  one tick away (111/112, dispersions 0.0649 / 0.0691). And one
  **stray tooth** stands alone at weight 12.5, delay 10
  (112/112, dispersions 0.0081 / 0.0216), with walking water on
  every side.
- **The shores:** no weight locks at **delay 0** — timing is
  necessary, at any strength. Nothing locks at delay 15 or 20.
  Nothing at **weight 15.0 or above** locks anywhere — not even
  timed (the iron thread at 35.0, delay 5: 90/112, 22 crowded).
- **Total: 34 of 156 cells lock.**

The first survey swept sparsely and reported what it swept,
then overstated it in one sentence. A boundary drawn from a
handful of points is a coastline guessed from a handful of
landings. The correction is not a different opinion about the
lock; it is the lock's actual shape.

## Falsification Condition

The claim — *the 2:1 lock occupies an island: a 27-cell
plateau, a holed fringe, one stray tooth, and clean shores* —
is falsified if any of these hold:

- any plateau cell (weights 5.0/7.5/10.0 × delays 2–12) walks —
  then there is no plateau, only scattered teeth;
- any cell at delay 0, at delay 15 or 20, or at weight ≥ 15.0
  locks — then a shore is not where the map draws it;
- the fringe at weight 3.0 is solid (delay 4 locks too) or the
  stray tooth has a locking neighbour — then the island's edge
  is a slope, not a coast with holes and an outlier.

All are pinned by unit tests (8/8).

## Usage

```bash
# Headless (handled before anything else; prints the map and ledger)
cargo run -p widow-island -- --headless

# Tests
cargo test -p widow-island
```

## The Ledger

Headless runs report both solo clocks, the pair period and its
lateness, the full 13×12 map (L = locked, . = walking) with
per-row locking counts, the locking total (34/156), the
plateau count (27/27), the fringe count (7), the three shore
counts (0, 0, 0), and six anchor cells counted in full —
the parent's tooth, its undelayed twin, a fringe landing, the
hole beside it, the stray tooth, and the timed iron thread.
This run: an island where a tooth was reported.

*Count everything. Forgive nothing that drifts.*
