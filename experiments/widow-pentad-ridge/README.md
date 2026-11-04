# Widow Pentad Ridge 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-11-04. Headless ledger below.

## Lineage

- **`neuro-sim`** — the two clocks: a Regular Spiking driver and a
  Fast Spiking follower, same equations, different recovery
  constants.
- **`widow-pentad`** took up the near-**5:3** pair (FS I=5.3): the
  follower runs *late* by 1.51 ticks per super-cycle, the medicine
  is excitation, and along a single row no lock was found.
- **`widow-pentad-plane`** drew the whole (weight, delay) plane
  and crowned a champion: **(+1.5, delay 4)**, longest episode 55.
- **`widow-pentad-peak`** found a tooth, not a plateau, around the
  champion — and a second tower on its north-west diagonal.
- **`widow-pentad-diagonal`** recentred on that diagonal and
  showed the tall cells trace a **ridge**: (+1.00, 2) at 40,
  (+1.25, 2) at 42, (+1.25, 3) at 46, (+1.50, 4) at 55 — lighter
  doses arriving sooner balancing heavier doses arriving later.
- This instrument walks the ridge's **compensation line** —
  delay = 2 + (weight − 1.00)/0.25, the line through the three
  on-line tall cells (+1.00, 2), (+1.25, 3), (+1.50, 4) — beyond
  the diagonal's grid in both directions, and asks whether the
  ridge is a road or a finite segment with the crown at its end.

## Concept

The pair and the counting are unchanged. Two neurons joined by
exactly one synapse:

- **The driver** — Regular Spiking, I=12. Solo period **39.60**.
- **The follower** — Fast Spiking, I=5.3. Solo period **24.06**.
  Five follower beats span **120.31** against three driver
  cycles' **118.80** — near 5:3, the follower **late by 1.51
  ticks per super-cycle**.

A **super-cycle** is three driver cycles; **perfect** = exactly
five follower beats, each within ±0.06 of its median phase
(two-gate rule, unchanged). An **episode** is a maximal run of
perfect super-cycles. Each cell counted over **24,000 ticks**
(first 1,500 discarded; 188 super-cycles).

The survey is the seven on-line cells — (+0.50, 0), (+0.75, 1),
(+1.00, 2), (+1.25, 3), (+1.50, 4), (+1.75, 5), (+2.00, 6) —
beside two **one-tick-early controls** at the extension weights,
(+1.75, 4) and (+2.00, 5). Five predictions were declared in the
source before the extension cells ran. Two held; three cracked,
and the cracks are the finding.

## Novel Trait

**The ridge is a finite segment, and the crown is its eastern
end. Past it, the compensation expires.**

Longest episode, on-line west to east:

| weight | delay | perfect | longest |
|--------|-------|---------|---------|
| +0.50 | 0 | 106/188 | 27 |
| +0.75 | 1 | 80/188 | **12** |
| +1.00 | 2 | 113/188 | 40 |
| +1.25 | 3 | 105/188 | 46 |
| +1.50 | 4 | 134/188 | **55 (crown)** |
| +1.75 | 5 | 90/188 | **16** |
| +2.00 | 6 | 106/188 | 34 |

Controls: (+1.75, 4) longest **25**; (+2.00, 5) longest **34**.

- **No lock, again (P1 HELD).** 9 cells, zero TRUE LOCK 5:3.
- **The anchors reproduce (P2 HELD).** The three on-line cells
  the diagonal already counted keep their numbers exactly:
  40 / 46 / 55. The instruments agree; what follows is not a
  counting difference.
- **The ridge does not continue (P3 CRACKED).** Neither
  extension reaches the plateau line 37: (+1.75, 5) collapses
  to **16**, and (+2.00, 6) recovers only to **34**. One step
  past the crown the line falls off a cliff (55 → 16); the
  partial recovery at +2.00 does not restore it. The crown was
  not a point on a road — it was the segment's end.
- **The western foot is not a foot (P4 CRACKED).** (+0.75, 1)
  falls to **12**, below the isolation line as declared — but
  (+0.50, 0), at delay 0, stands at **27**, above it. The line
  does not rise monotonically out of the west; it dips at
  +0.75 and rises again at +0.50, where the synapse arrives
  with no delay at all. West of the segment there is a ditch,
  then ordinary ground.
- **Arrival stops being the currency (P5 CRACKED).** At the
  extension weights the one-tick-early controls do *not* fall
  below their on-line partners: (+1.75, 4) at **25 beats**
  (+1.75, 5) at 16, and (+2.00, 5) **ties** (+2.00, 6) at 34.
  Inside the segment, dose traded against arrival; past the
  crown, arriving earlier is as good or better — the trade the
  ridge was built on no longer clears.

The segment, whole: it runs from (+1.00, 2) to the crown
(+1.50, 4), three cells long, and it is bounded on both sides —
by a ditch to the west and a cliff to the east. A ridge in the
diagonal's small grid was a segment in this one's longer walk.
Both records are pinned; the segment is the shape that
explains both.

## Falsification Condition

The claim — *the compensation line's tall segment is finite,
running (+1.00, 2) → (+1.50, 4), with a ditch at (+0.75, 1), a
cliff past the crown, and early controls that match or beat
their on-line partners at the extension weights* — is falsified
if:

- any cell reports TRUE LOCK 5:3 — then the pair could hold
  after all, somewhere along the line;
- the anchors fail to reproduce 40 / 46 / 55 — then the
  instruments are not counting the same pair;
- either extension reaches the plateau line 37 on a recount —
  then the ridge continues and the "segment" is this survey's
  artefact;
- the (+1.75, 4) control falls below (+1.75, 5) on a recount —
  then arrival was still the currency past the crown and P5's
  crack was noise.

All pinned by unit tests (6/6), including the full measured
walk — 27, 12, 40, 46, 55, 16, 34 on-line; controls 25 and 34 —
so the record cannot quietly heal itself into a road.

## Usage

```bash
cargo run -p widow-pentad-ridge -- --headless
cargo test -p widow-pentad-ridge
```

*Count everything. Forgive nothing that drifts — including a
ridge whose length was borrowed from the grid that framed it.*
