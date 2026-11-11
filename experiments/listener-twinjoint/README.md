# Listener Twinjoint 🦇

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
→ `experiments/listener-vdrift` (one wall that turns) →
`experiments/listener-vpair` (a turn beside a straight wall) →
this planting.

**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-10. The predictions below were declared
before the shipped run and were not edited after it.

## The Seed This Planting Exists For

listener-vpair closed with a named seed: *two joints in one cave —
a V beside a second turning wall: does the rent still separate at
depth, and which turn is lost first?* Every Listener pair cave
before this one held at most one turning wall. This cave holds
two, and asks the rent question inside the turning family itself:
when both walls turn, is the bill charged by *width* — a shallow
one-tick turn standing one tick from a straight wall and one tick
from its own mirror, a wide two-tick turn standing alone?

## The Audit

A wall here is a two-step trajectory: a base delay and two steps
(s0, s1), one per glint gap — delay(0) = base, delay(1) =
base + s0, delay(2) = base + s0 + s1. A V wall turns at the
joint (s1 = −s0, s0 ≠ 0); a SHALLOW V steps one tick, a WIDE V
steps two. A LINEAR wall walks straight (s1 = s0, |s0| ≤ 1); a
CURVED wall bends by at most one tick of accel (s1 = s0 + accel,
accel in {−1, 0, +1}, |s0| ≤ 1). A book of recorded trains — 120
moths and 120 stones per regime, struck at every phase of their
periods in ripple's medium (moth 0.80 + 0.20·cos, period 12;
stone 0.92 + 0.08·cos, period 17; per-contribution noise in
[−σ, +σ] drawn deterministically from the train's identity and
the contribution's channel; arrivals under the 0.22 ear
dropped) — is recorded in three caves, each holding two turning
walls, the first at **ρ = 0.55**, the second at **ρ = 0.40**:

- **shallow + shallow, opposed** — A (6, +1, −1): delays 6, 7, 6,
  echoes at 6, 11, 14; B (9, −1, +1): delays 9, 8, 9, echoes at
  9, 12, 17.
- **wide + shallow** — A (7, −2, +2): delays 7, 5, 7, echoes at
  7, 9, 15; B (8, +1, −1): delays 8, 9, 8, echoes at 8, 13, 16 —
  its first echo rides the third own glint.
- **shallow + wide** — A (5, +1, −1): delays 5, 6, 5, echoes at
  5, 10, 13; B (9, −2, +2): delays 9, 7, 9, echoes at 9, 11, 17.

The width comparison is balanced across strengths by design:
among the mixed caves each width stands once at ρ 0.55 and once
at ρ 0.40, and the opposed cave holds the shallow form at both
strengths — so a width bill cannot be a strength bill in
disguise, nor the other way round. No wall in these caves is a
curved wall (a shallow V needs accel ±2; a wide V opens with a
two-tick step), so the true pair is not in the curved pair
family's space in any cave — asserted by test.

Three judges read the same delivered record — amplitudes and
arrival offsets, nothing else:

- **Naive:** the first three arrivals against the 0.30 flutter
  line.
- **Twincurve:** listener-twincurve's search, carried whole —
  an exhaustive joint search over an unordered pair of CURVED
  trajectories and two strengths on a 0.05 grid. It can bend
  either wall by one tick. Neither of its walls may turn.
- **Twinjoint:** an unordered pair search over JOINTED
  trajectories (base 3..=9, each step free in {−2, −1, 0, +1,
  +2}, every per-glint delay inside 1..=12) with the strengths
  fitted beside each other: every unordered pair is first
  scored on a coarse strength lattice (0.2 steps), and the 64
  best-scoring pairs are re-fitted on the full 0.05 grid. The
  staging is path-free on purpose: earlier drafts ranked by
  coordinate descent, as listener-vpair's judge does, and the
  garden's own staged-vs-exhaustive test caught a sampled train
  where the exhaustive winner ranked ~175th by one-round
  descent — with two turning walls the strengths interact, and
  a descent's path strands good pairs at bad scores. A test asserts this staged search returns
  the same verdict, pair, and strengths as the exhaustive
  full-grid unordered search on sampled trains. Each candidate
  derives the own glints it implies, predicts the whole record,
  and is scored by the summed absolute difference over every
  offset either side mentions (sparse score with an abort bound
  that preserves exactness; a test asserts the sparse score
  equals the dense whole-record score). Per-wall naming is set
  membership — the pair is unordered, so a wall is named when
  it stands in the estimated pair.

