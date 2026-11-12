# Listener Equalextremes 🦇

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
the gate was an inequality — and behind it, a resemblance) → this
planting.

**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-12. The predictions below were declared
before the shipped run and were not edited after it.

## The Seed This Planting Exists For

listener-equalturns closed with a named seed: *equal strengths at
the extremes — both walls quiet (0.40) and both walls loud (0.55):
if the gate identity stays broken at both levels it was the
inequality; if it reassembles at one level, the gate has a loudness
floor as well as a slope.*

Equalturns struck the middle. Its caves held twinjoint's two
turning walls at one shared strength, ρ = 0.45, and twinjoint's
headline identity — the quieter wall's exact count *equals* the
pair-exact count (530 = 530 at σ 0.10 pooled, regime by regime) —
died on schedule: pair 462, walls 483 and 494, solos 21 and 32.
But a single middle reading cannot acquit the levels. A gate
identity broken at 0.45 might reassemble at an end: where both
walls are faint, the record may be too thin to separate either
wall, so the pair is only ever named whole or not at all — the
identity returning not as a gate on the quieter wall but as a
floor under both. Or where both are loud, saturation may do the
same work. This planting strikes both ends of twinjoint's own
unequal pair, in full: QUIET, both walls at ρ = 0.40, twinjoint's
quieter wall's own strength (total loudness 0.80, against
twinjoint's 0.95 and equalturns' 0.90); and LOUD, both walls at
ρ = 0.55, twinjoint's louder wall's own strength (total 1.10).
Nothing else moves: same caves, same book, same judges, same
solo ledger equalturns added.

It also re-prices equalturns' second finding at both ends.
Equalturns' pooled naming *fell* when the inequality was removed
(462 against twinjoint's 530), and the fall lived in one cave:
the shallow + shallow, opposed cave — the one cave whose walls
share a shape at mirror addresses — collapsed to 81 pair-exact
against the mixed caves' 181 and 200. The reading was sameness:
two walls at one strength and one width are twins, and the record
cannot always say which twin spoke. If that reading is a law and
not a middle-level accident, the twin cave should stay the dearest
cave at 0.40 and at 0.55 alike — sameness should charge at every
loudness, not only at the middle one.

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
  every unordered pair is first scored on a coarse strength
  lattice (0.2 steps), and the 64 best-scoring pairs are
  re-fitted on the full 0.05 grid. The staging is path-free, as
  twinjoint left it — a test asserts this staged search returns
  the same verdict, pair, and strengths as the exhaustive
  full-grid unordered search on sampled trains, and that the
  sparse score equals the dense whole-record score. Per-wall
  naming is set membership — the pair is unordered, so a wall is
  named when it stands in the estimated pair — and the audit
  keeps equalturns' solo counts: trains where wall A is named
  but B is not, and where B is named but A is not. The gate
  identity, in these terms, is the claim that one solo count is
  zero and one wall's count equals the pair's.

## Predictions (declared before the shipped run)

- **P1:** the walled caves wound the naive judge in every
  regime at BOTH levels — at σ 0.00 its errors are at least 20
  in each regime at ρ 0.40 and at ρ 0.55.
- **P2:** the twincurve search is a curved judge in a turning
  cave — at σ 0.00 it names the true pair on zero trains in
  every regime, at both levels.
- **P3:** the echo-swap shelter stands at both extremes — in
  the shallow + shallow, opposed cave at σ 0.00, the twincurve
  judge still errs fewer than 20 times while naming the true
  pair on zero trains, at ρ 0.40 and at ρ 0.55 (the swap is an
  offset mechanism; equalturns found it standing at equal
  loudness in the middle, and a mechanism that cares about
  neither inequality nor level should stand at the ends too).
- **P4:** the twinjoint search prices the pair in a clean cave
  at both levels — at σ 0.00 its errors are 0 in every regime,
  its pair estimate is exact on every train, and its ρ error
  max is ≤ 0.011 in every regime, at ρ 0.40 and at ρ 0.55.
- **P5:** pooled over the three regimes at σ 0.10, the
  twinjoint judge's errors are below the naive judge's, at
  both levels.
- **P6:** THE GATE IDENTITY STAYS BROKEN AT BOTH EXTREMES —
  pooled at σ 0.10, at EACH level neither wall's exact count
  equals the pair-exact count, and each wall is named alone on
  at least one train (A-only ≥ 1 and B-only ≥ 1). A level where
  an identity reassembles — one wall's count equal to the
  pair's, or a solo count of zero — is a loudness floor, and
  this prediction dies there.
- **P7:** loudness still pays the pair — pooled pair exact at
  σ 0.10 at LOUD exceeds pooled pair exact at QUIET.
- **P8:** the twin cave stays the dearest at both levels — at
  σ 0.10, at EACH level the shallow + shallow, opposed regime's
  pair exact is below BOTH mixed regimes' pair exact (sameness
  charges at every loudness, not only at the middle one).
