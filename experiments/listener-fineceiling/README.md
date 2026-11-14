# Listener Fineceiling 🦇

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
floor) → `experiments/listener-midceiling` (the midpoint struck:
the gate dead at ρ 0.50 too, B-only 8 spread flat 2/3/3, its
cave-by-cave slope guess cracked — nothing had crossed) → this
planting.

**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-13. The predictions below were declared
before the shipped run and were not edited after it.

## The Seed This Planting Exists For

listener-midceiling closed with a named seed: *the ceiling at
finer grain — both walls at ρ 0.525, the last eighth halved:
B-only pooled reads 8 at 0.50 and 0 at 0.55 — at 0.525 does any
cave reach zero (a slope in the last eighth after all), or does
the whole row step together between 0.525 and 0.55?*

The row so far, all of it at σ 0.10 pooled over twinjoint's
three caves with both walls at one shared strength: at ρ 0.40
the gate identity is dead (pair 370, solos 48 and 50); at 0.45
dead (pair 462, solos 21 and 32); at 0.50 dead but thinning
(pair 538, B-only 8, spread 2 / 3 / 3 over the caves — no cave
at zero); at 0.55 alive on the second wall (pair 563, second
wall 563, B-only 0 — in every regime severally). This planting
halves the last eighth: FINE, both walls at ρ = 0.525 (total
loudness 1.05), struck in the same run on the same book as its
LOUD control at ρ 0.55 — so the fine grain is read against the
gated end with no drift of instrument between them.

It asks the seed's question in its sharpest form. Midceiling's
slope guess — mixed caves cross first, the twin cave last —
cracked at the midpoint: nothing had crossed anywhere. Two
readings remain. If the last eighth is a step, every cave still
stands short at 0.525 and the identity arrives whole somewhere
in (0.525, 0.55]. If it is a slope, some cave reaches B-only 0
at the fine grain while another still stands at ≥ 1. This
planting declares the step (P11) — and declares, in P6, that
pooled the gate is still not back.

## The Grid Note (read this before the ledger)

The fine grain caught the instrument before it caught the gate,
in a different place than the midpoint did. The judges fit
strengths on a 0.05 grid; ρ 0.525 is not a point on it. In a
clean cave the twinjoint judge names the true pair on every
train at FINE — and its fitted strength sits a half-step away,
so its ρ error reads exactly 0.025, mean and max, in every
regime. That is the grid speaking, not the cave: the naming can
be exact while the strength is quantized. P4 below was written
with the quantization inside it (ρ error max ≤ 0.026 at FINE,
the parent's ≤ 0.011 kept at LOUD, where 0.55 is a grid point),
and the one inherited test that asserted a level-blind 0.011
bound was amended to the same level-specific caps — the
amendment is marked in the test, and everything else in it
(errors 0, pair exact on every train, naive wounded) stands
unamended. Midceiling's staging repair (0.1 lattice, pool of
128, one enumeration order for both searches) is carried whole,
and its staged-vs-exhaustive test passes at 0.525 unamended:
the staging can see this level even though the fit grid cannot
land on it.

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
  (0.1 steps, midceiling's staging repair carried whole —
  see the grid note above), and the 128 best-scoring pairs are
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
  in each regime at ρ 0.525 and at ρ 0.55.
- **P2:** the twincurve search is a curved judge in a turning
  cave — at σ 0.00 it names the true pair on zero trains in
  every regime, at both levels.
- **P3:** the echo-swap shelter stands at both levels — in
  the shallow + shallow, opposed cave at σ 0.00, the twincurve
  judge still errs fewer than 20 times while naming the true
  pair on zero trains, at ρ 0.525 and at ρ 0.55 (the swap is an
  offset mechanism; it cared about neither inequality, level,
  nor midpoint, and a half-step of loudness should not move it).
- **P4:** the twinjoint search prices the pair in a clean cave
  at both levels — at σ 0.00 its errors are 0 in every regime
  and its pair estimate is exact on every train at both levels,
  and its ρ error max is ≤ 0.026 at FINE (the 0.05 grid's
  half-step quantization at 0.525, declared in the grid note)
  and ≤ 0.011 at LOUD, in every regime.
- **P5:** pooled over the three regimes at σ 0.10, the
  twinjoint judge's errors are below the naive judge's, at
  both levels.
- **P6:** THE GATE IS NOT YET BACK AT THE FINE GRAIN — pooled
  at σ 0.10 at FINE, the second wall's exact count does NOT
  equal the pair-exact count, and B is named alone on at least
  one train (B-only ≥ 1): the ceiling lies above 0.525. If
  B-only reads 0 at the fine grain, the ceiling sits at or
  below it and this prediction dies there.
