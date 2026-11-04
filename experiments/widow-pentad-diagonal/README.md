# Widow Pentad Diagonal 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-11-03. Headless ledger below.

## Lineage

- **`neuro-sim`** — the two clocks: a Regular Spiking driver and a
  Fast Spiking follower, same equations, different recovery
  constants.
- **`widow-pentad`** took up the near-**5:3** pair (FS I=5.3): the
  follower runs *late* by 1.51 ticks per super-cycle, the medicine
  is excitation, and along a single row the best hand was
  (+1.0, delay 8), longest episode 47 super-cycles, with no lock.
- **`widow-pentad-plane`** drew the whole (weight, delay) plane —
  63 cells — and crowned a champion: **(+1.5, delay 4)**,
  134/188 perfect, longest episode **55**.
- **`widow-pentad-peak`** measured the fine grid around the
  champion and found a **tooth, not a plateau** — and one surprise:
  the grid's second tower was its north-west diagonal,
  **(+1.25, delay 3) at 46**, above the plateau line, at the
  lighter dose and the earlier hour. Its README named this
  instrument outright: *a fine grid centred on (+1.25, delay 3)
  itself — is the diagonal a second tooth, or the crown's
  shoulder seen from the wrong centre?*
- This instrument recentres the fine grid on the diagonal and
  counts again.

## Concept

The pair is unchanged, and so is the counting. Two neurons joined
by exactly one synapse:

- **The driver** — Regular Spiking, constant current I=12. Solo
  measured period: **39.60 ticks** (568 spikes in the window).
- **The follower** — Fast Spiking, constant current I=5.3. Solo
  measured period: **24.06 ticks** (935 spikes). Five follower
  beats span **120.31** against three driver cycles' **118.80** —
  near 5:3, the follower **late by 1.51 ticks per super-cycle**.

A **super-cycle** is three consecutive driver cycles. It is
**perfect** when it holds exactly five follower beats AND each
beat's phase lies within ±0.06 of that beat's median phase for
the regime (the two-gate rule, carried over unchanged). An
**episode** is a maximal run of consecutive perfect super-cycles.
The survey is **3 weights × 3 delays = 9 cells** — weights
+1.00, +1.25, +1.50 (quarter steps); delays 2, 3, 4 (single
ticks) — each counted over **24,000 ticks** (first 1,500
discarded; 188 super-cycles), beside the uncoupled pair on the
same window (101/188 perfect, longest 27). The champion
(+1.5, delay 4) stands in this grid's south-east corner, a
diagonal neighbour of the centre.

**Shape definitions, declared before the survey ran** (centre
longest C = 46): plateau line 37 (80% of C); isolation line 23
(50% of C); SHOULDER OF THE CROWN — some neighbour exceeds the
centre; PLATEAU — at least 2 orthogonal neighbours reach 37 and
none exceeds the centre; ISOLATED TOOTH — no orthogonal reaches
37, every neighbour at or below 23, none exceeds the centre;
TOOTH (with skirts) — no orthogonal reaches 37, none exceeds the
centre, but some neighbour stands above 23; SHOULDER — anything
else.

Five predictions were declared in the source before the survey
ran. All five held.

## Novel Trait

**The diagonal was the crown's shoulder — and the tall cells
trace a ridge, not a peak.**

Longest episode per cell (delays 2 / 3 / 4):

| weight | 2 | 3 | 4 |
|--------|---|---|---|
| +1.00 | **40** | 27 | 23 |
| +1.25 | **42** | **46** | 15 |
| +1.50 | 19 | 23 | **55** |

- **No lock, again (P1 HELD).** 9 cells, 188 super-cycles each,
  zero TRUE LOCK 5:3.
- **The instruments agree (P2 HELD).** The centre reproduces the
  peak grid's diagonal exactly in longest episode: **46**
  (perfect 105/188, 12 count slips, 71 phase slips).
- **Dethroned by its own corner (P3 HELD).** The south-east
  diagonal — the champion (+1.5, delay 4) itself — reproduces its
  ledger exactly (perfect **134/188**, longest **55**, mean
  episode 13.4) and exceeds the centre. Shape verdict:
  **SHOULDER OF THE CROWN**. The peak grid's "second tower" was
  never a second tooth; it was a point on the crown's slope,
  seen from a grid that put it near a corner.
- **The measured cross holds (P4 HELD).** The two orthogonals the
  peak grid already counted keep their numbers: (+1.5, delay 3)
  at **23** and (+1.25, delay 4) at **15**, both at or below the
  isolation line.
- **Good ground (P5 HELD).** The grid's mean longest episode is
  **32.2**, well above the plane's excitatory mean of 22.0.
- **The surprise, kept honestly:** the centre is not low on every
  side. Its north orthogonal (+1.25, delay 2) stands at **42**
  and its north-west diagonal (+1.00, delay 2) at **40** — both
  above the plateau line of 37. Read the four tall cells in
  order — (+1.00, 2) 40, (+1.25, 2) 42, (+1.25, 3) 46,
  (+1.50, 4) 55 — and they trace a **diagonal ridge**: lighter
  doses arriving sooner balance heavier doses arriving later,
  and the ridge climbs through the centre to the crown. The
  crown is a tooth in the peak's grid and a ridge's end in this
  one; both records are pinned, and the ridge is the shape that
  explains both.

## Falsification Condition

The claim — *centred on (+1.25, delay 3), the diagonal is the
crown's shoulder: the champion corner reproduces its ledger and
exceeds the centre, no cell locks, and the tall cells form a
weight–delay ridge* — is falsified if:

- any cell reports TRUE LOCK 5:3 — then the pair could hold
  after all, at an address every earlier grid skipped;
- the centre fails to reproduce 46, or the champion corner fails
  to reproduce perfect 134 / longest 55 — then the instruments
  are not counting the same pair, and no comparison between them
  means anything;
- no neighbour exceeds the centre — then the diagonal was a
  tooth in its own right and "shoulder" is the wrong word;
- the north cells (+1.00, delay 2) and (+1.25, delay 2) fall to
  the isolation line or below — then there is no ridge, only a
  slope, and the ridge reading is this instrument's artefact.

All pinned by unit tests (8/8), including the full measured
grid — centre 46; champion corner 55; north ridge 40 / 42 —
so the record cannot quietly heal itself into a peak.

## Usage

```bash
cargo run -p widow-pentad-diagonal -- --headless
cargo test -p widow-pentad-diagonal
```

*Count everything. Forgive nothing that drifts — including a
tower whose height was borrowed from the crown beside it.*
