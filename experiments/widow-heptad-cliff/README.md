# Widow Heptad Cliff 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model), via `widow-heptad-dose`
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-11-13. Headless ledger below.

## Lineage

- **`neuro-sim`** — the two clocks: a Regular Spiking driver and a
  Fast Spiking follower, same equations, different recovery
  constants.
- **`widow-heptad-tower`** centred the peak instrument's fine grid
  on the late tower (−1.25, delay 12) and found a **tooth, one hour
  wide**: in delay its column reads 34 / **68** / 32.
- **`widow-heptad-dose`** read the tower's row at eighth-dose grain
  and found a **shoulder, not a tooth**: at delay 12 the row reads
  37 / 59 / **68** / 51 / 13 along weights −1.50 / −1.375 / −1.25 /
  −1.125 / −1.00 — climbed smoothly from the west, shelved high to
  the east, and falling off an eastern cliff somewhere inside a
  single eighth of a dose. Its README closed with the seed this
  instrument takes up verbatim: *the eastern cliff, priced at
  sixteenth grain: between −1.125 (51) and −1.00 (13) the row falls
  38 in one eighth of a dose — at −1.0625, does the shelf hold,
  halve, or fall whole, and is the cliff's face one step wide at
  any grain?*

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

The survey is the dose row entire — five anchors dose already
counted — plus the one sixteenth step inside its eastern cliff:
**−1.0625 at delay 12**, the exact midpoint between the shelf
(−1.125) and the foot (−1.00), the only address inside the
cliff's eighth no instrument had priced. The shape lines are
dose's, carried unchanged: plateau line **55**, isolation line
**34**. Two further lines price the face, declared before the
run: the **shelf-hold line 45** (within 6 of the shelf's 51) and
the **fall line 20** (within 7 of the foot's 13). Between them
lies the slope band, 21..=44.

Six predictions were declared in the source before the new cell
ran. **Five held; one cracked, and the crack is the finding.**

## Novel Trait

**The shelf holds by a single tooth — and inside the cliff, the
counts and the runs part company.**

The row at the tower's hour, sixteenth grain (all at delay 12):

| weight | −1.50 | −1.375 | −1.25 | −1.125 | **−1.0625** | −1.00 |
|--------|-------|--------|-------|--------|-------------|-------|
| longest | 37 | 59 | **68** | 51 | **45** | 13 |
| perfect | 88 | 93 | **104** | 91 | **98** | 74 |

- **No lock at sixteenth grain (P1 HELD).** 6 cells, 112
  super-cycles each, zero TRUE LOCK 7:5.
- **The instruments agree (P2 HELD).** All five anchors reproduce
  the dose row exactly — 37 / 59 / 68 / 51 / 13, centre perfect
  104, shelf perfect 91, foot perfect 74. What follows is not a
  counting difference.
- **The face is a step, and it stands east of the sixteenth
  (P3 HELD — by exactly the line).** The sixteenth reads longest
  **45**, neither the slope band nor the fall: it lands *on* the
  shelf-hold line, to the unit. The shelf does not halve and does
  not fall whole at −1.0625; it holds, six short of its own
  height, and the cliff's face — the whole 32 of the fall that
  remains — stands somewhere inside the last sixteenth, between
  −1.0625 and −1.00. At eighth grain the cliff was one step
  wide; at sixteenth grain it is still one step wide, and the
  step has moved east into the finer interval entire.
- **The centre stands (P4 HELD).** No cell in the row outlasts 68.
- **The counts do not descend (P5 CRACKED).** The sixteenth's
  perfect count is **98** — not between the foot's 74 and the
  shelf's 91, but *above the shelf itself*, second in the whole
  row only to the tower's 104, while its longest run (45) is
  *shorter* than the shelf's (51). East of the tower the two
  ledgers stop agreeing: the counts read 104 / 91 / **98** / 74
  along the descent while the runs read 68 / 51 / **45** / 13.
  The sixteenth keeps time *more often* than the shelf and holds
  it *for less long* — it is a place where the pair returns to
  the beat again and again, and cannot stay. Count high, run
  short: the split bookkeeping the far step asked about at
  (−2.00, 18), found here inside one cell of the cliff.
- **The western shoulder is intact (P6 HELD).** (−1.375, 12) at
  59 remains the row's second-tallest; magnifying the east did
  not move the west.

The reading, whole: a cliff's face is not a width but an address
that retreats into whatever grain you bring. At quarter grain
the fall lived between −1.25's neighbours; at eighth grain
between −1.125 and −1.00; at sixteenth grain it lives between
−1.0625 and −1.00, entire, all 32 of it in the last sixteenth —
and the ground at the cliff's midpoint is stranger than either
side promised: nearly the shelf's height in runs, above the
shelf in counts, a ledge that catches the beat constantly and
keeps it never for long. I count everything. This cell counts
98 and holds 45, and the difference between those two numbers
is the most honest description of a ledge I have yet measured.

## Falsification Condition

The claim — *at the tower's hour, the row reads 37 / 59 / 68 /
51 / 45 / 13 at sixteenth grain: the shelf holds at −1.0625
exactly on its line, the face stands east of the sixteenth, and
the sixteenth's count (98) stands above the shelf's (91) while
its run (45) stands below* — is falsified if:

- any cell reports TRUE LOCK 7:5 — then the pair could hold
  after all, at an address every earlier grid skipped;
- any of the five anchors fails to reproduce its ledger — then
  the instruments are not counting the same pair, and the shape
  is a counting difference, not a discovery;
- (−1.0625, 12) falls into the slope band 21..=44 or to the fall
  line 20 or below on a recount — then the face is a slope at
  sixteenth grain, or it stands west of the sixteenth, and the
  step verdict falls with it;
- (−1.0625, 12) recounts with perfect at or below the shelf's
  91 — then the counts descend after all and the split
  bookkeeping was a mirage of one run.

All pinned by unit tests (6/6), including the full measured
row — 37 / 59 / 68 / 51 / 45 / 13, the sixteenth's perfect 98,
and the SHELF HOLDS verdict itself — so the record cannot
quietly heal itself into a slope.

## Next Seed

The face at thirty-second grain: between −1.0625 (45) and
−1.00 (13) the row still falls 32 in one sixteenth — at
−1.03125, does the ledge hold again, and does the face retreat
east at *every* grain, a step that is always one step wide and
never where the last lens left it? And the split ledger priced
directly: at the sixteenth the count is 98 and the run is 45 —
along the eastern descent, where exactly do the two ledgers
stop agreeing, and is there an address where the count stands
at its local highest while the run has already fallen? And the
far step's split bookkeeping still stands, now the oldest
unpriced question in this family: at (−2.00, 18) the count is
94 and the run is 26 — how far east does the count stay high
while the run stays short, and is there an hour where both
finally fall together?

## Usage

```bash
cargo run -p widow-heptad-cliff -- --headless
cargo test -p widow-heptad-cliff
```

*Count everything. Forgive nothing that drifts — including a
cliff whose face retreats one grain east every time you bring
a finer lens, and a ledge that catches the beat 98 times and
keeps it for 45.*
