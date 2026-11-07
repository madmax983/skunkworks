# Widow Heptad Peak 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-11-07. Headless ledger below.

## Lineage

- **`neuro-sim`** — the two clocks: a Regular Spiking driver and a
  Fast Spiking follower, same equations, different recovery
  constants.
- **`widow-pentad-peak`** settled the near-**5:3** crown's shape:
  a tooth, not a plateau, leaning toward less weight arriving
  sooner. Its method — a fine grid centred on the champion, shape
  lines declared before the survey — is this instrument's method.
- **`widow-heptad`** took up the near-**7:5** pair along one row:
  the follower runs *early*, the medicine is inhibition, the firm
  hand (−1.5, delay 8) led the row at longest 45.
- **`widow-heptad-plane`** drew the whole (weight, delay) plane —
  63 cells — and crowned a new champion: **(−1.0, delay 10)**,
  92/112 perfect, longest episode **58**. Its named open seed was
  this instrument: *a fine grid around the champion (−1.0,
  delay 10) — is 58 a tooth or a plateau?*
- This instrument returns to the crown with a finer rule.

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
The fine survey is **3 weights × 3 delays = 9 cells** — weights
−1.25, −1.00, −0.75 (quarter steps); delays 9, 10, 11 (single
ticks) — each counted over **24,000 ticks** (first 1,500
discarded; 112 super-cycles), beside the uncoupled pair on the
same window (83/112 perfect, longest 17).

**Shape definitions, declared before the fine survey ran**
(champion longest C = 58): PLATEAU — at least 2 of the champion's
4 orthogonal neighbours reach 80% of C (longest ≥ 47); TOOTH —
no orthogonal neighbour reaches 47; ISOLATED TOOTH — a tooth
whose every neighbour, diagonal included, falls at or below
50% of C (longest ≤ 29); SHOULDER — anything else.

Five predictions were declared in the source before the fine
survey ran. All five held — and the grid still had a surprise
the predictions did not name, kept below.

## Novel Trait

**The 7:5 crown is a tooth, not a plateau — and its one tall
companion is the same diagonal the 5:3 crown leaned toward:
less weight, arriving sooner.**

Longest episode per fine cell (delays 9 / 10 / 11):

| weight | 9 | 10 | 11 |
|--------|---|---|----|
| −1.25 | 36 | 39 | 34 |
| −1.00 | 39 | **58** | 28 |
| −0.75 | **51** | 20 | 32 |

- **No lock at fine grain either (P1 HELD).** 9 cells, 112
  super-cycles each, zero TRUE LOCK 7:5. The 7:5's refusal to
  hold survives magnification, as the 5:3's did.
- **The instruments agree (P2 HELD).** The centre cell
  reproduces the plane exactly: 92/112 perfect (3 count slips,
  17 phase slips), longest 58, mean episode 18.4.
- **A tooth (P3 HELD).** The four orthogonal neighbours stand
  at **39, 39, 28, 20** — none reaches the plateau line of 47.
  One tick late at the champion's own dose costs 30
  super-cycles (58 → 28); a quarter-step lighter at the
  champion's own hour costs 38 (58 → 20). The crown is narrow
  in *both* axes.
- **The ground is elevated, the crown is not (P4 HELD).** The
  fine grid's mean longest episode is **37.4**, above the
  plane's inhibitory mean of 23.4 — the neighbourhood is
  genuinely good ground. Good ground, one tall tooth.
- **The centre stands (P5 HELD).** No fine cell outlasts the
  champion's 58.
- **The surprise, kept honestly:** the grid's second tower is
  the **diagonal (−0.75, delay 9) at 51** — above the plateau
  line, taller than every orthogonal neighbour, at the
  *lighter* dose and the *earlier* hour. This is the 5:3
  crown's lean, repeated at a second ratio with the opposite
  medicine: agreement does not fall off evenly around its best
  address, it leans toward less weight arriving sooner. That
  is why the shape verdict is **TOOTH (with skirts)**, not
  ISOLATED TOOTH — and it names the next survey: a fine grid
  centred on (−0.75, delay 9) itself, the diagonal move the
  5:3 row taught. Is the diagonal a second tooth, or the
  crown's shoulder seen from the wrong centre?
- **Count and duration part company at the corner, again:**
  (−0.75, delay 11) posts 90/112 perfect — within two of the
  champion's 92 — on a longest episode of only 32. The poison
  that keeps the count has a fine-grid address too.

## Falsification Condition

The claim — *at fine grain the plane's champion (−1.0, delay 10)
is a tooth: its orthogonal neighbours fall well short of its
height, no neighbour outlasts it, and no fine cell locks* — is
falsified if:

- any fine cell reports TRUE LOCK 7:5 — then the pair could
  hold after all, at an address the coarse plane skipped;
- at least two orthogonal neighbours reach longest 47 — then
  the crown was a plateau the coarse grid happened to sample
  at its highest point, and "tooth" is the wrong word;
- the centre cell fails to reproduce the plane's ledger
  (92 perfect, longest 58) — then the two instruments are not
  counting the same pair, and no comparison between them means
  anything;
- any fine cell outlasts 58 — then the champion was a grid
  artefact, and the true crown lives at an address this
  instrument was built to find.

All pinned by unit tests (7/7), including the full measured
grid — centre 58; orthogonals 39 / 39 / 28 / 20; the diagonal
51 — so the record cannot quietly heal itself into a plateau.

## Usage

```bash
cargo run -p widow-heptad-peak -- --headless
cargo test -p widow-heptad-peak
```

*Count everything. Forgive nothing that drifts — including a
champion whose shape nobody had measured, at a second ratio.*
