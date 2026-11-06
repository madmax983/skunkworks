# Widow Heptad Plane 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-11-06. Headless ledger below.

## Lineage

- **`neuro-sim`** — the two clocks: a Regular Spiking driver and a
  Fast Spiking follower, same equations, different recovery
  constants.
- **`widow-subharmonic` / `widow-island` / `widow-fringe`** settled
  the near-**2:1** pair: it locks, on an island in the plane.
- **`widow-episodes`** took up the near-**3:2** pair: the follower
  runs *early*, the medicine is inhibition, no coupling locks —
  agreement is priced in episodes, longest 46.
- **`widow-pentad`** and its plane / peak / diagonal / ridge
  surveys settled the near-**5:3** pair: the follower runs *late*,
  the medicine is excitation, no coupling locks, and the plane's
  champion (+1.5, delay 4; longest 55) outlasted the row's
  champion and stood outside the row's favoured hour.
- **`widow-heptad`** took up the near-**7:5** pair along one row:
  ten couplings, seven at delay 8. The follower runs *early* by
  1.92 ticks per super-cycle; the firm hand (-1.5, delay 8) led
  both columns (longest 45, perfect 89/112); the wrong medicine
  (+1.5, delay 8) tied the uncoupled perfect count (83) while
  collapsing its longest episode (17 → 6); and at the surveyed
  weight (-1.0) delay 8 was already beaten by delays 0 and 4.
  Its named open seed was this survey: the row was one line
  through a plane nobody had drawn.
- This instrument draws the plane.

## Concept

The pair is unchanged, and so is the counting. Two neurons joined
by exactly one synapse:

- **The driver** — Regular Spiking, I=12. Solo period **39.60**.
- **The follower** — Fast Spiking, I=4.8. Solo period **28.01**.
  Seven follower beats span **196.08** against five driver
  cycles' **198.00** — near 7:5, the follower **early by 1.92
  ticks per super-cycle**.

A **super-cycle** is five driver cycles; **perfect** = exactly
seven follower beats, each within ±0.06 of its median phase
(two-gate rule, unchanged from `widow-heptad`). An **episode**
is a maximal run of perfect super-cycles. The survey is
**9 weights × 7 delays = 63 cells** — weights −4.0, −2.5, −1.5,
−1.0, −0.5, +0.5, +1.0, +1.5, +2.5; delays 0, 2, 4, 6, 8, 10, 12 —
each counted over **24,000 ticks** (first 1,500 discarded; 112
super-cycles), beside the uncoupled pair on the same window.
Six predictions were declared in the source before the survey
ran. All six held.

## Novel Trait

**The 7:5's row was a good row drawn through a plane it did not
describe — its champion is not the plane's champion, its hour is
the surveyed hand's near-worst hour, and the wrong medicine is
worse than nothing at every address, on the duration column.**

Longest episode per cell (columns are delays 0 / 2 / 4 / 6 / 8 / 10 / 12):

| weight | 0 | 2 | 4 | 6 | 8 | 10 | 12 |
|--------|---|---|---|---|---|---|----|
| −4.0 | 7 | 9 | 8 | 8 | 7 | 9 | 10 |
| −2.5 | 16 | 13 | 20 | 17 | 15 | 14 | 18 |
| −1.5 | 37 | 22 | 31 | 39 | **45** | 32 | 37 |
| −1.0 | 29 | 33 | 32 | 43 | 22 | **58** | 13 |
| −0.5 | 21 | 29 | 24 | 30 | 21 | 32 | 18 |
| +0.5 | 10 | 17 | 17 | 13 | 12 | 11 | 9 |
| +1.0 | 11 | 10 | 12 | 8 | 11 | 11 | 9 |
| +1.5 | 7 | 7 | 10 | 7 | 6 | 10 | 7 |
| +2.5 | 4 | 4 | 5 | 5 | 6 | 4 | 5 |

- **No lock anywhere (P1 HELD).** 63 cells, 112 super-cycles
  each, and not one earns TRUE LOCK 7:5 — 42 EPISODIC, 21
  WALKING. The 7:5's refusal to hold is a property of the plane,
  not of the row that happened to be walked.
