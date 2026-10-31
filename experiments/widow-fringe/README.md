# Widow Fringe 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-10-31. Headless ledger below.

## Lineage

- **`neuro-sim`** — the two clocks: a Regular Spiking driver and a
  Fast Spiking follower, same equations, different recovery
  constants.
- **`widow-subharmonic`** found the timed thread (weight 10.0,
  delay 5) that locks this pair 2:1 at 6,000 ticks.
- **`widow-island`** surveyed the whole (weight, delay) plane at
  that window and reported 34 locking cells of 156: a 27-cell
  plateau (weights 5.0/7.5/10.0 × delays 2–12), a fringe at weight
  3.0 (delays 3, 5, 6, 8, 10 — with a hole at delay 4), one landing
  at (10.0, 1), and a stray tooth at (12.5, 10). Its lock
  definition: exactly two follower beats in every driver cycle
  AND both aggregate beat dispersions under 0.05.
- **`widow-episodes`** built the ledger this instrument borrows:
  when a pair will not hold, price how long it almost holds —
  episodes of consecutive perfect cycles under a two-gate rule
  (count AND per-beat phase within ±0.06 of the median phase).

## Concept

The island's verdicts are photographs taken at 112 cycles. This
instrument re-counts thirteen of its cells — every fringe landing,
the hole, the stray tooth and the water beside it, the plateau's
heart and corner, the uncoupled pair, the timed iron thread — at
**24,000 ticks** (~567 driver cycles, first 1,500 discarded),
under the episodes instrument's two-gate rule. The pair is
unchanged: driver RS I=12, solo period **39.60**; follower FS
I=6, solo period **20.38**; the follower's pair spans 40.75
against the driver's 39.60 — ratio 1.029, running long by 1.15
ticks per cycle, corrected by one delayed excitatory synapse.

## Novel Trait

**At the long window, nothing the island called a lock holds
every cycle — and the shore it drew dissolves.**

- **No TRUE LOCK survives.** Of the nine island-LOCKED cells
  re-counted, **zero** hold all 567 cycles under the two-gate
  rule. The plateau heart (10.0, 5) comes closest: 536/567
  perfect, **0 count slips**, dispersions 0.0115/0.0232 — by the
  island's own aggregate definition it would still pass; its 31
  failures are single beats straying past ±0.06 of their median
  inside an otherwise tight pattern. The lock is real as a
  pattern and episodic as a sequence: longest episode **72
  cycles** (~2,851 ticks), mean 20.6, 26 episodes.
- **The hole is a short window's artefact.** The fringe landings
  at weight 3.0 score 395–420 perfect of 567, longest episodes
  27–50, mean ~7–9. The hole at (3.0, 4) — walking at 112 cycles —
  scores **401 perfect, longest 29, mean 8.2**: inside the
  landings' own spread. At this window there is no hole; there
  is a fringe that breathes everywhere at roughly the same rate.
- **The longest visit is in open water.** Beside the stray tooth,
  (12.5, 8) — walking on the island — holds the sweep's longest
  episode: **115 cycles** (~4,554 ticks), 531/567 perfect, mean
  episode 23.1, the best of any cell. The stray tooth itself
  (12.5, 10): 519 perfect, longest 57. Coastlines drawn at 112
  cycles put the boundary in the wrong place in both directions.
- **The iron thread stays water, differently.** (35.0, 5): 456
  perfect but 103 count slips (it crowds), longest episode 20,
  verdict WALKING — strength still cannot buy what timing sells.
- The uncoupled pair: 113 perfect, longest 8 — the floor every
  coupled cell stands well above.

## Falsification Condition

The claim — *the island's fringe verdicts are window-length
artefacts; at 24,000 ticks the fringe and its hole are one
breathing band, and even the plateau is episodic under a
per-cycle phase gate* — is falsified if:

- any re-counted cell earns TRUE LOCK under the two-gate rule
  (the test suite forbids the verdict with any slip present);
- the hole (3.0, 4) scores dramatically below the landings
  (perfect count or longest episode under half the landings'
  worst) — then the island's coastline was real after all;
- the plateau heart falls below 90% perfect — then even the
  ground was weather, and the island's whole map, not just its
  fringe, was a photograph of a gust.

All pinned by unit tests (6/6).

## Usage

```bash
cargo run -p widow-fringe -- --headless
cargo test -p widow-fringe
```

*Count everything. Forgive nothing that drifts — including a map.*