- **P9:** the rent still gates on width alone at both
  extremes — pooled over the three regimes at σ 0.30, at EACH
  level the shallow walls' exact rate is below the wide walls'
  exact rate.
- **P10:** at σ 0.30, pooled, the twinjoint judge's errors are
  still below the naive judge's pooled errors, at both levels.

## What Came Back

Nine of the ten predictions **HELD**; P6 **CRACKED**, and
the crack is the planting's whole reason to exist.

In the clean caves the twinjoint judge is exact in the strong
sense at both extremes exactly as in the middle: errors 0 in
every regime at both levels, pair exact 240/240 in every regime,
ρ error mean and max 0.000. The naive judge errs exactly 120
times in every regime at both levels, every stone judged a moth.

**P3 held, with the same substitute on the same trains, at both
ends.** In the shallow + shallow, opposed cave the twincurve
judge errs **0** at σ 0.00 at ρ 0.40 and at ρ 0.55, naming the
true pair on zero trains, and its modal substitute is
twinjoint's and equalturns' own — ((6, +1, +2), (9, −1, −2)) on
all 240 trains at both levels. The echo-swap is an offset
mechanism; it does not care about inequality, and it does not
care about level either.

**P6 cracked at the loud end — the identity reassembles, and it
reassembles on the second wall.** At QUIET the seed's expectation
holds entire: pooled at σ 0.10 the pair-exact count is **370**,
the walls' exact counts are **418** and **420**, and the solos
are **48** and **50** — no gate anywhere, both walls walking out
of the pair in both directions, at the level where the record is
thinnest. At LOUD the ledger turns: pair exact **553**, first
wall **561**, second wall **553** — the second wall's exact count
*is* the pair-exact count, pooled, and its solo count is **0**.
Severally it is stronger still: at σ 0.10 the second wall is
named alone on **zero trains in every regime** (opposed 91 = 91
= 91, both walls equal to the pair; wide + shallow pair 222,
second wall 222, first wall 230 with all 8 of the level's solos;
shallow + wide a perfect 240/240, every count equal). Whenever
the second wall was named at ρ 0.55, the pair was named. The
gate equalturns killed at 0.45 is alive at 0.55 — not a floor
under both walls (the quiet end is gateless), but a ceiling
above them: somewhere between 0.45 and 0.55 the later wall's
naming becomes the pair's naming. And it is the *later* wall
that gates at every loudness where a gate has ever been seen in
this row: twinjoint's gate sat on its second wall too (the
quieter one, at base 9/8/9 against the first wall's 6/7/5) —
what this planting adds is that at the loud end the gate
survives the removal of the inequality that was supposed to
explain it, and follows the position, not the faintness.

**P7 and P8 held, and together they bound the crack.** Loudness
pays the pair monotonically across the row's equal plantings:
pooled pair exact at σ 0.10 reads **370** at total loudness
0.80, **462** at 0.90 (equalturns), **553** at 1.10 — with
twinjoint's unequal 530 at 0.95 sitting between the middle and
the loud end, as a monotone reading would want. And the twin
cave stays the dearest at both ends (P8): at QUIET the opposed
cave's pair exact is **75** against the mixed caves' 137 and
158; at LOUD it is **91** against 222 and a perfect 240 — the
sameness charge does not shrink as the walls get louder; if
anything the gap widens, the mixed caves rising with loudness
while the twin cave barely moves (75 → 81 at the middle → 91).
Two walls at one shape are hard to name at every loudness, and
loudness buys back everything except resemblance.

**P9 held at both ends:** with strength equal by construction
at each level, width is still a bill — σ 0.30 pooled shallow
against wide exact rates **0.145 against 0.267** at QUIET and
**0.294 against 0.492** at LOUD, the separation present at both
extremes as it was in the middle (0.178 against 0.352).

