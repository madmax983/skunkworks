# Listener Vdrift 🦇

**Lineage:** `experiments/vespertilio` → `experiments/listener-choir`
(multi-bat jamming) → `experiments/listener-flutter` (glint trains)
→ `experiments/listener-ripple` (the noisier cave) →
`experiments/listener-adaptive` (the fourth glint) →
`experiments/listener-paired` (the paired audit) →
`experiments/listener-jammed` (interleaved foreign glints, the
timing gate) → `experiments/listener-echoed` (the coincident wall,
the orphan tail) → `experiments/listener-adrift` (the wall at an
unknown address) → `experiments/listener-drifting` (the wall in
motion) → `experiments/listener-twinwalled` (two walls, both still)
→ `experiments/listener-twindrift` (two walls, both walking)
→ `experiments/listener-twincurve` (two walls, both bending)
→ `experiments/listener-census` (how many walls, before pricing any)
→ this planting.

**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-08. The predictions below were declared
before the shipped run and were not edited after it; one design
correction from the first draft is kept in the open below.

## The Seed This Planting Exists For

listener-twindrift closed with a named seed, kept standing through
twincurve and the census: *a wall whose drift changes sign
mid-flight (a V in the record).* Every Listener trajectory before
this one was a shape with no joint: still, walking at one pace, or
bending at one constant accel. A V turns. The novel trait is a
trajectory family with a joint in it — and the audit of where the
older families' failures live when the wall turns: in their
verdicts, or only in their naming.

## The Audit

A wall here is a two-step trajectory: a base delay and two steps
(s0, s1), one per glint gap — delay(0) = base, delay(1) =
base + s0, delay(2) = base + s0 + s1. The lineage's families are
shapes of (s0, s1): a LINEAR wall has s1 = s0; a CURVED wall has
s1 = s0 + accel, accel in {−1, 0, +1}. A V wall turns at the
joint: s1 = −s0. A book of recorded trains — 120 moths and 120
stones per regime, struck at every phase of their periods in
ripple's medium (moth 0.80 + 0.20·cos, period 12; stone
0.92 + 0.08·cos, period 17; per-contribution noise in [−σ, +σ]
drawn deterministically from the train's identity and channel;
arrivals under the 0.22 ear dropped) — is recorded behind a
single wall at ρ = 0.55 in three regimes:

- **shallow V, out and back** — (6, +1, −1): delays 6, 7, 6;
  echoes at 6, 11, 14.
- **wide V, down then up** — (7, −2, +2): delays 7, 5, 7;
  echoes at 7, 9, 15.
- **wide V, up then down** — (5, +2, −2): delays 5, 7, 5;
  echoes at 5, 11, 13.

Four judges read the same delivered record — amplitudes and
arrival offsets, nothing else:

- **Naive:** the first three arrivals against the 0.30 flutter
  line.
- **Drift:** listener-twindrift's family, alone — a joint search
  over LINEAR trajectories (base 3..=9, step −1/0/+1, both steps
  equal) and one strength on a 0.05 grid. Never told any wall
  turns.
- **Curve:** listener-twincurve's family, alone — CURVED
  trajectories (drift −1/0/+1, accel −1/0/+1) and one strength
  on the same grid. It can bend a wall by one tick of accel, and
  its first step is at most one tick.
- **Vdrift:** JOINTED trajectories — base 3..=9, each step free
  in {−2, −1, 0, +1, +2}, every per-glint delay inside 1..=12 —
  and one strength on the same grid. The linear and curved
  families are subsets of this one; the joint is the novel
  trait. Each candidate derives the own glints it implies,
  predicts the whole record, and is scored by the summed
  absolute difference over every offset either side mentions
  (sparse score with an abort bound that preserves exactness).

No V in this cave is a linear wall (that would need s0 = −s0) and
none is a curved wall (the shallow V needs accel −2; the wide Vs
open with a two-tick step, outside the curved family's drift
range) — asserted by test, not by prose.

## Predictions (declared before the shipped run)

- **P1:** the naive judge is wounded in every regime — at
  σ 0.00 its errors are at least 20 in each regime.
- **P2:** the drift search is a linear judge — at σ 0.00 it
  names the true trajectory on zero trains in every regime, and
  it errs at least 20 times in the shallow V and the wide
  up-then-down V.
- **P3:** the wide down-then-up V is the exception that prices
  the rule — at σ 0.00 the drift judge errs 0 times there
  despite naming the truth on zero trains, and its modal
  substitute is the still wall at the V's own base delay,
  (7, 0, 0): verdict-perfect, trajectory-blind.
- **P4:** the curve search is a one-tick-bend judge — at σ 0.00
  it errs at least 20 times in each regime and names the true
  trajectory on zero trains in every regime.
- **P5:** the vdrift search prices the jointed wall in a clean
  cave — at σ 0.00 its errors are 0 in every regime, its
  trajectory estimate is exact on every train, and its ρ error
  max is ≤ 0.011 in every regime.
- **P6:** pooled over the three regimes at σ 0.10, the vdrift
  judge's errors are below the naive judge's.
