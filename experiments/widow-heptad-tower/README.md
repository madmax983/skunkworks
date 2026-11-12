# Widow Heptad Tower 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-11-11. Headless ledger below.

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
- **`widow-heptad-peak`** measured the fine grid around the crown
  and found a **tooth, not a plateau**. Its method — a fine grid
  centred on the champion, shape lines declared before the
  survey — is this instrument's method.
- **`widow-heptad-ridge`** walked a compensation line out of the
  grid and found a cliff past the crown; **`widow-heptad-late`**
  priced the one-tick-late cells and found the cliff was a
  property of the line's slope: **(−1.25, delay 12)** — one tick
  late — stands at longest **68** with perfect **104/112**, above
  the crown itself.
- **`widow-heptad-steep`** followed the steep line through crown
  and tower and found a chord, not a road. Its README named this
  instrument outright, in the late instrument's own words: *the
  tower deserves its own fine grid, as the crown had: centred on
  (−1.25, 12) — is 68 a tooth, a plateau, or a shoulder of
  something taller still?* A tooth. The grid is below.

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

The survey is the peak instrument's fine grid moved to the
tower: **3 weights × 3 delays = 9 cells** — weights −1.50,
−1.25, −1.00 (quarter steps); delays 11, 12, 13 (single ticks) —
centred on (−1.25, 12). Eight of the nine cells are **anchors**
earlier instruments already counted — the −1.50 column from the
ridge and the late instrument, the tower and its western hour
from three instruments, the −1.00 column from the peak grid, the
plane, and the late instrument — assembled as one grid around
this centre for the first time. One hour is new: **(−1.25,
delay 13)**, the tower's own column one tick east, the only
neighbour of the centre no instrument had priced.

**Shape definitions, declared before the survey ran** (centre
longest C = 68): plateau line **55** (80% of 68, rounded up);
isolation line **34** (50% of 68). PLATEAU — at least 2 of the
centre's 4 orthogonal neighbours reach 55; TOOTH — no orthogonal
neighbour reaches 55; ISOLATED TOOTH — a tooth whose every
neighbour, diagonal included, falls at or below 34; SHOULDER —
anything else. Five predictions were declared in the source
before the new cell ran. **All five held** — the first clean
sweep in this family's recent memory, and the grid still has
things to say.

## Novel Trait

**The tower is a tooth one hour wide — and unlike the crown, it
does not lean.**

Longest episode per fine cell (delays 11 / 12 / 13):

| weight | 11 | 12 | 13 |
|--------|----|----|----|
| −1.50 | 15 | 37 | 25 |
| −1.25 | 34 | **68** | 32 |
| −1.00 | 28 | 13 | 31 |

- **No lock at fine grain, at a second centre (P1 HELD).**
  9 cells, 112 super-cycles each, zero TRUE LOCK 7:5.
- **The instruments agree (P2 HELD).** All eight anchors
  reproduce their scattered ledgers exactly — 15 / 37 / 25,
  34 / 68, 28 / 13 / 31 — with the tower's perfect 104, the
  crown column's late hour at 90, and (−1.50, 13) at 77. What
  follows is not a counting difference.
- **A tooth (P3 HELD).** The four orthogonal neighbours stand
  at **37, 13, 34, 32** — none reaches the plateau line of 55;
  the tallest is barely past half the tower's height. The
  verdict is TOOTH, kept from ISOLATED TOOTH by a single
  neighbour: (−1.50, 12) at 37 stands three above the isolation
  line, the tower's only skirt, and it lies to the *west*, in
  the heavier dose at the tower's own hour.
- **The centre stands (P4 HELD).** No fine cell outlasts 68;
  the next tallest cell in the grid is 37. The tower is not a
  shoulder of anything taller — there is nothing taller within
  a quarter-dose or a tick in any direction.
- **One hour wide (P5 HELD, by two super-cycles).** The one
  new hour, (−1.25, delay 13), stands at longest **32** — two
  below the isolation line, less than half the tower — with
  perfect 80/112 (8 episodes, mean 10.0). One tick late at
  this weight was rewarded beyond the crown; two ticks late
  is taxed like everywhere else in the garden. West of the
  tower the same shape repeats: 34 at delay 11, exactly the
  isolation line. In delay, the tower's column reads
  34 / **68** / 32 — a single hour, walled on both sides.
- **No diagonal lean.** The crown's fine grid hid its second
  tower on the diagonal — (−0.75, 9) at 51, lighter-and-sooner,
  the lean two ratios shared. The tower's diagonals stand at
  **15, 25, 28, 31** — every one at or below the isolation
  line. Whatever carries the lean, the tower does not: it
  rises from ordinary ground on all eight sides, and the
  grid's mean longest, **31.4**, is ordinary ground's own
  average. The crown was a tooth on a ridge; the tower is a
  tooth on a plain.

The reading, whole: two heights east of the crown have now
been measured at fine grain. The crown leans — its grid kept a
diagonal companion and a compensation line. The tower does
not lean, keeps no companion, and is exactly one hour wide.
A height can be a place the ground rises toward, or a place
where one hour happens to agree with one dose. The grid
cannot tell the tower's *cause*; it can only testify that
68 has no neighbours, and now it has.

## Falsification Condition

The claim — *at fine grain the late tower (−1.25, delay 12) is
a tooth: no orthogonal neighbour reaches 55, no cell outlasts
68, its diagonals all fall at or below 34, and its own next
hour stands at 32* — is falsified if:

- any fine cell reports TRUE LOCK 7:5 — then the pair could
  hold after all, at an address every earlier grid skipped;
- any of the eight anchors fails to reproduce its ledger —
  then the instruments are not counting the same pair, and
  the shape is a counting difference, not a discovery;
- (−1.25, 13) reaches the plateau line 55 on a recount — then
  the tower is at least two hours wide and the tooth verdict
  falls with it;
- any diagonal reaches the plateau line on a recount — then
  the tower leans after all, and the plain reading is this
  instrument's artefact.

All pinned by unit tests (6/6), including the full measured
grid — rows 15 / 37 / 25, 34 / 68 / 32, 28 / 13 / 31, the new
hour's perfect 80, and the TOOTH verdict itself — so the
record cannot quietly heal itself into a plateau.

## Next Seed

The far step's split bookkeeping still stands, now the oldest
unpriced question in this family: at (−2.00, 18) the count is
94 and the run is 26 — how far east does the count stay high
while the run stays short, and is there an hour where both
finally fall together? And a finer grain on the tower itself
— eighth-dose steps at its own hour (weights −1.375, −1.125,
delay 12): is the tooth also one *dose* wide, or does the
weight axis keep a skirt the delay axis refused?

## Usage

```bash
cargo run -p widow-heptad-tower -- --headless
cargo test -p widow-heptad-tower
```

*Count everything. Forgive nothing that drifts — including a
tower that kept no neighbours, at a second centre, one hour
wide.*
