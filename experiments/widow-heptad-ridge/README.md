# Widow Heptad Ridge 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-11-09. Headless ledger below.

## Lineage

- **`neuro-sim`** — the two clocks: a Regular Spiking driver and a
  Fast Spiking follower, same equations, different recovery
  constants.
- **`widow-heptad`** took up the near-**7:5** pair (FS I=4.8): the
  follower runs *early*, the medicine is inhibition, and along one
  row the firm hand (−1.5, delay 8) led at longest 45.
- **`widow-heptad-plane`** drew the whole (weight, delay) plane —
  63 cells — and crowned a champion: **(−1.0, delay 10)**,
  92/112 perfect, longest episode **58**.
- **`widow-heptad-peak`** measured the fine grid around the
  champion and found a **tooth, not a plateau** — and a second
  tower on its diagonal, (−0.75, delay 9) at 51.
- **`widow-heptad-diagonal`** recentred on that diagonal and
  showed it was the crown's **shoulder**: the tall cells traced a
  compensation diagonal — (−0.50, 8) 21, (−0.75, 9) 51,
  (−1.00, 10) 58 — heavier doses arriving later balancing
  lighter doses arriving sooner. Its README named this
  instrument outright: *walk the compensation line itself out of
  this grid in both directions — does the line fall off a cliff
  past the crown (−1.25, delay 11 / −1.50, delay 12), and does
  ordinary ground resume west of (−0.50, 8) at (−0.25, delay 7)?*
- **`widow-pentad-ridge`** walked the 5:3 pair's line the same
  way and found a **finite segment**: a ditch to the west, a
  cliff past its crown (55 → 16, recovering only to 34), and
  one-tick-early controls that matched or beat their on-line
  partners past the crown. This instrument walks the 7:5 line
  under the opposite medicine, so the two ratios' extents can
  be compared step for step.

## Concept

The pair is unchanged, and so is the counting. Two neurons joined
by exactly one synapse:

- **The driver** — Regular Spiking, constant current I=12. Solo
  measured period: **39.60 ticks** (568 spikes in the window).
- **The follower** — Fast Spiking, constant current I=4.8. Solo
  measured period: **28.01 ticks** (803 spikes). Seven follower
  beats span **196.08** against five driver cycles' **198.00** —
  near 7:5, the follower **early by 1.92 ticks per super-cycle**.

A **super-cycle** is five consecutive driver cycles. It is
**perfect** when it holds exactly seven follower beats AND each
beat's phase lies within ±0.06 of that beat's median phase for
the regime (the two-gate rule, carried over unchanged). An
**episode** is a maximal run of consecutive perfect super-cycles.
Each cell is counted over **24,000 ticks** (first 1,500
discarded; 112 super-cycles), beside the uncoupled pair on the
same window (83/112 perfect, longest 17).

The survey is the six on-line cells of the compensation line —
delay = 8 + (−0.50 − weight)/0.25 — walked west to east:
(−0.25, 7), (−0.50, 8), (−0.75, 9), (−1.00, 10), (−1.25, 11),
(−1.50, 12) — beside three **one-tick-early controls** at the
three weights the diagonal never surveyed: (−0.25, 6),
(−1.25, 10), (−1.50, 11). Lines declared before the survey ran,
from the crown (−1.00, 10) at longest 58: plateau line **47**
(80% of 58, rounded up); isolation line **29** (50% of 58).
Five predictions were declared in the source before the
extension cells ran. Three held; two cracked, and the cracks
are the finding.

## Novel Trait

**The 7:5 ridge is a finite segment too — two cells long, with
the crown at its eastern end, and past the crown the
compensation expires at this ratio as well.**

Longest episode, on-line west to east:

| weight | delay | perfect | longest |
|--------|-------|---------|---------|
| −0.25 | 7 | 82/112 | 17 |
| −0.50 | 8 | 75/112 | 21 |
| −0.75 | 9 | 86/112 | 51 |
| −1.00 | 10 | 92/112 | **58 (crown)** |
| −1.25 | 11 | 69/112 | **34** |
| −1.50 | 12 | 88/112 | 37 |

