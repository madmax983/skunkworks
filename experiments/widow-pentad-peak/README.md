# Widow Pentad Peak 🕰️

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
  63 cells — and crowned a new champion: **(+1.5, delay 4)**,
  134/188 perfect, longest episode **55**. Its named open seed was
  this instrument: *the same plane at the fine grid around the
  champion (weights +1.25/+1.5/+1.75, delays 3/4/5) — is (1.5, 4)
  a peak or a plateau?*
- This instrument returns to the crown with a finer rule.

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
The fine survey is **3 weights × 3 delays = 9 cells** — weights
+1.25, +1.50, +1.75 (quarter steps); delays 3, 4, 5 (single
ticks) — each counted over **24,000 ticks** (first 1,500
discarded; 188 super-cycles), beside the uncoupled pair on the
same window (101/188 perfect, longest 27).

**Shape definitions, declared before the fine survey ran**
(champion longest C = 55): PLATEAU — at least 2 of the champion's
4 orthogonal neighbours reach 80% of C (longest ≥ 44); TOOTH —
no orthogonal neighbour reaches 44; ISOLATED TOOTH — a tooth
whose every neighbour, diagonal included, falls below 50% of C
(longest ≤ 27); SHOULDER — anything else.

Five predictions were declared in the source before the fine
survey ran. All five held — and the grid still had a surprise
the predictions did not name, kept below.

## Novel Trait

**The crown is a tooth, not a plateau — and its one tall
companion is a diagonal.**

Longest episode per fine cell (delays 3 / 4 / 5):

| weight | 3 | 4 | 5 |
|--------|---|---|---|
| +1.25 | **46** | 15 | 21 |
| +1.50 | 23 | **55** | 25 |
| +1.75 | 22 | 25 | 16 |

- **No lock at fine grain either (P1 HELD).** 9 cells, 188
  super-cycles each, zero TRUE LOCK 5:3. The 5:3's refusal to
  hold survives magnification.
- **The instruments agree (P2 HELD).** The centre cell
  reproduces the plane exactly: 134/188 perfect (12 count slips,
  42 phase slips), longest 55, mean episode 13.4.
- **A tooth (P3 HELD).** The four orthogonal neighbours stand at
  **15, 23, 25, 25** — none reaches the plateau line of 44, and
  the nearest dose at the champion's own hour (+1.25, delay 4)
  collapses to 15, the grid's second-lowest cell. A quarter-step
  of weight at the right hour costs 40 super-cycles of longest
  episode. The crown is narrow in *both* axes.
- **The ground is elevated, the crown is not (P4 HELD).** The
  fine grid's mean longest episode is **27.6**, above the plane's
  excitatory mean of 22.0 — the neighbourhood is genuinely good
  ground. Good ground, one tall tooth.
- **The centre stands (P5 HELD).** No fine cell outlasts the
  champion's 55.
- **The surprise, kept honestly:** the grid's second tower is
  the **north-west diagonal (+1.25, delay 3) at 46** — above the
  plateau line, taller than every orthogonal neighbour, at the
  *lighter* dose and the *earlier* hour. The crown's height is
  not radially symmetric: agreement at 5:3 does not fall off
  evenly around its best address, it leans toward less weight
  arriving sooner. That is why the shape verdict is **TOOTH
  (with skirts)**, not ISOLATED TOOTH — and it names the next
  survey: a fine grid centred on (+1.25, delay 3) itself. Is the
  diagonal a second tooth, or the crown's shoulder seen from
  the wrong centre?

## Falsification Condition

The claim — *at fine grain the plane's champion (+1.5, delay 4)
is a tooth: its orthogonal neighbours fall well short of its
height, no neighbour outlasts it, and no fine cell locks* — is
falsified if:

- any fine cell reports TRUE LOCK 5:3 — then the pair could
  hold after all, at an address the coarse plane skipped;
- at least two orthogonal neighbours reach longest 44 — then
  the crown was a plateau the coarse grid happened to sample
  at its highest point, and "tooth" is the wrong word;
- the centre cell fails to reproduce the plane's ledger
  (134 perfect, longest 55) — then the two instruments are not
  counting the same pair, and no comparison between them means
  anything;
- any fine cell outlasts 55 — then the champion was a grid
  artefact, and the true crown lives at an address this
  instrument was built to find.

All pinned by unit tests (7/7), including the full measured
grid — centre 55; orthogonals 15 / 23 / 25 / 25; the diagonal
46 — so the record cannot quietly heal itself into a plateau.

## Usage

```bash
cargo run -p widow-pentad-peak -- --headless
cargo test -p widow-pentad-peak
```

*Count everything. Forgive nothing that drifts — including a
champion whose shape nobody had measured.*
