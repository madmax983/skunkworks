# Listener Midceiling 🦇

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
wall) → `experiments/listener-equalturns` (two turns at one strength,
the gate was an inequality — and behind it, a resemblance) →
`experiments/listener-equalextremes` (the extremes struck: the gate
dead at 0.40, alive at 0.55 on the later wall — a ceiling, not a
floor) → this planting.

**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-12. The predictions below were declared
before the shipped run and were not edited after it.

## The Seed This Planting Exists For

listener-equalextremes closed with a named seed: *the ceiling
located — both walls at ρ 0.50, between equalturns' gateless 0.45
and equalextremes' gated 0.55: where does the second wall's
identity reassemble (B-only 32 pooled at 0.45, 0 at 0.55), is the
ceiling a step or a slope, and does the twin cave cross it first?*

The row so far, all of it at σ 0.10 pooled over twinjoint's three
caves with both walls at one shared strength: at ρ 0.40 the gate
identity is dead (pair 370, solos 48 and 50); at 0.45 it is dead
(pair 462, solos 21 and 32); at 0.55 it is alive on the second
wall (pair 553, second wall 553, B-only 0 — in every regime
severally). Somewhere in (0.45, 0.55] the later wall's naming
becomes the pair's naming. This planting strikes the bracket's
midpoint, ρ = 0.50 (total loudness 1.00), and re-strikes the loud
end, ρ = 0.55, in the same run on the same book as its control —
so the midpoint is read against the gated end with no drift of
instrument between them. Nothing else moves: same caves, same
book, same judges, same solo ledger.

It also asks the seed's second question in the only form a single
level can answer it: regime by regime at the midpoint, has any
single cave crossed while the pooled ledger has not? A ceiling
that arrives cave by cave is a slope; one that arrives everywhere
at once is a step. And the seed's guess about the twin cave is
put on the record as a prediction (P11): the shallow + shallow,
opposed cave — the dearest cave at every level yet struck — is
expected to cross *last*, not first.

## The Staging Repair (read this before the ledger)

The midpoint caught the instrument before it caught the gate.
Twinjoint's judge is a staged search: every unordered pair is
scored on a coarse strength lattice and the best 64 are refitted
on the full 0.05 grid. Carried whole into this planting, its own
staged-vs-exhaustive test failed at ρ 0.50 — and the failure
decomposed into two honest parts:

- **The midpoint was not on the lattice.** The parent's lattice
  steps 0.2 (0.0, 0.2, 0.4, 0.6, 0.8, 1.0): ρ 0.50 is not a point
  on it. At 0.40 the truth's strengths sat exactly on a lattice
  point; at 0.50 the staging ranked the true pair by neighbours
  of its true strengths. The lattice in this planting steps 0.1
  (121 points), and the refit pool is widened from 64 to 128.
- **The tie broke two ways.** With the finer lattice in place,
  one sampled train in the opposed cave at σ 0.20 still divided
  the searches: the truth ((6, +1, −1), (9, −1, +1)) and its
  echo-swap substitute ((6, +1, +2), (9, −1, −2)) score *exactly*
  equal best (0.4140, at the same fitted strengths (0.45, 0.55)).
  The exhaustive search walks pairs in trajectory order; the
  staged refit walked its pool in lattice-rank order; a strict
  less-than keeps the first of a tie, so the two searches named
  different walls from one record. The refit in this planting
  walks its pool in the exhaustive search's own enumeration
  order: the staging may prune the pool, but it may not
  reshuffle it. The inherited test — staged returns the same
  verdict, pair, and strengths as exhaustive on sampled trains —
  passes unamended.

The repair is recorded here because it is also a measurement:
at the midpoint, in the twin cave, the truth and its substitute
are not merely close — on at least one noisy train they are
indistinguishable to the last digit the score keeps. Whatever
the ceiling is, it is priced by an instrument whose two best
answers can tie.

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
twinjoint's three caves, unchanged stone for stone, at **each of
the two levels in turn**:

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

