# Listener Equalturns 🦇

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
`experiments/listener-twinjoint` (two turns, gated by the quieter
wall) → this planting.

**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-11. The predictions below were declared
before the shipped run and were not edited after it.

## The Seed This Planting Exists For

listener-twinjoint closed with a named seed: *equal strengths on
two turns (both walls at one ρ) — the gate charged to the quieter
wall should vanish: does the pair's naming then gate on width
alone, and does either wall's exact count still equal the pair's?*

Twinjoint's headline was an identity, not a trend. Its caves held
a first wall at ρ 0.55 and a second at ρ 0.40, and at σ 0.10 the
quieter wall's exact count **was** the pair-exact count — 530 =
530 pooled, and regime by regime severally (160 = 160, 176 = 176,
194 = 194). Whenever the quieter wall was named, the pair was
named; the pair was only as nameable as its fainter member. That
is a gate with a name on it. This planting removes the name:
same caves, same walls, same judges — both walls at one strength,
ρ = 0.45, the on-grid strength nearest below the midpoint of
0.55 and 0.40 (the judge names strengths on a 0.05 grid, and
0.475 would sit half a step from every nameable strength,
counterfeiting a ρ error in a clean cave). Total loudness falls
slightly (0.95 to 0.90), biasing every pooled comparison against
this planting; only the *inequality* is gone at full strength.
A design correction kept from the planting: the first draft set
ρ = 0.475 exactly, and its own clean-cave test caught the
counterfeit — ρ error max 0.025 at σ 0.00, pure grid quantisation
wearing the costume of a mispriced wall. The strength was moved
to the grid before any shipped run; no prediction changed.
If the gate was the inequality, the identity must break here:
each wall should sometimes be named alone. And what is left to
gate the pair, if anything, is width — the shallow turn one tick
from straight, from mirror, and from counterfeit; the wide turn
standing alone.

## The Audit

A wall is a two-step trajectory: a base delay and two steps
(s0, s1), one per glint gap — delay(0) = base, delay(1) =
base + s0, delay(2) = base + s0 + s1. A V wall turns at the
joint (s1 = −s0, s0 ≠ 0); a SHALLOW V steps one tick, a WIDE V
steps two. A book of recorded trains — 120 moths and 120 stones
per regime, struck at every phase of their periods in ripple's
medium (moth 0.80 + 0.20·cos, period 12; stone 0.92 + 0.08·cos,
period 17; per-contribution noise in [−σ, +σ] drawn
deterministically from the train's identity and the contribution's
channel; arrivals under the 0.22 ear dropped) — is recorded in
twinjoint's three caves, unchanged stone for stone, except that
**both walls stand at ρ = 0.45**:

- **shallow + shallow, opposed** — A (6, +1, −1): delays 6, 7, 6,
  echoes at 6, 11, 14; B (9, −1, +1): delays 9, 8, 9, echoes at
  9, 12, 17.
- **wide + shallow** — A (7, −2, +2): delays 7, 5, 7, echoes at
  7, 9, 15; B (8, +1, −1): delays 8, 9, 8, echoes at 8, 13, 16 —
  its first echo rides the third own glint.
- **shallow + wide** — A (5, +1, −1): delays 5, 6, 5, echoes at
  5, 10, 13; B (9, −2, +2): delays 9, 7, 9, echoes at 9, 11, 17.

No wall in these caves is a curved wall (a shallow V needs accel
±2; a wide V opens with a two-tick step), so the true pair is not
in the curved pair family's space in any cave — asserted by test.

Three judges read the same delivered record — amplitudes and
arrival offsets, nothing else:

- **Naive:** the first three arrivals against the 0.30 flutter
  line.
- **Twincurve:** listener-twincurve's search, carried whole —
  an exhaustive joint search over an unordered pair of CURVED
  trajectories and two strengths on a 0.05 grid. It can bend
  either wall by one tick. Neither of its walls may turn.
- **Twinjoint:** twinjoint's judge itself, carried whole — an
  unordered pair search over JOINTED trajectories (base 3..=9,
  each step free in {−2, −1, 0, +1, +2}, every per-glint delay
  inside 1..=12) with the strengths fitted beside each other:
  every unordered pair is first scored on a coarse strength
  lattice (0.2 steps), and the 64 best-scoring pairs are
  re-fitted on the full 0.05 grid. The staging is path-free, as
  twinjoint left it — a test asserts this staged search returns
  the same verdict, pair, and strengths as the exhaustive
  full-grid unordered search on sampled trains, and that the
  sparse score equals the dense whole-record score. Per-wall
  naming is set membership — the pair is unordered, so a wall is
  named when it stands in the estimated pair. This planting adds
  one bookkeeping line to the audit and no new machinery: the
  **solo counts** — trains where wall A is named but B is not,
  and where B is named but A is not. The gate identity, in these
  terms, is the claim that one solo count is zero.

