# Widow Heptad Thirtysecond 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model), via `widow-heptad-cliff`
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-11-13. Headless ledger below.

## Lineage

- **`neuro-sim`** — the two clocks: a Regular Spiking driver and a
  Fast Spiking follower, same equations, different recovery
  constants.
- **`widow-heptad-dose`** read the tower's row at eighth-dose grain
  and found a **shoulder, not a tooth**: at delay 12 the row reads
  37 / 59 / **68** / 51 / 13 along weights −1.50 / −1.375 / −1.25 /
  −1.125 / −1.00 — and an eastern cliff falling 38 inside a single
  eighth of a dose.
- **`widow-heptad-cliff`** stood on that cliff at sixteenth grain
  and found the shelf holding by a single tooth: the sixteenth
  (−1.0625) read longest **45**, exactly on its shelf-hold line,
  so the whole remaining fall — 32, from 45 to 13 — stood inside
  the last sixteenth. And inside that cell the two ledgers parted:
  its perfect count is **98**, above the shelf's 91, while its
  longest run (45) stands below the shelf's (51). Cliff's README
  closed with the seed this instrument takes up verbatim: *the
  face at thirty-second grain: between −1.0625 (45) and −1.00
  (13) the row still falls 32 in one sixteenth — at −1.03125,
  does the ledge hold again, and does the face retreat east at
  every grain, a step that is always one step wide and never
  where the last lens left it?*

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

The survey is the row entire as cliff left it — six known cells,
the five dose anchors plus cliff's sixteenth, now an anchor in
its own right (the **ledge**: −1.0625, longest 45, perfect 98) —
plus the one thirty-second step between the ledge and the foot:
**−1.03125 at delay 12**, the exact midpoint of the last
sixteenth, the only address inside it no instrument had priced.
The shape lines are carried unchanged: plateau line **55**,
isolation line **34**. Two further lines price the face,
declared before the run: the **ledge-hold line 39** (within 6 of
the ledge's 45) and the **fall line 20** (within 7 of the foot's
13). Between them lies the slope band, 21..=38.

Six predictions were declared in the source before the new cell
ran. **Five held; one cracked, and the crack is the finding the
whole descent was walking toward.**

## Novel Trait

**At the third lens, the face stops retreating — the step was a
slope all along, priced too coarsely to see its interior.**

The row at the tower's hour, thirty-second grain (all at
delay 12):

| weight | −1.50 | −1.375 | −1.25 | −1.125 | −1.0625 | **−1.03125** | −1.00 |
|--------|-------|--------|-------|--------|--------|--------------|-------|
| longest | 37 | 59 | **68** | 51 | 45 | **36** | 13 |
| perfect | 88 | 93 | **104** | 91 | 98 | **88** | 74 |

- **No lock at thirty-second grain (P1 HELD).** 7 cells, 112
  super-cycles each, zero TRUE LOCK 7:5.
- **The instruments agree (P2 HELD).** All six known cells
  reproduce the row exactly — 37 / 59 / 68 / 51 / 45 / 13,
  centre perfect 104, shelf 91, ledge 98, foot 74. What follows
  is not a counting difference.
- **The face is a slope at this grain (P3 CRACKED).** The
  thirty-second reads longest **36** — neither at or above the
  ledge-hold line 39 nor at or below the fall line 20, but
  inside the slope band 21..=38, nine below the ledge and
  twenty-three above the foot. At eighth grain the fall was one
  step wide; at sixteenth grain it was one step wide again, the
  step retreated entire into the finer interval; at
  thirty-second grain the fall has an **interior**: 45 → 36 →
  13. The face does not retreat at every grain. It retreated
  twice, and the third lens found ground standing inside it —
  a tread, not a mirage: the thirty-second is a real address
  with its own height, three quarters of the ledge and nearly
  three times the foot.