Three judges read the same delivered record at each level —
amplitudes and arrival offsets, nothing else:

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
  every unordered pair is first scored on the strength lattice
  (0.1 steps in this planting, widened from the parent's 0.2 —
  see the repair above), and the 128 best-scoring pairs are
  re-fitted on the full 0.05 grid, walked in the exhaustive
  search's own enumeration order. A test asserts this staged
  search returns the same verdict, pair, and strengths as the
  exhaustive full-grid unordered search on sampled trains, and
  that the sparse score equals the dense whole-record score.
  Per-wall naming is set membership — the pair is unordered, so
  a wall is named when it stands in the estimated pair — and
  the audit keeps equalturns' solo counts: trains where wall A
  is named but B is not, and where B is named but A is not. The
  gate identity, in these terms, is the claim that one solo
  count is zero and one wall's count equals the pair's.

## Predictions (declared before the shipped run)

- **P1:** the walled caves wound the naive judge in every
  regime at BOTH levels — at σ 0.00 its errors are at least 20
  in each regime at ρ 0.50 and at ρ 0.55.
- **P2:** the twincurve search is a curved judge in a turning
  cave — at σ 0.00 it names the true pair on zero trains in
  every regime, at both levels.
- **P3:** the echo-swap shelter stands at both levels — in
  the shallow + shallow, opposed cave at σ 0.00, the twincurve
  judge still errs fewer than 20 times while naming the true
  pair on zero trains, at ρ 0.50 and at ρ 0.55 (the swap is an
  offset mechanism; equalextremes found it standing at the ends,
  and a mechanism that cares about neither inequality nor level
  should stand at the midpoint too).
- **P4:** the twinjoint search prices the pair in a clean cave
  at both levels — at σ 0.00 its errors are 0 in every regime,
  its pair estimate is exact on every train, and its ρ error
  max is ≤ 0.011 in every regime, at ρ 0.50 and at ρ 0.55.
- **P5:** pooled over the three regimes at σ 0.10, the
  twinjoint judge's errors are below the naive judge's, at
  both levels.
- **P6:** THE GATE IS NOT YET BACK AT THE MIDPOINT — pooled
  at σ 0.10 at MID, the second wall's exact count does NOT
  equal the pair-exact count, and B is named alone on at least
  one train (B-only ≥ 1): the ceiling lies above 0.50. If
  B-only reads 0 at the midpoint, the ceiling sits at or below
  it and this prediction dies there.
- **P7:** loudness still pays the pair — pooled pair exact at
  σ 0.10 at LOUD exceeds pooled pair exact at MID.
- **P8:** the twin cave stays the dearest at both levels — at
  σ 0.10, at EACH level the shallow + shallow, opposed regime's
  pair exact is below BOTH mixed regimes' pair exact (sameness
  charges at the midpoint as at the ends).
- **P9:** the rent still gates on width alone at both
  levels — pooled over the three regimes at σ 0.30, at EACH
  level the shallow walls' exact rate is below the wide walls'
  exact rate.
- **P10:** at σ 0.30, pooled, the twinjoint judge's errors are
  still below the naive judge's pooled errors, at both levels.
- **P11:** THE CEILING IS CROSSED CAVE BY CAVE, MIXED CAVES
  FIRST — at MID, σ 0.10, B-only is 0 in at least one MIXED
  regime while the shallow + shallow, opposed regime's B-only
  is still ≥ 1: a slope, not a step, and the twin cave crosses
  last, not first.

## What Came Back

Ten of the eleven predictions **HELD**; P11 **CRACKED**,
and the crack is the answer to the seed's second question.

In the clean caves the twinjoint judge is exact in the strong
sense at both levels: errors 0 in every regime, pair exact
240/240 in every regime, ρ error mean and max 0.000. The naive
judge errs exactly 120 times in every regime at both levels,
every stone judged a moth.