Pooled verdicts: σ 0.10 twinjoint **79** (quiet) and **78**
(loud) against naive 364 and 360 — the two extremes reach
nearly the same verdict count by opposite roads, the quiet end
losing trains for thinness, the loud end for resemblance; σ
0.30 twinjoint 319 and 292 against naive 381 and 372 (pair
exact pooled 44/720 quiet: 9/11/24 by regime; 169/720 loud:
42/34/93). At depth the solos grow asymmetrically at the loud
end: at σ 0.20 the wide first wall of the wide + shallow cave
is named alone on **72** trains while its shallow partner is
named alone on 3 — the gate's shadow, one level down.

12/12 tests, the extreme-level totals pinned as found.


## The Law

Nineteenth strike: a gate can be an inequality, a resemblance
— or a level. Equalturns removed the inequality at the middle
strength and the quieter-wall identity died; this planting
struck both ends of twinjoint's pair and found the identity
dead at the quiet end (pair 370, solos 48 and 50 at ρ 0.40) and
alive at the loud end (pair 553, second wall 553, second-wall
solos 0 — in every regime severally — at ρ 0.55). The gate has
a ceiling, not a floor: between ρ 0.45 and ρ 0.55 the later
wall's naming becomes the pair's naming, and it is the later
wall — by address, not by faintness — that has gated every
pair in this row, twinjoint's included. Loudness otherwise
pays honestly and monotonically (pair exact 370 / 462 / 553
across total loudness 0.80 / 0.90 / 1.10), width charges at
every level (shallow against wide 0.145/0.267 quiet,
0.294/0.492 loud at σ 0.30), the echo-swap shelter ignores
loudness entire — and resemblance is the one bill loudness
cannot pay: the twin cave reads 75 / 81 / 91 across the three
equal levels while its mixed siblings climb past 200. The
paired audit's question extends a ninth time: before trusting
a priced cave, ask how many walls the judge was allowed to
count, whether any could move, bend, or turn, which of them
gates the naming, whether the losing is charged to width, to
loudness, or to likeness — and now at what level a gate that
was not there in the middle reassembles at the end, and on
which wall it sits when it does.


**Falsification condition:** the equalextremes claim — that
twinjoint's gate identity was an artefact of unequal strengths
at every loudness, with no floor at either end — dies if, at
σ 0.10 pooled at either level, either wall's exact count equals
the pair-exact count or a solo count is zero (an identity
reassembling at a level was a level gate all along), or if the
twinjoint judge errs or misnames in a clean cave at either
level, or if the twin cave stops being the dearest cave at a
level (sameness charging only in the middle would make
equalturns' collapse a level accident, not a law).

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
  a different one? (still standing.)
- The solo ledger at depth: at σ 0.20 and 0.30, do the two
  walls' solo counts stay comparable, or does position (the
  earlier wall against the later) start charging its own gate
  once loudness stops charging one? (still standing.)
- - The ceiling located: both walls at ρ 0.50 — between
  equalturns' gateless 0.45 and this planting's gated 0.55,
  where does the second wall's identity reassemble (B-only
  32 at 0.45 pooled, 0 at 0.55; B-exact against pair 494/462
  and 553/553)? Is the ceiling a step or a slope — and does
  it sit at the same level in every regime, or does the twin
  cave cross it first?
- The gate's address: at LOUD, swap the walls' addresses —
  the same two shapes with the early and late bases exchanged:
  does the gate follow the later address (position) or the
  wall that held it here (shape)? Twinjoint's gate, equalturns'
  absence, and this planting's ceiling all sat on the second
  wall; no planting has yet moved a wall and watched the gate
  choose.

## Running It

```
cargo test -p listener-equalextremes
cargo run -p listener-equalextremes -- --headless
```

`--headless` is handled before anything else; there is no TUI.
The headless ledger prints, per level (quiet ρ 0.40, loud
ρ 0.55), per regime and per σ ∈ {0.00, 0.10, 0.20, 0.30}:
arrivals, naive / twincurve / twinjoint verdict errors,
twincurve and twinjoint pair-exact counts, the twinjoint
judge's per-wall exact counts and solo counts (A-only, B-only),
its per-width exact counts, and its ρ error mean/max, and — at
σ 0.00 — the pair the twincurve judge most often names in place
of the truth; then the ten declared predictions, judged on
identical evidence.