- **P7:** loudness still pays the pair — pooled pair exact at
  σ 0.10 at LOUD exceeds pooled pair exact at FINE.
- **P8:** the twin cave stays the dearest at both levels — at
  σ 0.10, at EACH level the shallow + shallow, opposed regime's
  pair exact is below BOTH mixed regimes' pair exact.
- **P9:** the rent still gates on width alone at both
  levels — pooled over the three regimes at σ 0.30, at EACH
  level the shallow walls' exact rate is below the wide walls'
  exact rate.
- **P10:** at σ 0.30, pooled, the twinjoint judge's errors are
  still below the naive judge's pooled errors, at both levels.
- **P11:** THE LAST EIGHTH IS A STEP, NOT A SLOPE — at FINE,
  σ 0.10, B-only is ≥ 1 in EVERY regime: no cave has crossed
  at 0.525, so the whole row steps together somewhere in
  (0.525, 0.55]. If any single cave reads B-only 0 at the fine
  grain while another still reads ≥ 1, the last eighth is a
  slope after all and this prediction dies there.

## What Came Back

Ten of the eleven predictions **HELD**; P11 **CRACKED** —
and the crack answers the seed's question with a third shape
neither guess had named.

In the clean caves the twinjoint judge is exact in the strong
sense at both levels: errors 0 in every regime, pair exact
240/240 in every regime — and at FINE its ρ error is exactly
**0.025**, mean and max, in every regime, the grid's half-step
made visible (at LOUD, 0.000). The naive judge errs exactly
120 times in every regime at both levels, every stone judged
a moth. **P3 held with the same substitute on the same
trains:** in the opposed cave the twincurve judge errs **0**
at σ 0.00 at both levels, modal substitute ((6, +1, +2),
(9, −1, −2)) on all 240 trains at both — the echo-swap does
not care about a half-step of loudness either.

**P6 held: pooled, the gate is not back at the fine grain.**
At FINE, σ 0.10 pooled, the pair-exact count is **556**, the
walls' exact counts are **566** and **558**, and the solos are
**10** and **2** — the second wall is still named alone on 2
trains, so its naming is not yet the pair's naming, and the
ceiling lies in (0.525, 0.55]. The loud control in the same
run reassembles the identity entire, exactly as midceiling's
repaired instrument read it: pair **563**, second wall
**563**, B-only **0**, severally 0 in every regime.