**P3 held, with the same substitute on the same trains, at the
midpoint as at the ends.** In the shallow + shallow, opposed cave
the twincurve judge errs **0** at σ 0.00 at ρ 0.50 and at
ρ 0.55, naming the true pair on zero trains, and its modal
substitute is the row's own — ((6, +1, +2), (9, −1, −2)) on all
240 trains at both levels. The echo-swap is an offset mechanism;
it does not care about inequality, level, or midpoint.

**P6 held: the gate is not back at the midpoint.** Pooled at
σ 0.10 at MID the pair-exact count is **538**, the walls' exact
counts are **551** and **546**, and the solos are **13** and
**8** — the second wall is still named alone on 8 trains, so its
naming is not yet the pair's naming. The ceiling lies in
(0.50, 0.55]. The loud control, re-struck in the same run under
the repaired staging, reassembles the identity entire: pair
**563**, second wall **563**, B-only **0** — and severally, B-only
0 in every regime (opposed 101 = 101 = 101; wide + shallow pair
222 with B 222 and all 8 solos on A; shallow + wide a perfect
240/240). One honest note on that control: the repaired
instrument reads the loud end slightly richer than
equalextremes' unrepaired staging did (pair 563 against 553,
opposed 101 against 91) — the finer lattice and wider pool name
ten more pairs at the loud end. The identity's verdict is
unchanged; the counts beside it are this instrument's, and
cross-run comparisons should be made in verdicts first.

**P11 cracked: no cave has crossed at the midpoint.** The
seed's slope guess said some mixed cave would reach B-only 0
while the twin cave lagged. Instead B-only at MID, σ 0.10, reads
**2** in the opposed cave, **3** in wide + shallow, **3** in
shallow + wide — flat, and nowhere zero. Between 0.45 and 0.50
the pooled B-only count fell from 32 to 8, spread almost evenly
across the caves; somewhere in the last eighth, (0.50, 0.55], it
falls from 8 to 0 everywhere at once, as far as these three caves
can tell. The solo count approaches like a slope; the identity
itself — a count of exactly zero — still arrives like a step,
and at the midpoint it has not arrived anywhere. The twin cave
did not cross first; nothing crossed.

**P7 and P8 held, and together they frame the crack.** Loudness
pays the pair monotonically across the row's equal plantings:
pooled pair exact at σ 0.10 reads **370** at ρ 0.40, **462** at
0.45 (equalturns), **538** at 0.50, **563** at 0.55 — the
midpoint sits on the line, nearer the loud end than the middle
distance suggests. And the twin cave stays the dearest at both
levels struck here (P8): at MID the opposed cave's pair exact is
**99** against the mixed caves' 207 and 232; at LOUD it is
**101** against 222 and a perfect 240. The twin cave barely
moves between the midpoint and the loud end (99 → 101) while
its mixed siblings climb — resemblance is still the bill
loudness cannot pay, and it is paid in the cave where the
echo-swap tie was found.

**P9 held at both levels:** σ 0.30 pooled shallow against wide
exact rates **0.224 against 0.408** at MID and **0.297 against
0.494** at LOUD — width charges at the midpoint as at the ends.

At depth the gate's shadow falls where it fell at the loud end:
at MID, σ 0.20, the wide first wall of the wide + shallow cave
is named alone on **71** trains while its shallow partner is
named alone on 9 (equalextremes read 72 against 3 at ρ 0.55) —
the asymmetry that precedes the gate is already full-grown at
the midpoint, one level down, while the gate itself is not.

Pooled verdicts: σ 0.10 twinjoint **75** (mid) and **78** (loud)
against naive 362 and 360; σ 0.30 twinjoint 305 and 292 against
naive 378 and 372 (pair exact pooled at σ 0.30: 107/720 mid —
26/21/60 by regime; 169/720 loud — 42/34/93).

12/12 tests — including the inherited staged-vs-exhaustive
test, which failed at the midpoint until the staging repair
above, and passes unamended against the repaired staging.


## The Law