## Predictions (declared before the shipped run)

- **P1:** the walled caves wound the naive judge in every
  regime — at σ 0.00 its errors are at least 20 in each regime.
- **P2:** the twincurve search is a curved judge in a turning
  cave — at σ 0.00 it names the true pair on zero trains in
  every regime (neither V is in its family).
- **P3:** no cave shelters the twincurve judge — at σ 0.00 it
  errs at least 20 times in every regime.
- **P4:** the twinjoint search prices the twin-turning pair in
  a clean cave — at σ 0.00 its errors are 0 in every regime,
  its pair estimate is exact on every train, and its ρ error
  max is ≤ 0.011 in every regime.
- **P5:** pooled over the three regimes at σ 0.10, the
  twinjoint judge's errors are below the naive judge's.
- **P6:** at σ 0.10 the twinjoint judge still names the pair:
  its pair estimate is exact on at least 90% of trains pooled.
- **P7:** the rent separates by width at depth — pooled over
  the three regimes at σ 0.30, the shallow walls' exact rate is
  below the wide walls' exact rate (a one-tick turn stands one
  tick from a straight wall and from its mirror; a two-tick
  turn stands alone).
- **P8:** at σ 0.30, pooled, the twinjoint judge's errors are
  still below the naive judge's pooled errors.

## What Came Back

Six of the eight predictions **HELD**; P3 and P6 **CRACKED**,
and the cracks are the survey.

In the clean caves the twinjoint judge is exact in the strong
sense: errors 0 in every regime, pair exact 240/240 in every
regime, ρ error mean and max 0.000 — two turns are six returns,
and six returns name both walls when the search is allowed to
hold two turns at once.

The naive judge is conscripted as always: at σ 0.00 it errs
exactly 120 times in every regime, every stone judged a moth.

**P3 cracked, and the crack reopens a closed door.** vpair's
law said there is no shelter in a pair cave — a judge with two
walls to spend always finds a substitute that steals an echo
and fails in the verdicts. In the wide caves that holds here
(twincurve errs 130 and 97 clean). But in the shallow +
shallow, opposed cave the twincurve judge errs **0** at
σ 0.00 while naming the true pair on zero trains — verdict-
perfect and pair-blind at once, vdrift's shelter standing
again in a pair cave. Its modal substitute, named on all 240
trains, is ((6, +1, +2), (9, −1, −2)) — and the mechanism is
exact, not approximate: the true walls' echo sets are
{6, 11, 14} and {9, 12, 17}; the substitutes' are {6, 11, 17}
and {9, 12, 14}. The two bends **swap third echoes** — each
curved wall's third return lands where the other true wall's
third return fell — so the delivered record's offsets are the
truth's offsets, and only the strengths are counterfeit. The
shelter was never closed by the pair; it was closed by the
*widths*. Give both turns a two-tick step and no bend can
reach the swap; keep both turns shallow and opposed, and the
swap is there. The curved judge keeps sheltering in that cave
under mild noise, too: at σ 0.10 it errs 18 there, under the
unwalled book's own floor of 21, still naming nothing.

**P6 cracked: two turns are dearer to name than a turn beside
a straight wall.** At σ 0.10 the twinjoint judge's pooled
pair-exact is 530/720 (73.6%), against vpair's 683/720 on its
own caves. The per-wall invoice shows who collects: at
σ 0.10 the first wall (ρ 0.55) is named on 650 trains pooled
and the second (ρ 0.40) on **530 — exactly the pair-exact
count, in every regime severally as well as pooled** (160 =
160, 176 = 176, 194 = 194). Whenever the quieter wall is
named, the pair is named; the pair's naming is gated by its
quieter member, whole and entire, at the surface. At
σ 0.20 the gate loosens only slightly (B named 224, pair
211); at σ 0.30, 112 against 84.

**P7 held: inside the gate, the bill is itemised by width.**
Pooled exact rates, shallow against wide: σ 0.10 — 0.788
against 0.883; σ 0.20 — 0.418 against 0.606; σ 0.30 — 0.225
against 0.388. The separation is present at every depth
surveyed, not only in the deep water, and it widens as the
water deepens. A one-tick turn stands one tick from a straight
wall, one tick from its mirror, and one tick from the bend
that counterfeits it; a two-tick turn stands alone, and the
ledger charges accordingly. (The design's balance matters
here: each width stands at each strength among the mixed
caves, so this is a width bill, not a strength bill wearing
one — and severally, the wide B wall of the third cave is
named more at σ 0.10 than the shallow B wall of the second,
194 against 176, at the same ρ 0.40.)