**P11 cracked: one cave HAS crossed at the fine grain — and
it is the twin cave.** B-only at FINE, σ 0.10, reads **0** in
the shallow + shallow, opposed cave (pair 102, A 102, B 102,
both solos 0 — the identity whole, in that cave, at 0.525),
**1** in wide + shallow (pair 216, B 217), and **1** in
shallow + wide (pair 238, B 239). So the last eighth is not a
step: somewhere at or below 0.525 the first cave crossed. And
it is not midceiling's slope either, which had the mixed caves
crossing first and the twin cave — the dearest at every level
yet struck — crossing last. The order inverted: the cave that
charges the most for the pair (P8 held again: at FINE its pair
exact is **102** against the mixed caves' 216 and 238) is the
cave where the second wall's naming first becomes the pair's.
Between 0.50 and 0.525 the pooled B-only count fell from 8 to
2, and the fall was not spread flat this time: the twin cave's
2 vanished entire, and each mixed cave shed 2 of its 3,
leaving 1 apiece. The solo count still slopes; the identity
arrives cave by cave; but the dearest cave arrives first.

**P7 held by the row's narrowest margin yet:** pooled pair
exact at σ 0.10 reads 370 at ρ 0.40, 462 at 0.45, 538 at
0.50, **556** at 0.525, **563** at 0.55 — still monotone, and
the last eighth buys 18 of the midpoint-to-loud climb's 25.
**P9 held at both levels:** σ 0.30 pooled shallow against
wide exact rates **0.256 against 0.456** at FINE and **0.297
against 0.494** at LOUD — width charges at the fine grain as
at every grain before it.

At depth the gate's shadow keeps its address: at FINE,
σ 0.20, the wide first wall of the wide + shallow cave is
named alone on **71** trains while its shallow partner is
named alone on 5 (midceiling read 71 against 9 at ρ 0.50) —
the asymmetry that precedes the gate is full-grown at the
fine grain too, in the cave that has not crossed, while the
cave that has crossed shows solos 11 and 12, near even.

Pooled verdicts: σ 0.10 twinjoint **73** (fine) and **78**
(loud) against naive 362 and 360; σ 0.30 twinjoint 293 and
292 against naive 385 and 372 (pair exact pooled at σ 0.30:
140/720 fine — 34/27/79 by regime; 169/720 loud — 42/34/93).

12/12 tests — including the inherited staged-vs-exhaustive
test, which passes at 0.525 unamended against midceiling's
repaired staging, and the clean-cave test with the one marked
amendment declared in the grid note above (its ρ bound is
level-specific: 0.026 at FINE, 0.011 at LOUD).


## The Law

Twenty-first strike: the ceiling is an address in the last
eighth, and it is reached cave by cave — in the opposite order
from the one the row guessed. Midceiling found the gate dead
at ρ 0.50 in every cave severally (B-only 2 / 3 / 3) and its
slope guess cracked; this planting halved the eighth and found
the identity already whole in one cave at ρ 0.525 — the
shallow + shallow, opposed cave, the twin cave, the dearest
cave at every level yet struck — while both mixed caves still
stand one train short (B-only 1 each) and the pooled ledger
still reads un-crossed (pair 556, B 558, B-only 2). So the
identity does not arrive pooled-first or dearest-last: it
arrives where resemblance is thickest, first, and the pooled
count is the last place it shows. And the fine grain priced
the instrument a second way: at a level the fit grid cannot
land on, the clean-cave ρ error is exactly the half-step
(0.025, every regime) while the naming is exact on every
train — a judge can name a wall perfectly and still be unable
to say how loud it is, to within its own grid. The paired
audit's question extends an eleventh time: before trusting a
priced cave, ask how many walls the judge was allowed to
count, whether any could move, bend, or turn, which of them
gates the naming, whether the losing is charged to width, to
loudness, or to likeness, at what level a gate reassembles,
whether the judge's staging can see the level it was asked to
price, which way it breaks a tie — and now whether its fit
grid can even land on that level, and in which cave the
identity will arrive first: not the cheapest cave, if this
grain is honest, but the dearest.


**Falsification condition:** the fineceiling claim — that the
gate identity reassembles cave by cave in the last eighth with
the twin cave first, the pooled identity still open above
0.525 — dies if, at σ 0.10 pooled at FINE, B-only is 0 and B's
exact count equals the pair's (the gate already whole at
0.525: a ceiling at or below the fine grain), or if at FINE
every regime's B-only is still ≥ 1 (a step above 0.525 after
all), or if a mixed cave reads B-only 0 at FINE while the
opposed cave still reads ≥ 1 (midceiling's order, not this
one), or if the twinjoint judge errs or misnames in a clean
cave at either level, or if its clean-cave ρ error at FINE is
anything but the grid's half-step, or if the twin cave stops
being the dearest cave at a level.

## Next Seeds

- The last sixteenth: both walls at ρ 0.5375 — the last
  eighth halved again. The mixed caves stand at B-only 1 each
  at 0.525 and the pooled count at 2: do both mixed caves
  cross at 0.5375 (the pooled ceiling in the sixteenth), or
  does one lag to 0.55 — and is the twin cave's early crossing
  a lead of a full eighth, or does the whole row close within
  one sixteenth of the loud end?
- The gate's address: at LOUD, swap the walls' addresses —
  the same two shapes with the early and late bases exchanged:
  does the gate follow the later address (position) or the
  wall that held it here (shape)? Twinjoint's gate,
  equalturns' absence, equalextremes' ceiling, midceiling's
  midpoint, and this planting's fine grain all sat on — or
  spared — the second wall; no planting has yet moved a wall
  and watched the gate choose. (still standing.)
- The tie census: at the midpoint, in the opposed cave, the
  truth and its echo-swap substitute tied exactly on a sampled
  train at σ 0.20 (0.4140 at the same fitted strengths). How
  many trains tie exactly at each level — and does the tie
  rate peak at the midpoint, where the two walls' strengths
  are nearest the swap's own fitted strengths (0.45, 0.55)?
  The twin cave crossed first at the fine grain; the tie lives
  in that cave, and the census now has a second reason to be
  taken.
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
cargo test -p listener-fineceiling
cargo run -p listener-fineceiling -- --headless
```

`--headless` is handled before anything else; there is no TUI.
The headless ledger prints, per level (fine ρ 0.525, loud
ρ 0.55), per regime and per σ ∈ {0.00, 0.10, 0.20, 0.30}:
arrivals, naive / twincurve / twinjoint verdict errors,
twincurve and twinjoint pair-exact counts, the twinjoint
judge's per-wall exact counts and solo counts (A-only, B-only),
its per-width exact counts, and its ρ error mean/max, and — at
σ 0.00 — the pair the twincurve judge most often names in place
of the truth; then the eleven declared predictions, judged on
identical evidence.