Twentieth strike: the ceiling is an address in the last
eighth, and it is approached evenly. Equalextremes found the
later wall's gate identity dead at ρ 0.40 and 0.45 and alive at
0.55; this planting struck the bracket's midpoint and found it
dead there too — pooled pair 538, B 546, B-only 8 at ρ 0.50 —
and dead in every cave severally (B-only 2 / 3 / 3), while the
loud control in the same run stood gated entire (563 = 563,
B-only 0, severally). So the ceiling sits in (0.50, 0.55], the
solo count slopes toward it evenly across caves (32 at 0.45, 8
at 0.50, spread flat), and the identity — which is a solo count
of exactly zero, nothing softer — has not begun to arrive in
any single cave at the midpoint: a slope in the counts, a step
in the identity, compressed into the last eighth of the bracket.
And the midpoint priced the instrument as well as the gate:
twinjoint's staged search, carried whole, could not see ρ 0.50
on its 0.2 lattice and broke an exact tie (truth and echo-swap
substitute at 0.4140, same strengths) two ways against its own
exhaustive search — repaired here with a 0.1 lattice, a wider
pool, and one enumeration order for both searches. The paired
audit's question extends a tenth time: before trusting a priced
cave, ask how many walls the judge was allowed to count, whether
any could move, bend, or turn, which of them gates the naming,
whether the losing is charged to width, to loudness, or to
likeness, at what level a gate reassembles — and now whether
the judge's own staging can even see the level it was asked to
price, and which way it breaks a tie when the record gives it
two walls for one price.


**Falsification condition:** the midceiling claim — that the
gate identity reassembles only above the midpoint, arriving
cave by cave with the twin cave last — dies if, at σ 0.10 pooled
at MID, B-only is 0 and B's exact count equals the pair's (the
gate already whole at 0.50: a ceiling at or below the midpoint),
or if at MID every regime's B-only is still ≥ 1 while the pooled
count is also un-crossed in a later finer strike (a step above
0.50, not a slope), or if the twinjoint judge errs or misnames
in a clean cave at either level, or if the twin cave stops being
the dearest cave at a level.

## Next Seeds

- The ceiling at finer grain: both walls at ρ 0.525 — the
  last eighth halved. B-only pooled reads 8 at 0.50 and 0 at
  0.55: at 0.525 does any cave reach zero (a slope in the last
  eighth after all), or does the whole row step together
  between 0.525 and 0.55?
- The gate's address: at LOUD, swap the walls' addresses —
  the same two shapes with the early and late bases exchanged:
  does the gate follow the later address (position) or the
  wall that held it here (shape)? Twinjoint's gate,
  equalturns' absence, equalextremes' ceiling, and this
  planting's midpoint all sat on — or spared — the second
  wall; no planting has yet moved a wall and watched the gate
  choose. (still standing.)
- The tie census: at the midpoint, in the opposed cave, the
  truth and its echo-swap substitute tied exactly on a sampled
  train at σ 0.20 (0.4140 at the same fitted strengths). How
  many trains tie exactly at each level — and does the tie
  rate peak at the midpoint, where the two walls' strengths
  are nearest the swap's own fitted strengths (0.45, 0.55)?
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
  a different one? (still standing.)
- The solo ledger at depth: at σ 0.20 and 0.30, do the two
  walls' solo counts stay comparable, or does position (the
  earlier wall against the later) start charging its own gate
  once loudness stops charging one? (still standing.)


## Running It

```
cargo test -p listener-midceiling
cargo run -p listener-midceiling -- --headless
```

`--headless` is handled before anything else; there is no TUI.
The headless ledger prints, per level (mid ρ 0.50, loud
ρ 0.55), per regime and per σ ∈ {0.00, 0.10, 0.20, 0.30}:
arrivals, naive / twincurve / twinjoint verdict errors,
twincurve and twinjoint pair-exact counts, the twinjoint
judge's per-wall exact counts and solo counts (A-only, B-only),
its per-width exact counts, and its ρ error mean/max, and — at
σ 0.00 — the pair the twincurve judge most often names in place
of the truth; then the eleven declared predictions, judged on
identical evidence.
