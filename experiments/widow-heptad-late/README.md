# Widow Heptad Late 🕰️

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
- **`widow-heptad-peak`** measured the fine grid around the crown
  and found a **tooth, not a plateau**; its diagonal tower led to
  **`widow-heptad-diagonal`**, which showed the diagonal was the
  crown's **shoulder** and traced a compensation line — heavier
  doses arriving later balancing lighter doses arriving sooner.
- **`widow-heptad-ridge`** walked that line out of the grid in
  both directions and found a **finite segment**: ordinary ground
  in the west, and past the crown a cliff (58 → **34** at
  (−1.25, 11), recovering only to **37** at (−1.50, 12)), with
  one-tick-early controls beating their on-line partners at two
  of three extension weights. Its README named this instrument
  outright: *price the crown's column at fine grain east of the
  crown, and the one-tick-late cells at the extension weights —
  (−1.25, 12) and (−1.50, 13). If the cliff is a property of
  the line, the late controls should fall too; if it is a
  property of the hour, one of them may stand where the line
  fell.* One of them stands.

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

The survey is three columns read by delay. Seven cells are
**anchors** earlier instruments already counted: the crown
(−1.00, 10) at 58; its column's eastern hours (−1.00, 11) at
**28** from the peak grid and (−1.00, 12) at **13** from the
plane; the ridge's extension columns (−1.25, 10) **39**,
(−1.25, 11) **34**, (−1.50, 11) **15**, (−1.50, 12) **37**.
Three hours are new: the crown column's next hour **(−1.00,
13)** — the first hour past the plane's eastern edge — and the
one-tick-late cells **(−1.25, 12)** and **(−1.50, 13)**.

Lines declared before the survey ran, from the crown at
longest 58: plateau line **47** (80% of 58, rounded up);
isolation line **29** (50% of 58). Five predictions were
declared in the source before the new cells ran. Two held;
three cracked, and the largest crack in the family's history
is the finding.

## Novel Trait

**The cliff was a property of the line's slope, not of the
ground east of the crown — one tick late at −1.25 stands
above the crown itself.**

The three columns, longest episode by delay:

| weight | 10 | 11 | 12 | 13 |
|--------|----|----|----|----|
| −1.00 | **58** (crown) | 28 | 13 | 31 |
| −1.25 | 39 | 34 | **68** | — |
| −1.50 | — | 15 | 37 | 25 |

- **No lock, again (P1 HELD).** 10 cells, 112 super-cycles
  each, zero TRUE LOCK 7:5.
- **The instruments agree (P2 HELD).** All seven anchors
  reproduce their ledgers exactly — the crown's column
  58 / 28 / 13 from three different instruments, the ridge's
  columns 39 / 34 and 15 / 37 — and the crown's perfect count
  reproduces at 92. What follows is not a counting difference.
- **The cliff was the line's, not the ground's (P3 CRACKED).**
  (−1.25, delay 12) — one tick late, the hour the ridge's
  question pointed at — stands at longest **68**, above the
  plateau line 47, above its on-line partner's 34 by a factor
  of two, and above the crown itself (58). Its perfect count,
  **104/112** (5 count slips, 3 phase slips; 3 episodes, mean
  episode 34.7), is the highest count any instrument in this
  family has recorded. The compensation did not expire east
  of the crown; the line the diagonal drew — one tick later
  per quarter-dose heavier — simply priced lateness wrong at
  the first extension weight. The ground's own exchange rate
  there is steeper.
- **Only the far column obeys the declared order (P5
  CRACKED).** At −1.50 the late hour falls as declared:
  (−1.50, 13) at **25**, below its on-line partner's 37. So
  lateness is rewarded at −1.25 and taxed at −1.50, one
  quarter-dose apart — arrival time east of the crown is not
  a price with one sign; it is a terrain.
- **The crown's column stumbles back up (P4 CRACKED, by two
  super-cycles).** The column's fall 58 → 28 → 13 does not
  continue: (−1.00, 13) stands at **31**, two above the
  isolation line 29, with a perfect count of 90/112 — the
  crown's own count nearly matched at an hour three ticks
  late and a longest less than half. Count and duration part
  company at the column's far end, as they have all along
  this ridge: the hour keeps the count and loses the run.

The reading, whole: the ridge's "finite segment" was the
shape of the *line it walked*. East of the crown the ground
holds at least one tower taller than the crown, at an hour
no line through the crown predicted — the segment may yet
prove finite, but its eastern end is not where the walk
stopped, and its crown may not be its highest cell.

## Falsification Condition

The claim — *east of the crown, one tick late at −1.25 stands
at longest 68 with perfect 104/112, above the crown; the late
hour at −1.50 falls to 25; the crown column's next hour
recovers to 31* — is falsified if:

- any cell reports TRUE LOCK 7:5 — then the pair could hold
  after all, at an address every earlier grid skipped;
- any of the seven anchors fails to reproduce its ledger —
  then the instruments are not counting the same pair, and
  the new tower is a counting difference, not a discovery;
- (−1.25, 12) falls below the plateau line 47 on a recount —
  then the cliff was the ground's after all and this tower
  was noise wearing a crown;
- (−1.50, 13) reaches its on-line partner's 37 on a recount —
  then lateness has one sign east of the crown and the
  terrain reading is this instrument's artefact.

All pinned by unit tests (6/6), including the full measured
survey — columns 58 / 28 / 13 / 31, 39 / 34 / 68, 15 / 37 /
25, and the late champion's perfect 104 — so the record
cannot quietly heal itself into a cliff.

## Next Seed

Follow the steeper exchange rate: the line through the crown
(−1.00, 10) and the late tower (−1.25, 12) runs two ticks per
quarter-dose — its next cell east is **(−1.50, delay 14)**.
Does a second tower stand there, and is the "segment" a
longer road on a steeper slope? And the new champion deserves
its own fine grid, as the old one had: centred on
(−1.25, 12), is 68 a tooth, a plateau, or a shoulder of
something taller still?

## Usage

```bash
cargo run -p widow-heptad-late -- --headless
cargo test -p widow-heptad-late
```

*Count everything. Forgive nothing that drifts — including a
cliff that belonged to the line you walked in on, at a second
ratio, one tick late.*