Controls: (−0.25, 6) longest **33**; (−1.25, 10) longest **39**;
(−1.50, 11) longest **15**.

- **No lock, again (P1 HELD).** 9 cells, 112 super-cycles each,
  zero TRUE LOCK 7:5.
- **The anchors reproduce (P2 HELD).** The three on-line cells
  the diagonal already counted keep their numbers exactly:
  21 / 51 / 58, perfect 86 and 92 at the centre and crown. The
  instruments agree; what follows is not a counting difference.
- **The ridge does not continue (P3 CRACKED).** Neither eastern
  extension reaches the plateau line 47: one step past the
  crown the line falls off its cliff (58 → **34**), and the
  second step recovers only to **37**. The 5:3 walk fell
  55 → 16 → 34 along the same shape; the 7:5 walk falls
  58 → 34 → 37. Two ratios, opposite medicines, one silhouette:
  a crown, a cliff, a partial recovery that does not restore.
- **Ordinary ground resumes in the west (P4 HELD).** (−0.25, 7)
  stands at **17** — at or below the isolation line 29, and
  exactly the uncoupled pair's own longest episode. West of
  (−0.50, 8) the line is not low ground on the ridge; it is no
  ridge at all, the bare pair keeping its own time. The tall
  segment is two cells long: (−0.75, 9) 51 to the crown
  (−1.00, 10) 58, approached from 21.
- **Arrival stops being the currency (P5 CRACKED).** Two of the
  three one-tick-early controls *beat* their on-line partners:
  (−0.25, 6) at **33 doubles** (−0.25, 7) at 17, and (−1.25, 10)
  at **39 beats** (−1.25, 11) at 34. Only at the far eastern
  weight does the declared order hold — (−1.50, 11) at 15 below
  (−1.50, 12) at 37. Inside the segment, dose traded against
  arrival; at every extension the trade no longer clears, and
  in the west arriving early is worth a whole ridge the line
  itself does not have. As at 5:3 — the medicine differs, the
  expiry does not.

The segment, whole: it runs from (−0.75, 9) to the crown
(−1.00, 10), two cells long, bounded by ordinary ground to the
west and a cliff to the east. A ridge in the diagonal's small
grid was a segment in this one's longer walk — the second
ratio to answer so, under the opposite medicine. Both records
are pinned; the segment is the shape that explains both.

## Falsification Condition

The claim — *the 7:5 compensation line's tall segment is
finite, running (−0.75, 9) → (−1.00, 10), with ordinary ground
at (−0.25, 7), a cliff past the crown, and early controls that
match or beat their on-line partners at two of the three
extension weights* — is falsified if:

- any cell reports TRUE LOCK 7:5 — then the pair could hold
  after all, somewhere along the line;
- the anchors fail to reproduce 21 / 51 / 58 — then the
  instruments are not counting the same pair;
- either eastern extension reaches the plateau line 47 on a
  recount — then the ridge continues and the "segment" is this
  survey's artefact;
- both eastern controls fall below their on-line partners on a
  recount — then arrival was still the currency past the crown
  and P5's crack was noise.

All pinned by unit tests (6/6), including the full measured
walk — 17, 21, 51, 58, 34, 37 on-line; controls 33, 39, 15 —
so the record cannot quietly heal itself into a road.

## Next Seed

Walk one step *off* the line at the crown's own weight: the
plane counted (−1.00, delay 8) and (−1.00, delay 9) inside its
63 cells, but no instrument has yet priced the crown's column
at the fine grain the line cells received, east of the crown —
(−1.25, 12) and (−1.50, 13), one tick late at the extension
weights. If the cliff is a property of the line, the late
controls should fall too; if it is a property of the hour,
one of them may stand where the line fell.

## Usage

```bash
cargo run -p widow-heptad-ridge -- --headless
cargo test -p widow-heptad-ridge
```

*Count everything. Forgive nothing that drifts — including a
ridge whose length was borrowed from the grid that framed it,
at a second ratio.*