- **P7:** at σ 0.10 the vdrift judge still names the joint:
  its trajectory estimate is exact on at least 90% of trains
  pooled.
- **P8:** at σ 0.30, pooled, the vdrift judge's errors are still
  below the naive judge's pooled errors.

## What Came Back

All eight predictions **HELD**.

In the clean cave the jointed judge is exact in the strong
sense: errors 0 in every regime, trajectory exact 240/240 in
every regime, ρ error mean and max 0.000 — three returns are
enough to name a turn, when the search is allowed to hold one.

The predecessors fail, but they fail in two different places,
and the split is the survey. In the shallow V and the wide
up-then-down V, both predecessors' failures are verdict failures:
drift errs 140 and 140, curve 140 and 150, each naming the truth
on zero trains. Their modal substitutes are walking walls —
(8, −1, −1) on 190 trains in the shallow cave (both judges),
(5, −1, −1) on 240 (drift) and 127 (curve) in the up-then-down
cave — walls that steal one of the V's echoes by landing it on a
later own glint, self-consistently, and pay for the theft in the
derived glints. In the wide down-then-up V the drift judge's
failure is a naming failure only: errors **0**, on a modal
substitute of the still wall (7, 0, 0) on 205 trains — the V's
own base delay, held still. That substitute mispredicts two echo
offsets (it says 7/11/15, the cave said 7/9/15) and touches no
own glint, so every verdict it derives is right and every wall
it names is wrong. The curve judge does not find even that
shelter (errors 120, modal substitute (4, −1, −2) on 110): its
family has no still wall wide enough at that address, and its
least-wrong bend contaminates.

Under noise the joint's rent is ordinary. At σ 0.10 the vdrift
judge posts exactly the unwalled book's own noise floor in every
regime (21 errors, the clean-naive ledger of the same book) and
still names the trajectory on 720/720 pooled; at σ 0.20 it errs
58/61/57 with the joint named on 217/222/221; at σ 0.30 pooled
it errs 290 against naive's 372, the joint named on
162/183/167. Note where the down-then-up V stays kind longest at
depth: at σ 0.30 the drift judge errs 101 there — still
trajectory-blind (exact 0 at every σ), still sheltering behind
the average — against its own 116 and 121 in the other caves.

## Design Correction Kept From the Planting

One, found by the cave itself before anything shipped: the
first draft declared the drift judge wounded (errors ≥ 20) in
every regime. The draft's own ledger refused it — in the wide
down-then-up regime the drift judge erred 0 times, modal
substitute (7, 0, 0) on 205 trains. The regime was kept, the
declaration was split (P2 for the two caves where the failure
is a verdict failure, P3 for the one where it is a naming
failure only), and the predictions were re-declared against the
split before the shipped run; the shipped run reproduced the
draft's ledger line for line. A prediction that cannot survive
its own draft is not a prediction worth shipping unamended —
but the draft that cracked it is part of the record, not a
shame to be tidied away.

## The Law

Fifteenth strike: a judge one shape short of the record does
not always degrade — ask where its failure lives before pricing
it. Where a turning wall offers a walking substitute that can
steal an echo through an own glint, the failure is in the
verdicts (140 of 240 wrong, on a wall that was never there).
Where the turn is symmetric about a still delay, the straight
judge shelters behind the V's own average: verdict-perfect,
trajectory-blind, and wrong about the cave on every train it
judges correctly. The paired audit's question extends a fifth
time: before trusting a priced cave, ask how many walls the
judge was allowed to count, whether any could move, whether
any could bend, whether the count was struck or assumed — and
now whether any could *turn*, and if the judge could not, do
not ask only what it got wrong; ask whether what it got wrong
was the answer, or merely the wall.

**Falsification condition:** the vdrift claim — that a wall's
turn is recoverable from the delivered record alone by a search
whose steps are free — dies if, in a clean cave, its trajectory
estimate or verdict is wrong on any train, or if at σ 0.10 its
pooled trajectory-exact rate falls under 90%, or if a V regime
is built (same step family, same ear, strength clearing the
ear) whose records the linear judge prices with trajectory
exactness above zero — a straight judge naming a turning wall
exactly would kill the law's premise, not just its numbers.

## Next Seeds

- A V beside a second wall — the joint in a pair: does the
  pair search's rent land on the jointed wall first, as the
  census's rent landed on the count?
- Longer trains (four or five glints) — a wall with two joints,
  or one joint carried by more returns: how the naming's
  drowning σ moves with one more return per wall (the standing
  Listener seed, sharpened by this run's split).
- A four-line census form in a cave that sometimes holds four
  walls (listener-census's standing seed, still standing).

## Running It

```
cargo test -p listener-vdrift
cargo run -p listener-vdrift -- --headless
```

`--headless` is handled before anything else; there is no TUI.
The headless ledger prints, per regime and per σ ∈ {0.00, 0.10,
0.20, 0.30}: arrivals, naive / drift / curve / vdrift verdict
errors, each search judge's trajectory-exact count, the vdrift
judge's ρ error mean/max, and — at σ 0.00 — the trajectory each
predecessor judge most often names in place of the V; then the
eight declared predictions, judged on identical evidence.
