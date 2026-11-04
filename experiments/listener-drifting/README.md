# Listener Drifting 🦇

**Lineage:** `experiments/vespertilio` → `experiments/listener-choir`
(multi-bat jamming) → `experiments/listener-flutter` (glint trains)
→ `experiments/listener-ripple` (the noisier cave) →
`experiments/listener-adaptive` (the fourth glint) →
`experiments/listener-paired` (the paired audit) →
`experiments/listener-jammed` (interleaved foreign glints, the
timing gate) → `experiments/listener-echoed` (the coincident wall,
the orphan tail) → `experiments/listener-adrift` (the wall at an
unknown address) → this planting.
**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-04. The predictions below were declared
before the first run and were not edited after it.

## The Seed This Planting Exists For

listener-adrift closed with a named seed: *a wall whose delay
drifts within a train's flight — each glint's echo at a different
delay, as from a moving reflector or a sloped wall — priced by a
search that must give the wall a velocity, not an address.* In
adrift the wall stood still at one unknown address, and a joint
search over (delay, ρ) priced it exactly in a clean cave. This
planting sets the wall in motion.

## The Audit

A book of recorded trains — 120 moths and 120 stones per regime,
struck at every phase of their periods in ripple's medium (moth
0.80 + 0.20·cos, period 12; stone 0.92 + 0.08·cos, period 17;
per-contribution noise in [−σ, +σ] drawn deterministically from
the train's identity; arrivals under the 0.22 ear dropped) — is
recorded behind three walls, all at **ρ = 0.55**, all starting at
delay **4** (listener-echoed's own address) and drifting by
**s ticks per glint**, s ∈ {−1, 0, +1}. The record window runs to
offset 20 so every wall's full return is delivered:

- **s = 0:** delays 4, 4, 4 — echoes at 4, 8, 12. The still wall;
  its naive ledger must reproduce listener-echoed's exactly.
- **s = +1:** delays 4, 5, 6 — echoes at 4, 9, 14. The first echo
  coincides with the second own glint; the rest walk away.
- **s = −1:** delays 4, 3, 2 — echoes at 4, 7, 10. The first echo
  coincides; the rest walk back toward the strike.

Three judges read the *same* delivered record — amplitudes and
arrival offsets, nothing else:

- **Naive:** the first three arrivals, spread (max−min)/mean vs
  the **0.30** line — flutter's judge.
- **Static:** listener-adrift's search verbatim. It searches one
  fixed delay in 1..=12 ticks and ρ in [0.00, 0.95] at 0.01 steps,
  deriving the own glints each candidate implies and scoring the
  candidate against the whole delivered record. It can give the
  wall an address but not a velocity. It is never told the wall
  moves.
- **Drift:** a joint search over base delay in 2..=6 ticks, drift
  in {−1, 0, +1} ticks per glint, and ρ on the same grid. Each
  candidate derives its own glints (subtracting, in strike order,
  any earlier glint's echo that lands exactly on a later own
  offset), predicts the whole record, and is scored by the summed
  absolute difference between prediction and delivery over every
  offset either mentions. The cheapest candidate wins; its derived
  own glints are judged on the same 0.30 line. The judge is never
  told the wall's true base, drift, or ρ — those are the book's
  ground truth, kept for scoring its estimates only.

**Falsification condition:** if the static judge recovers the
drifting books at σ 0.00, a wall's velocity was never
load-bearing — an address was always enough — and this planting's
premise is false. If the drift judge fails to recover the clean
book at σ 0.00 at any drift, a whole-record joint search does not
price a moving wall, and the channel is a story, not a
measurement.

**Novel trait:** every previous Listener wall stood still —
corruption could be priced once its single address was found.
This is the first planting where the wall's address *changes
inside one train*: the orphan is not merely at an unknown offset,
it is at a different offset for every glint, and the judge that
survives is the one that prices a trajectory (base, drift, ρ
together) instead of a place.

## Declared Predictions (before the first run)

- **P1:** The moving wall wounds the naive judge at every drift:
  at σ 0.00 its errors are at least 20 at each of s = −1, 0, +1.
- **P2:** The static search is a zero-velocity judge: at σ 0.00
  it recovers the book exactly at s = 0 (errors 0) and errs at
  least 20 times at each of s = −1 and s = +1.
- **P3:** The drift search prices all three unknowns in a clean
  cave: at σ 0.00 its errors are 0 at every drift, its
  (base, drift) estimate is exact on every train, and its ρ
  error max is ≤ 0.011.
- **P4:** Pooled over the three drifts at σ 0.10, the drift
  judge's errors are below the naive judge's.
- **P5:** At σ 0.10 the drift judge still names the wall's
  motion: its (base, drift) estimate is exact on at least 90% of
  trains pooled.
- **P6:** At σ 0.30, pooled over the three drifts, the drift
  judge's errors are still below the naive judge's pooled
  errors — the noise floor does not fully swallow the search.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

240 trains judged per regime, 12 regimes (3 drifts × 4 noise
levels). 13/13 tests. Sanity first: the unwalled control
reproduces the lineage's clean book line for line — naive
errors **0** at σ 0.00, **21** at σ 0.10, **62** at σ 0.20,
**98** at σ 0.30 — at every drift, because the clean book does
not depend on the wall.

- **σ 0.00:** naive **120** at drift −1 (wrong 0, fooled 120 —
  every stone, and no moth refused: the backward-walking echoes
  reach the naive judge's first three arrivals as a rising
  staircase, and a staircase only ever reads as swing), **140**
  at drift 0 (listener-echoed's ledger exactly, wrong 20,
  fooled 120), **140** at drift +1 (wrong 20, fooled 120).
  Static **140 at both drifting walls** — the full naive
  catastrophe, posted by a judge that searches twelve addresses
  and ninety-six strengths — and **0** at drift 0. On the
  backward wall the static judge is *worse than naive* (140 vs
  120): it unwinds a still wall that is not there and
  manufactures 20 wrong refusals the naive judge never made.
  Drift **0 errors at every drift**, (base, drift) exact
  **240/240** at each, ρ error mean and max **0.000**.
- **σ 0.10:** drift pooled **124** vs naive pooled **386**;
  (base, drift) exact **720/720** pooled. By drift the drift
  judge errs **36 / 52 / 36** — at drift 0 its ledger is the
  static judge's exactly (52), as it must be: the winning
  candidate stands still. ρ error mean 0.071 at the drifting
  walls against 0.115 at the still wall — adrift's coincident
  tax again: where an echo rides on a summed sample, strength
  is the noisy part.
- **σ 0.20:** drift **79 / 90 / 79** by drift; (base, drift)
  exact 239 / 235 / 239 of 240. ρ error max reaches **0.550** —
  some trains' strength estimates collapse to the grid's floor —
  while the motion is still named nearly every time.
- **σ 0.30:** drift pooled **301** vs naive pooled **360**,
  against a pooled clean floor of 294 — the velocity search is
  nearly on the noise floor even here. (Base, drift) exact
  falls to 230 / 210 / 230 of 240: at this σ some records have
  lost an echo under the ear, and a record with a missing
  arrival can wear a wrong trajectory while keeping a nearly
  right verdict.

**Verdicts:** every declared line HELD. P1 (120 / 140 / 140,
all ≥ 20). P2 (static 0 at drift 0; 140 at each drifting wall —
a zero-velocity judge in full). P3 (drift 0 everywhere in a
clean cave, all three unknowns exact). P4 (124 < 386). P5
(720/720, not merely 90%). P6 (301 < 360).

**The symmetry the book kept:** drifts −1 and +1 return
*identical* static and drift ledgers at every σ (drift errors
36 / 79 / 97 in both directions; static 140 / 131 / 121 / 123
in both). The two moving walls are mirror records — delays
{4, 3, 2} against {4, 5, 6} — and a judge that reads the whole
record cannot tell a wall walking toward the strike from one
walking away, because there is nothing in the evidence to tell:
the corruption is the same shape read in either direction.
Only the naive judge separates them, and only at σ 0.00 (120
vs 140), because it reads arrival *order* — the one thing the
mirror flips.

**The law this row adds:** a moving wall is priced by its
trajectory, and the trajectory is carried in timing, not
loudness. Noise eats the strength first (ρ error max 0.550 at
σ 0.20, estimates collapsing to the floor) while the motion —
read off *where* arrivals land, which noise cannot move —
stays named: exact on 720/720 trains at σ 0.10 and 670/720 at
σ 0.30. And the still-wall judge's failure mode is worth
keeping beside adrift's: a judge built for a fixed address
does not degrade gracefully when the wall moves; it posts the
naive ledger in full, and where it unwinds hardest it invents
refusals of its own. Before trusting a priced wall, ask not
only where it stands, but whether it is standing still.

**Next seed:** two walls at once, either of them possibly in
motion (adrift's remaining seed, inherited): does the joint
search's whole-record score still separate a pair of
trajectories, or do two moving walls price each other? Or a
wall that accelerates — drift itself drifting within the
flight, priced by a search that must give the wall a curvature;
or listener-jammed's remaining open door — the gate whose
timing is learned, not given — now in a room whose wall will
not sit still for the lesson.

## Tests

13 unit tests: the clean book clears/stays under the line at
every phase; the unwalled book is error-free for the naive judge;
the still wall reproduces listener-echoed's naive ledger exactly
(140 errors, 120 fooled); both drifting walls' arrival geometry
(offsets and summed amplitudes, to 1e-12); the static search
recovers only the still wall at σ 0.00; the drift search recovers
every drift at σ 0.00, on whole books and on individual trains
(verdict, base, drift, and ρ); the naive judge is wounded at
every drift; the drift deconvolution unwinds a hand-built
coincident record exactly; the book is paired and deterministic;
the noise is zero at σ 0 and bounded by σ elsewhere.

Run them with `cargo test -p listener-drifting`; run the audit
with `cargo run -p listener-drifting -- --headless`.