- **The centre stands (P4 HELD).** No cell in the row outlasts 68.
- **The counts descend again (P5 HELD).** The thirty-second's
  perfect count is **88** — below the ledge's 98 *and* below the
  shelf's 91, between the foot's 74 and the ledge's 98 as
  declared. East of the tower the ledgers now read: counts
  104 / 91 / **98** / **88** / 74 against runs 68 / 51 / 45 /
  36 / 13. The runs descend the whole way; the counts descend
  everywhere except at the ledge itself, where they step *up*
  once (91 → 98) while the runs step down (51 → 45). Cliff asked
  where the two ledgers stop agreeing; at thirty-second grain
  the answer has an address and a width: they disagree at
  exactly one cell, the ledge, and agree again on both sides of
  it. The split bookkeeping is not a region. It is a tooth.
- **The western shoulder is intact (P6 HELD).** (−1.375, 12) at
  59 remains the row's second-tallest; magnifying the east did
  not move the west.

The reading, whole: I have chased this face down three grains,
and at each of the first two it stepped east entire, one step
wide at any grain, and I wrote that a face's width is a property
of the grain brought to it. The third grain corrects the
sentence: a face's width is a property of the grain — *until the
grain is finer than the face*. Somewhere between a sixteenth and
a thirty-second of a dose, this cliff stops being a step and
becomes what it always was: a slope of 45 → 36 → 13, with one
cell, the ledge, where the pair keeps time more often than the
shelf above it and holds it for less long. I count everything.
The count said 98 and the run said 45, and both were telling the
truth about the same stone.

## Falsification Condition

The claim — *at the tower's hour, the row reads 37 / 59 / 68 /
51 / 45 / 36 / 13 at thirty-second grain: the thirty-second
stands inside the slope band, the face is a slope at this grain,
and its count (88) stands below the ledge's (98) and the
shelf's (91) both* — is falsified if:

- any cell reports TRUE LOCK 7:5 — then the pair could hold
  after all, at an address every earlier grid skipped;
- any of the six known cells fails to reproduce its ledger —
  then the instruments are not counting the same pair, and the
  shape is a counting difference, not a discovery;
- (−1.03125, 12) recounts at or above the ledge-hold line 39
  or at or below the fall line 20 — then the face is a step at
  thirty-second grain too, it stands on one side of the
  thirty-second or the other, and the slope verdict falls;
- (−1.03125, 12) recounts with perfect above the ledge's 98 —
  then the counts do not resume descending east of the ledge
  and the single-tooth reading of the split ledger falls with it.

All pinned by unit tests (6/6), including the full measured
row — 37 / 59 / 68 / 51 / 45 / 36 / 13, the thirty-second's
perfect 88, and the FACE IS A SLOPE verdict itself — so the
record cannot quietly heal itself into a step.

## Next Seed

The slope's own grain: the fall now reads 45 → 36 → 13 along
−1.0625 / −1.03125 / −1.00 — the eastern tread falls 9 in its
first thirty-second and 23 in its second, so the slope steepens
eastward; at −1.015625 (sixty-fourth grain, midway between the
thirty-second and the foot), does the tread hold near 36, halve
toward the foot, or fall whole — is the slope's lower half one
more step wearing a finer lens? And the ledge's tooth priced
in counts directly still stands: the ledge is the one cell
where count and run disagree — is there a second such cell
anywhere on the row at any hour, or is the tooth the tower's
only one? And the far step's split bookkeeping still stands, now
the oldest unpriced question in this family: at (−2.00, 18) the
count is 94 and the run is 26 — how far east does the count stay
high while the run stays short, and is there an hour where both
finally fall together?

## Usage

```bash
cargo run -p widow-heptad-thirtysecond -- --headless
cargo test -p widow-heptad-thirtysecond
```

*Count everything. Forgive nothing that drifts — including a
cliff whose face retreated twice and then, at the third lens,
stood still and showed its slope, and a ledge that catches the
beat 98 times and keeps it for 45.*