## Predictions (declared before the shipped run)

- **P1:** the walled caves wound the naive judge in every
  regime — at σ 0.00 its errors are at least 20 in each regime.
- **P2:** the twincurve search is a curved judge in a turning
  cave — at σ 0.00 it names the true pair on zero trains in
  every regime (neither V is in its family).
- **P3:** the echo-swap shelter does not need unequal strengths
  — in the shallow + shallow, opposed cave at σ 0.00, the
  twincurve judge still errs fewer than 20 times while naming
  the true pair on zero trains (twinjoint found its modal
  substitutes there swap third echoes: {6, 11, 17} and
  {9, 12, 14} for the truth's {6, 11, 14} and {9, 12, 17} — an
  offset mechanism, not a loudness mechanism, so equalising
  should not close it).
- **P4:** the twinjoint search prices the pair in a clean cave
  — at σ 0.00 its errors are 0 in every regime, its pair
  estimate is exact on every train, and its ρ error max is
  ≤ 0.011 in every regime.
- **P5:** pooled over the three regimes at σ 0.10, the
  twinjoint judge's errors are below the naive judge's.
- **P6:** THE GATE IDENTITY BREAKS — pooled at σ 0.10, neither
  wall's exact count equals the pair-exact count, and each wall
  is named alone on at least one train (A-only ≥ 1 and
  B-only ≥ 1). With no quieter wall, no wall's naming can be
  the pair's naming in disguise.
- **P7:** the rent now gates on width alone — pooled over the
  three regimes at σ 0.30, the shallow walls' exact rate is
  below the wide walls' exact rate.
- **P8:** at σ 0.30, pooled, the twinjoint judge's errors are
  still below the naive judge's pooled errors.
- **P9:** raising the quieter wall lifts the pair — pooled
  pair exact at σ 0.10 exceeds twinjoint's unequal-strength
  530/720, even though the louder wall fell from 0.55 to 0.45 and total loudness fell with it.
  The gate was charged to the quieter wall; paying that wall
  should buy more pair-namings than the louder wall's discount
  costs.

## What Came Back

Eight of the nine predictions **HELD**; P9 **CRACKED**, and
the crack redraws the gate.

In the clean caves the twinjoint judge is exact in the strong
sense, at the equal strength exactly as at twinjoint's unequal
pair: errors 0 in every regime, pair exact 240/240 in every
regime, ρ error mean and max 0.000. The naive judge errs
exactly 120 times in every regime, every stone judged a moth.

**P3 held, with the same substitute on the same trains.** In
the shallow + shallow, opposed cave the twincurve judge errs
**0** at σ 0.00 while naming the true pair on zero trains, and
its modal substitute is twinjoint's own: ((6, +1, +2),
(9, −1, −2)) on all 240 trains — the two bends that swap third
echoes ({6, 11, 14}/{9, 12, 17} returned as {6, 11, 17}/
{9, 12, 14}). The swap is an offset mechanism; equalising the
strengths did not touch it, and the shelter stands at equal
loudness exactly where it stood at unequal. (In the wide caves
the curved judge errs 150 and 128 clean, as before in kind.)

**P6 held: the gate identity is gone.** Pooled at σ 0.10 the
pair-exact count is **462**, the first wall's exact count is
**483**, the second wall's is **494** — neither equals the
pair, and the solo ledger says why: the first wall is named
alone on **21** trains, the second alone on **32**. Under
twinjoint's unequal strengths the quieter wall's count *was*
the pair count, regime by regime; here both walls walk out of
the pair in both directions. There is no quieter wall, and
there is no identity. The gate this planting was built to
remove is removed.

**P9 cracked: removing the gate did not lift the pair — it
moved the weight.** Pooled pair exact at σ 0.10 is 462/720,
*below* twinjoint's unequal 530/720, even though the quieter
wall rose (0.40 → 0.45) and total loudness fell only slightly.
Severally, the story splits by cave. The mixed caves rose a
little, as the seed expected: wide + shallow 181 against
twinjoint's 176, shallow + wide 200 against 194. But the
shallow + shallow, opposed cave **collapsed**: pair exact **81**
against twinjoint's 160, its per-wall counts 85 and 89, solos
4 and 8 — at σ 0.10 that cave's walls are each named barely
more often than the pair, and at σ 0.20 its pair exact is 39
against the mixed caves' 55 and 105. The opposed cave is the
one cave where the two walls are the same *shape* at mirror
addresses; give them the same strength as well and they become
twins — and a twin is the hardest wall to name, because every
record it explains is explained as well by its sibling wearing
its name. The gate was not only charged to the quieter wall.
In the twin cave it is charged to *sameness*.

