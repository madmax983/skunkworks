# Widow Heptad Diagonal 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-11-08. Headless ledger below.

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
  champion and found a **tooth, not a plateau** — and one
  surprise: the grid's second tower was its diagonal,
  **(−0.75, delay 9) at 51**, above the plateau line, at the
  lighter dose and the earlier hour — the same lean the 5:3
  crown showed. Its README named this instrument outright:
  *a fine grid centred on (−0.75, delay 9) itself — is the
  diagonal a second tooth, or the crown's shoulder seen from
  the wrong centre?*
- **`widow-pentad-diagonal`** answered that exact question for
  the 5:3 pair: the diagonal was the crown's **shoulder**, and
  the tall cells traced a ridge. This instrument recentres the
  fine grid on the 7:5 diagonal and counts again, so the two
  ratios' answers can be compared cell for cell.

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
The survey is **3 weights × 3 delays = 9 cells** — weights
−1.00, −0.75, −0.50 (quarter steps); delays 8, 9, 10 (single
ticks) — each counted over **24,000 ticks** (first 1,500
discarded; 112 super-cycles), beside the uncoupled pair on the
same window (83/112 perfect, longest 17). The champion
(−1.0, delay 10) stands in this grid as a diagonal neighbour
of the centre.

**Shape definitions, declared before the survey ran** (centre
longest C = 51): plateau line 41 (80% of C); isolation line 25
(50% of C); SHOULDER OF THE CROWN — some neighbour exceeds the
centre; PLATEAU — at least 2 orthogonal neighbours reach 41 and
none exceeds the centre; ISOLATED TOOTH — no orthogonal reaches
41, every neighbour at or below 25, none exceeds the centre;
TOOTH (with skirts) — no orthogonal reaches 41, none exceeds
the centre, but some neighbour stands above 25; SHOULDER —
anything else.

Five predictions were declared in the source before the survey
ran. All five held.

## Novel Trait

**The 7:5 diagonal was the crown's shoulder too — and the lean
stops at the diagonal: lighter still buys nothing.**

Longest episode per cell (delays 8 / 9 / 10):

| weight | 8 | 9 | 10 |
|--------|---|---|----|
| −1.00 | 22 | 39 | **58** |
| −0.75 | 34 | **51** | 20 |
| −0.50 | 21 | 33 | 32 |

- **No lock, again (P1 HELD).** 9 cells, 112 super-cycles each,
  zero TRUE LOCK 7:5.
- **The instruments agree (P2 HELD).** The centre reproduces
  the peak grid's diagonal exactly: perfect **86/112** (10 count
  slips, 16 phase slips), longest **51**, mean episode 10.8.
- **Dethroned by its own corner (P3 HELD).** The diagonal
  neighbour (−1.0, delay 10) — the champion itself — reproduces
  its ledger exactly (perfect **92/112**, longest **58**, mean
  episode 18.4) and exceeds the centre. Shape verdict:
  **SHOULDER OF THE CROWN**. The peak grid's "second tower" was
  never a second tooth; it was a point on the crown's slope,
  seen from a grid that put it near a corner — the 5:3
  diagonal's verdict, repeated at a second ratio under the
  opposite medicine.
- **The measured cross holds (P4 HELD).** The two orthogonals
  the peak grid already counted keep their numbers: (−1.0,
  delay 9) at **39** and (−0.75, delay 10) at **20**, both below
  the plateau line of 41. No orthogonal reaches 41 at all
  (39 / 34 / 20 / 33).
- **Good ground (P5 HELD).** The grid's mean longest episode
  is **34.4**, above the plane's inhibitory mean of 23.4.
- **The ridge, and where it ends, kept honestly:** read the
  tall cells along the compensation diagonal — (−0.50, 8) 21,
  (−0.75, 9) 51, (−1.00, 10) 58 — and they climb through the
  centre to the crown, heavier doses arriving later balancing
  lighter doses arriving sooner, as the 5:3 ridge did. But
  this ridge does not extend past the centre toward lighter
  and sooner: the whole −0.50 row stands at 21 / 33 / 32, no
  second tower, and the centre's own row collapses one tick
  late (51 → 20 at delay 10). The lean the peak grid saw was
  the slope of one crown, not a direction the ground keeps
  rewarding.
- **Count and duration part company on the lightest row:**
  (−0.50, delay 9) posts perfect **89/112** — the grid's
  second-best count, three behind the champion — on a longest
  episode of only 33, though its mean episode (17.8) nearly
  matches the champion's 18.4. The poison that keeps the
  count has a fine-grid address on this side of the crown too.

## Falsification Condition

The claim — *centred on (−0.75, delay 9), the diagonal is the
crown's shoulder: the champion diagonal reproduces its ledger
and exceeds the centre, no cell locks, and the tall cells form
a weight–delay ridge that ends at the crown* — is falsified if:

- any cell reports TRUE LOCK 7:5 — then the pair could hold
  after all, at an address every earlier grid skipped;
- the centre fails to reproduce perfect 86 / longest 51, or
  the champion diagonal fails to reproduce perfect 92 /
  longest 58 — then the instruments are not counting the same
  pair, and no comparison between them means anything;
- no neighbour exceeds the centre — then the diagonal was a
  tooth in its own right and "shoulder" is the wrong word;
- any cell in the −0.50 row outlasts the centre — then the
  lean extends past the diagonal and the ridge reading is
  this instrument's artefact.

All pinned by unit tests (9/9), including the full measured
grid — centre 51; champion diagonal 58; orthogonals 39 / 34 /
20 / 33 — so the record cannot quietly heal itself into a peak.

## Next Seed

Walk the compensation line itself out of this grid in both
directions, as `widow-pentad-ridge` walked the 5:3 line: the
on-line cells (−0.50, 8) 21 and (−1.00, 10) 58 are this grid's
ends — does the line fall off a cliff past the crown
(−1.25, delay 11 / −1.50, delay 12), and does ordinary ground
resume west of (−0.50, 8) at (−0.25, delay 7)? A ridge is a
claim about extent as much as height.

## Usage

```bash
cargo run -p widow-heptad-diagonal -- --headless
cargo test -p widow-heptad-diagonal
```

*Count everything. Forgive nothing that drifts — including a
tower whose height was borrowed from the crown beside it,
at a second ratio.*