Pooled verdicts: σ 0.10 twinjoint 73 against naive 360 and
twincurve 255; σ 0.30 twinjoint 310 against naive 371 and
twincurve 338 — the twinjoint judge keeps the verdict lead at
depth even as its naming drowns (pair exact 84/720 at
σ 0.30).

## Design Correction Kept From the Planting

One, found by the garden's own test before anything shipped:
the staged twinjoint search was first built on listener-
vpair's pattern — rank every pair by coordinate descent,
refit the best few on the full grid. The staged-vs-exhaustive
test refused it on a sampled train in the wide + shallow cave
(σ 0.20): the exhaustive winner, a substitute pair keeping the
true wide wall beside a faint fake at ρ 0.10, ranked ~175th of
14,365 under one-round descent (score 0.586 against its grid
score 0.023), and a two-round descent refit still stranded it
at a local strength minimum (0.074). With two turning walls
the strengths interact strongly enough that a descent's *path*
is evidence of nothing. The staging was rebuilt path-free —
every pair scored on a coarse strength lattice (0.2 steps),
the best 64 re-fitted on the full grid — and the test then
passed on the same samples. vpair's descent staging stands in
its own cave; it does not survive promotion to two turns.


## The Law

Seventeenth strike: in a twin-turning cave, ask by what the
rent is itemised — and check the shallow corner for a shelter
you were told was closed. The pair's naming is gated by its
quieter wall: at σ 0.10 the ρ 0.40 wall's exact count *is* the
pair-exact count, regime by regime, so a pair is only as
nameable as its fainter member is. Inside that gate the bill
is itemised by width at every depth — the shallow turn, one
tick from straight, from mirror, and from counterfeit, is
lost before the wide turn at σ 0.10 (0.788 against 0.883),
σ 0.20 (0.418 against 0.606), and σ 0.30 (0.225 against
0.388) alike. And the curved judge's shelter, which the pair
cave was said to abolish, reopens exactly where both turns
are shallow and opposed: two bends swap third echoes, the
offset record is the truth's own, and the judge is verdict-
perfect on a pair it cannot name. The paired audit's question
extends a seventh time: before trusting a priced cave, ask
how many walls the judge was allowed to count, whether any
could move, bend, or turn — and now, when two walls turn
together, which of them gates the naming, whether the losing
is charged to width or to loudness, and whether a bending
judge's clean verdicts there are a reading or a swap.


**Falsification condition:** the twinjoint claim — that two
turning walls standing together are recoverable from the
delivered record alone by an unordered search whose walls'
steps are free — dies if, in a clean cave, its pair estimate
or verdict is wrong on any train, or if at σ 0.10 its pooled
pair-exact rate falls under 90%, or if a cave is built (same
families, same ear, strengths clearing the ear) whose records
the curved pair judge prices with pair exactness above zero —
a bending judge naming a twin-turning pair exactly would kill
the law's premise, not just its numbers.

## Next Seeds

- Longer trains (four or five glints) — a wall with two joints,
  or one joint carried by more returns: how the naming's
  drowning σ moves with one more return per wall (the standing
  Listener seed, still standing).
- A four-line census form in a cave that sometimes holds four
  walls (listener-census's standing seed, still standing).
- The reopened shelter, priced in noise: the shallow-opposed
  cave where two bends swap third echoes — at what σ does the
  swap stop paying the curved judge's verdicts, and does the
  twinjoint judge's naming in that cave drown at the same σ or
  a different one?
- Equal strengths on two turns (both walls at one ρ): the
  gate this run charged to the quieter wall should vanish —
  does the pair's naming then gate on width alone, and does
  either wall's exact count still equal the pair's?

## Running It

```
cargo test -p listener-twinjoint
cargo run -p listener-twinjoint -- --headless
```

`--headless` is handled before anything else; there is no TUI.
The headless ledger prints, per regime and per σ ∈ {0.00, 0.10,
0.20, 0.30}: arrivals, naive / twincurve / twinjoint verdict
errors, twincurve and twinjoint pair-exact counts, the
twinjoint judge's per-wall exact counts split by position and
by width, and its ρ error mean/max, and — at σ 0.00 — the pair
the twincurve judge most often names in place of the truth;
then the eight declared predictions, judged on identical
evidence.