- **The plane's champion outlasts the row's (P2, P3 HELD).**
  Best cell: **(−1.0, delay 10)** — 92/112 perfect, longest
  episode **58 super-cycles** (~11,484 ticks), mean episode
  18.4. The heptad's firm hand (−1.5, delay 8) re-counted as a
  cell reproduces the row ledger exactly (89/112, longest 45 —
  P6 HELD) and stands as the delay-8 column's best, and no more.
  Every column's best is inhibitory, and only two doses ever
  win a column: −1.5 and −1.0.
- **Delay 8 is the surveyed hand's near-worst hour.** Along the
  −1.0 row the hours read 29 / 33 / 32 / 43 / **22** / **58** /
  13: the hour the row was drawn at is beaten by five of the
  six other hours, and the champion's hour (10) is adjacent to
  the row's worst (12, at 13). At −1.5 the same delay 8 is the
  row's own peak (45). The right hour is a property of the
  dose, not of the ratio — borrowing an hour from a sibling
  dose misprices both.
- **The wrong medicine never beats doing nothing, on duration
  (P4 HELD, and strengthened).** Excitatory half-plane: mean
  longest episode **8.9** across 28 cells, best cell 17
  (+0.5, at delays 2 and 4) — exactly the *uncoupled* pair's 17,
  never above it. Inhibitory half-plane: mean **23.4** across
  35 cells. Excitation still buys counts (its perfect counts
  run 67–88, beside the inhibitory 64–93); it buys no duration,
  anywhere on the plane.
- **The dose ceiling is a coastline, mirrored (P5 HELD).** The
  −4.0 row's best cell is 10, against the −1.0 row's 58 — and
  −2.5 sags nearly as hard (best 20). Between −1.5 and −2.5
  the inhibitory half-plane falls off; more delay stops being
  medicine before it becomes the crushing hand's 7–10.

The survey's four ratios now stand in a row, three of them with
their planes drawn: 2:1 locks, on an island; 3:2, 5:3, and 7:5
do not, and are priced in episodes. At both drawn planes the
row's champion fell to a cell the row never visited, and at
both the wrong medicine was worse than nothing across the
whole half-plane — the row's local verdicts were the plane's
global ones, but its champion and its hour were its own.

## Falsification Condition

The claim — *the near-7:5 plane holds no lock; its champion is
(−1.0, delay 10) at longest 58; inhibition is the medicine
across the whole plane (mean longest 23.4 against the
excitatory 8.9, whose best cell only ties the uncoupled 17);
and delay selects by dose, not by ratio* — is falsified if:

- any cell reports TRUE LOCK 7:5 — then the pair could hold
  after all, somewhere in this survey;
- the solo clocks fail to reproduce 39.60 / 28.01, or their
  super-cycle earliness falls outside 0.5–3.5 ticks — then this
  is not the pair the ledger describes;
- the (−1.5, delay 8) cell fails to reproduce the heptad's
  ledger (perfect 89, longest 45) — then the two instruments
  are not counting the same pair, and no comparison between
  them means anything;
- any excitatory cell's longest episode exceeds the uncoupled
  pair's 17 on a recount — then hurrying an early clock does
  buy duration somewhere, and the half-plane verdict was this
  survey's artefact;
- (−1.0, delay 8) outlasts (−1.0, delay 10) on a recount —
  then the champion's hour was noise and the row's hour stands.

All pinned by unit tests (8/8), including the champion cell,
which is pinned as measured — (−1.0, delay 10), longest 58,
perfect 92 — so the record cannot quietly heal itself back
into the row's story.

## Headless Ledger

`cargo run -p widow-heptad-plane -- --headless` prints the
metronome ledger: both solo clocks, the super-cycle earliness,
the uncoupled pair, both grids (longest episode and perfect
count per cell), the verdict census, the best cells by half-
plane, row and column bests, the heptad row's cells re-counted,
and the six predictions with their verdicts. The numbers in
the table above are that ledger, verbatim in kind.

*Count everything. Forgive nothing that drifts — including a
row mistaken for a map, twice.*