**P7 held, and cleanly now: with strength equal, width is the
only bill left, and it still arrives.** Pooled exact rates at
σ 0.30: shallow **171/960 = 0.178** against wide
**169/480 = 0.352** — the wide turn named at twice the shallow
turn's rate when nothing but width separates them. The
separation is present at σ 0.10 too (shallow 566/960 = 0.590
against wide 411/480 = 0.856) and at σ 0.20 (314/960 = 0.327
against 255/480 = 0.531). Twinjoint's width invoice survives
the removal of its strength confound entire.

Pooled verdicts: σ 0.10 twinjoint **73** against naive 362 and
twincurve 252 — the same 73 twinjoint posted at unequal
strengths, reached by a different road; σ 0.30 twinjoint 308
against naive 381 (pair exact 72/720 pooled: 12/17/43 by
regime). At depth the solo counts grow teeth in the mixed
caves — at σ 0.20 and 0.30 the wide first wall of the wide +
shallow cave is named alone on **62** trains at both depths,
the widest solo in the ledger: a wide wall keeps being found
long after its shallow partner is lost.

12/12 tests, the equal-strength totals pinned as found.


## The Law

Eighteenth strike: a gate can be an inequality, or it can be
a resemblance — and removing the first reveals the second.
Equalise two turning walls and twinjoint's identity dies on
schedule: at σ 0.10 neither wall's exact count is the pair's
(483 and 494 against 462), each wall is named alone (21 and
32 solos), and no wall gates the pair by being quieter,
because none is. But the pair is not freed: pooled naming
*falls* (462 against the unequal 530), and the fall lives in
one cave — the shallow-opposed cave, where the walls share a
shape at mirror addresses, collapses to 81 pair-exact against
twinjoint's 160 while the mixed caves rise slightly. Two walls
at one strength and one width are twins, and the record cannot
always say which twin spoke. What still charges, at every
depth, is width alone: shallow 0.178 against wide 0.352 at
σ 0.30 pooled, with strength held equal by construction. The
paired audit's question extends an eighth time: before
trusting a priced cave, ask how many walls the judge was
allowed to count, whether any could move, bend, or turn, which
of them gates the naming, whether the losing is charged to
width or to loudness — and now whether two walls are hard to
name not because one is faint, but because they are alike.


**Falsification condition:** the equalturns claim — that
twinjoint's gate identity was an artefact of unequal strengths,
not of turning pairs as such — dies if, at σ 0.10 pooled, either
wall's exact count still equals the pair-exact count (a gate
surviving equal loudness was never a loudness gate), or if the
twinjoint judge errs or misnames in a clean cave here, or if the
width separation at σ 0.30 pooled vanishes or reverses once
strength can no longer explain it (shallow rate ≥ wide rate) —
that last would kill not this planting's premise but the width
reading of twinjoint's invoice itself.

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
  a different one? (still standing, now sharpened: the swap
  stands at equal strength too, if P3 holds.)
- Equal strengths at the extremes — both walls quiet (0.40) and
  both walls loud (0.55): if the gate identity stays broken at
  both levels, it was the inequality; if it reassembles at one
  level, the gate has a loudness floor as well as a slope.
- The solo ledger at depth: at σ 0.20 and 0.30, do the two
  walls' solo counts stay comparable, or does position (the
  earlier wall against the later) start charging its own gate
  once loudness stops charging one?

## Running It

```
cargo test -p listener-equalturns
cargo run -p listener-equalturns -- --headless
```

`--headless` is handled before anything else; there is no TUI.
The headless ledger prints, per regime and per σ ∈ {0.00, 0.10,
0.20, 0.30}: arrivals, naive / twincurve / twinjoint verdict
errors, twincurve and twinjoint pair-exact counts, the
twinjoint judge's per-wall exact counts and solo counts (A-only,
B-only), its per-width exact counts, and its ρ error mean/max,
and — at σ 0.00 — the pair the twincurve judge most often names
in place of the truth; then the nine declared predictions,
judged on identical evidence.
