# Widow Heptad Steep 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-11-10. Headless ledger below.

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
- **`widow-heptad-diagonal`** traced a compensation line through
  the crown — heavier doses arriving later balancing lighter
  doses arriving sooner — and **`widow-heptad-ridge`** walked it
  out of the grid and found a **finite segment**: a cliff past
  the crown (58 → **34** at (−1.25, 11), **37** at (−1.50, 12)).
- **`widow-heptad-late`** priced the crown's column and the
  one-tick-late cells, and the cliff turned out to be a property
  of the *line's slope*: (−1.25, delay 12) — one tick late —
  stands at longest **68** with perfect **104/112**, above the
  crown itself. Its README named this instrument outright:
  *follow the steeper exchange rate — the line through the crown
  (−1.00, 10) and the late tower (−1.25, 12) runs two ticks per
  quarter-dose; its next cell east is (−1.50, delay 14). Does a
  second tower stand there, and is the "segment" a longer road
  on a steeper slope?* No second tower stands. The line was a
  chord.

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

The survey walks the **steep line** end to end — the line
through the crown (−1.00, 10) and the late tower (−1.25, 12),
two ticks later per quarter-dose heavier — and keeps the old
ground beside it. Five cells are **anchors** earlier instruments
already counted: the crown (−1.00, 10) at 58, the tower's
on-line partner (−1.25, 11) at 34, the tower (−1.25, 12) at 68,
and the −1.50 column's known hours (−1.50, 12) at 37 and
(−1.50, 13) at 25. Four hours are new, all on the line: the
western step **(−0.75, 8)**, the seed's cell **(−1.50, 14)**,
and the two eastern steps **(−1.75, 16)** and **(−2.00, 18)**.

Lines declared before the survey ran, from the late tower at
longest 68: plateau line **55** (80% of 68, rounded up);
isolation line **34** (50% of 68). Six predictions were
declared in the source before the new cells ran. Four held;
the two that cracked were the road itself.

## Novel Trait

**A line through two heights is a chord, not a road — the
steep line's third point falls to the ordinary ground of its
own column.**

The steep line, longest episode west to east:

| weight | delay | longest | perfect | |
|--------|-------|---------|---------|--|
| −0.75 | 8 | 34 | 78/112 | new — western step |
| −1.00 | 10 | **58** | 92/112 | crown (anchor) |
| −1.25 | 12 | **68** | 104/112 | late tower (anchor) |
| −1.50 | 14 | 38 | 86/112 | new — the seed's cell |
| −1.75 | 16 | 17 | 61/112 | new |
| −2.00 | 18 | 26 | 94/112 | new — far step |

- **No lock, again (P1 HELD).** 9 cells, 112 super-cycles
  each, zero TRUE LOCK 7:5.
- **The instruments agree (P2 HELD).** All five anchors
  reproduce their ledgers exactly — 58 / 34 / 68 / 37 / 25,
  crown perfect 92, tower perfect 104. What follows is not a
  counting difference.
- **No second tower (P3 CRACKED).** The seed's cell
  (−1.50, delay 14) stands at longest **38** — below the
  plateau line 55, and within one super-cycle of its own
  column's ordinary on-line hour, (−1.50, 12) at 37. The
  address the steep line assigned it is worth nothing its
  column did not already pay at gentler hours (37 at delay
  12, 25 at delay 13, 38 at delay 14 — the column is a
  shallow bowl, not a slope). Two collinear heights were a
  coincidence of two points; the third point declines to
  join them.
- **The road does not continue east (P4 CRACKED).** One step
  further, (−1.75, 16) falls to **17**, below the isolation
  line 34 and no higher than the uncoupled pair's own
  longest. Whatever stands at the crown and the tower does
  not extend past the tower in either the old slope or the
  steep one.
- **The ends behave as declared (P5, P6 HELD).** The western
  step (−0.75, 8) stands at **34**, exactly the isolation
  line, below the plateau — the steep line inherits nothing
  west of the crown. The far step (−2.00, 18) stands at
  **26**, below isolation, as a segment's end should — but
  its perfect count, **94/112**, is the survey's second
  highest, above the crown's own 92: the far hour keeps the
  count and loses the run, as hours at the ends of this
  family's roads have done before. Count and duration part
  company at both ends of the line.

The reading, whole: the late tower is not the second point of
a longer road. It is a tower — one address, standing alone,
with the crown for a neighbour and ordinary ground on every
side the lines have walked. The "segment" the ridge found and
the "steeper road" the late instrument proposed were both the
shadows of lines drawn through heights; the ground east of
the crown holds two heights and, between and beyond them, a
bowl and a fall. A fine grid around the tower — the late
instrument's other seed — is now the honest next question:
not which road leads away from 68, but how wide 68 itself is.

## Falsification Condition

The claim — *the steep line through the crown and the late
tower is a chord, not a road: its next cell east (−1.50, 14)
stands at longest 38, its further steps at 17 and 26, and its
western step at 34* — is falsified if:

- any cell reports TRUE LOCK 7:5 — then the pair could hold
  after all, at an address every earlier grid skipped;
- any of the five anchors fails to reproduce its ledger —
  then the instruments are not counting the same pair, and
  the chord is a counting difference, not a discovery;
- (−1.50, 14) reaches the plateau line 55 on a recount —
  then a second tower stands after all and the road reading
  revives;
- (−1.75, 16) reaches the isolation line 34 on a recount —
  then the road continues east diminished, as declared, and
  this instrument stopped one step too early in its doubt.

All pinned by unit tests (6/6), including the full measured
survey — the line 34 / 58 / 68 / 38 / 17 / 26 west to east,
and the far step's perfect 94 — so the record cannot quietly
heal itself into a road.

## Next Seed

The tower deserves its own fine grid, as the crown had: centred
on (−1.25, 12) — the late instrument's second seed, now the only
road left — is 68 a tooth, a plateau, or a shoulder of something
taller still? And the far step's split bookkeeping stands open:
at (−2.00, 18) the count is 94 and the run is 26 — how far east
does the count stay high while the run stays short, and is there
an hour where both finally fall together?

## Usage

```bash
cargo run -p widow-heptad-steep -- --headless
cargo test -p widow-heptad-steep
```

*Count everything. Forgive nothing that drifts — including a
line you drew through two heights and called a road, at a third
ratio, two ticks per quarter-dose.*
