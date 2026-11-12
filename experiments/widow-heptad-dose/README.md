# Widow Heptad Dose 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-11-12. Headless ledger below.

## Lineage

- **`neuro-sim`** — the two clocks: a Regular Spiking driver and a
  Fast Spiking follower, same equations, different recovery
  constants.
- **`widow-heptad`** took up the near-**7:5** pair (FS I=4.8): the
  follower runs *early*, the medicine is inhibition, and along one
  row the firm hand (−1.5, delay 8) led at longest 45.
- **`widow-heptad-late`** priced the one-tick-late cells and found
  a tower where the ridge had drawn a cliff: **(−1.25, delay 12)**
  stands at longest **68** with perfect **104/112**, above the
  crown (−1.00, 10) itself.
- **`widow-heptad-tower`** centred the peak instrument's fine grid
  on the tower and found a **tooth, one hour wide**: in delay its
  column reads 34 / **68** / 32, walled on both sides, no diagonal
  lean, no companion. Its README closed with two seeds; the far
  step's split bookkeeping was the first. This instrument takes
  the second, in the tower's own words: *a finer grain on the
  tower itself — eighth-dose steps at its own hour (weights
  −1.375, −1.125, delay 12): is the tooth also one dose wide, or
  does the weight axis keep a skirt the delay axis refused?*
  The weight axis keeps more than a skirt. The row is below.

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

The survey is one row of the tower's grid read at twice the
grain: **five weights at the tower's own hour, delay 12** —
−1.50, **−1.375**, −1.25, **−1.125**, −1.00. Three are anchors
the tower grid already counted: the western quarter neighbour
at 37, the centre at 68 with perfect 104, the eastern quarter
neighbour at 13. Two are new: the eighth steps either side of
the centre, the only addresses at this hour no instrument had
priced. The shape lines are the tower grid's, carried
unchanged: plateau line **55** (80% of 68, rounded up);
isolation line **34** (50% of 68).

Six predictions were declared in the source before the new
cells ran. **Four held; two cracked, and the cracks are the
finding.**

## Novel Trait

**In dose, the tooth is a shoulder — the weight axis keeps what
the delay axis refused, on both sides.**

Longest episode along the row (all at delay 12):

| weight | −1.50 | −1.375 | −1.25 | −1.125 | −1.00 |
|--------|-------|--------|-------|--------|-------|
| longest | 37 | **59** | **68** | **51** | 13 |
| perfect | 88 | 93 | **104** | 91 | 74 |

- **No lock at eighth grain (P1 HELD).** 5 cells, 112
  super-cycles each, zero TRUE LOCK 7:5.
- **The instruments agree (P2 HELD).** All three anchors
  reproduce the tower grid's row exactly — 37 / 68 / 13, tower
  perfect 104. What follows is not a counting difference.
- **The tooth is not one dose wide (P3 CRACKED).** The western
  eighth step, (−1.375, 12), stands at longest **59** — above
  the plateau line 55 the delay axis never let a neighbour
  approach. Half a quarter-dose west of the tower, the ground
  is nearly the tower's own height: the row climbs 37 → 59 →
  68, a slope, not a wall. In delay the tower was walled at
  34 and 32; in dose it is shouldered. The verdict is
  **SHOULDER IN DOSE**.
- **The centre stands (P4 HELD).** No cell in the row outlasts
  68; the shoulder approaches the tower and does not pass it.
- **The skirt is western (P5 HELD).** 59 west against 51 east —
  the asymmetry the quarter grid showed (37 over 13) survives
  at eighth grain, narrowed but not closed.
- **The east is not ordinary ground either (P6 CRACKED).**
  The eastern eighth, (−1.125, 12), stands at **51** — far
  above the isolation line 34, four short of the plateau line.
  The quarter grid's eastern 13 was not the tower's eastern
  face; it was the far side of a cliff whose edge stands
  between −1.125 and −1.00. East of the tower the row reads
  68 → 51 → 13: a high shelf, then the fall.

The reading, whole: the tower's shape depends on the axis you
magnify. In time it is a tooth one hour wide, walled on both
sides; in dose it is a shoulder — climbed smoothly from the
west, shelved high to the east, and falling off its eastern
cliff somewhere inside a single eighth of a dose. A champion's
width is not a property of the champion. It is a property of
the axis, and each axis must be asked separately, at its own
grain.

## Falsification Condition

The claim — *at the tower's hour, the dose row reads 37 / 59 /
68 / 51 / 13: a western shoulder above the plateau line, an
eastern shelf above the isolation line, and the centre
unpassed* — is falsified if:

- any cell reports TRUE LOCK 7:5 — then the pair could hold
  after all, at an address every earlier grid skipped;
- any of the three anchors fails to reproduce its ledger —
  then the instruments are not counting the same pair, and
  the shape is a counting difference, not a discovery;
- (−1.375, 12) falls below the plateau line 55 on a recount —
  then the tooth is one dose wide after all and the shoulder
  verdict falls with it;
- (−1.125, 12) falls to the isolation line 34 or below on a
  recount — then the east is ordinary ground and only the
  cliff between the eighths remains to be priced.

All pinned by unit tests (6/6), including the full measured
row — 37 / 59 / 68 / 51 / 13, the eighths' perfect counts
93 and 91, and the SHOULDER IN DOSE verdict itself — so the
record cannot quietly heal itself into a tooth.

## Next Seed

The eastern cliff, priced at sixteenth grain: between
−1.125 (51) and −1.00 (13) the row falls 38 in one eighth of
a dose — at −1.0625, does the shelf hold, halve, or fall
whole, and is the cliff's face one step wide at any grain?
And the far step's split bookkeeping still stands, now the
oldest unpriced question in this family: at (−2.00, 18) the
count is 94 and the run is 26 — how far east does the count
stay high while the run stays short, and is there an hour
where both finally fall together?

## Usage

```bash
cargo run -p widow-heptad-dose -- --headless
cargo test -p widow-heptad-dose
```

*Count everything. Forgive nothing that drifts — including a
tower that was one hour wide and a whole shoulder broad,
depending on which way you turned the lens.*
