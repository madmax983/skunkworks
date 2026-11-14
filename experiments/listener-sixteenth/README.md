# Listener Sixteenth 🦇

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
the gate dead at ρ 0.50 too, B-only 8 spread flat 2/3/3) →
`experiments/listener-fineceiling` (the fine grain: the twin cave
crossed entire at ρ 0.525 while both mixed caves stood at B-only 1
each — the identity arriving cave by cave, dearest first) → this
planting.

**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-14. The predictions below were declared
before the shipped run and were not edited after it.

## The Seed This Planting Exists For

listener-fineceiling closed with a named seed: *the last sixteenth
— both walls at ρ 0.5375, the last eighth halved again: the mixed
caves stand at B-only 1 each at 0.525 and the pooled count at 2 —
do both mixed caves cross at 0.5375 (the pooled ceiling in the
sixteenth), or does one lag to 0.55, and is the twin cave's early
crossing a lead of a full eighth or does the whole row close
within one sixteenth of the loud end?*

The row so far, all of it at σ 0.10 pooled over twinjoint's three
caves with both walls at one shared strength: at ρ 0.40 the gate
identity is dead (pair 370, solos 48 and 50); at 0.45 dead (pair
462, solos 21 and 32); at 0.50 dead but thinning (pair 538, B-only
8, spread 2 / 3 / 3 — no cave at zero); at 0.525 the twin cave
crossed entire (B-only 0 there, pair 102, the dearest cave first)
while each mixed cave shed two of its three and stood at B-only 1
(pooled pair 556, B 558, B-only 2); at 0.55 alive on the second
wall in every regime severally (pair 563, B 563, B-only 0). This
planting halves the last eighth again: SIXTEENTH, both walls at
ρ = 0.5375 (total loudness 1.075), struck in the same run on the
same book as its LOUD control at ρ 0.55 — so the sixteenth is
read against the gated end with no drift of instrument between
them.

It asks the seed's question in its sharpest form. Each mixed cave
stands exactly one train short of the identity. One more
sixteenth of loudness either takes that last train in both caves
— the pooled ceiling stands in the sixteenth, and the twin cave's
lead over the row was a single sixteenth — or it leaves one cave
standing, and the row's closing is staggered across the whole
last eighth after all. This planting declares the closing (P6
pooled, P11 severally) — and declares, in P12, that the
instrument will betray the grain in a second, quieter way.

## The Grid Note (read this before the ledger)

The grain caught the instrument at the fine grain, and it catches
it again here, at a different distance. The judges fit strengths
on a 0.05 grid; ρ 0.5375 is not a point on it. Its nearest grid
point is 0.55, a quarter-step (0.0125) away; the next is 0.50,
three quarter-steps below. In a clean cave the twinjoint judge
should name the true pair on every train at SIXTEENTH — and its
fitted strength should sit a quarter-step away, so its ρ error
reads exactly 0.0125, mean and max, in every regime. That is the
grid speaking, not the cave: the naming can be exact while the
strength is quantized, and at this grain the quantization is a
quarter of the step that priced the fine grain's half-step. P4
below was written with the quantization inside it (ρ error max
≤ 0.014 at SIXTEENTH, the parent's ≤ 0.011 kept at LOUD, where
0.55 is a grid point), P12 declares the exact quarter-step
outright, and the one inherited test that asserts a clean-cave
ρ bound carries the same level-specific caps, marked where it
stands. Midceiling's staging repair (0.1 lattice, pool of 128,
one enumeration order for both searches) is carried whole.

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
  (0.1 steps, midceiling's staging repair carried whole — see
  the grid note above), and the 128 best-scoring pairs are
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
  in each regime at ρ 0.5375 and at ρ 0.55.
- **P2:** the twincurve search is a curved judge in a turning
  cave — at σ 0.00 it names the true pair on zero trains in
  every regime, at both levels.
- **P3:** the echo-swap shelter stands at both levels — in
  the shallow + shallow, opposed cave at σ 0.00, the twincurve
  judge still errs fewer than 20 times while naming the true
  pair on zero trains, at ρ 0.5375 and at ρ 0.55 (the swap is an
  offset mechanism; it cared about neither inequality, level,
  midpoint, nor fine grain, and a sixteenth of loudness should
  not move it).
- **P4:** the twinjoint search prices the pair in a clean cave
  at both levels — at σ 0.00 its errors are 0 in every regime
  and its pair estimate is exact on every train at both levels,
  and its ρ error max is ≤ 0.014 at SIXTEENTH (the 0.05 grid's
  quarter-step distance to 0.5375, declared in the grid note)
  and ≤ 0.011 at LOUD, in every regime.
- **P5:** pooled over the three regimes at σ 0.10, the
  twinjoint judge's errors are below the naive judge's, at
  both levels.
- **P6:** THE GATE IS BACK AT THE SIXTEENTH, POOLED — pooled
  at σ 0.10 at SIXTEENTH, the second wall's exact count EQUALS
  the pair-exact count, and B is named alone on no train
  (B-only = 0): the pooled ceiling lies in (0.525, 0.5375]. If
  B-only still reads ≥ 1 pooled at the sixteenth, the ceiling
  stands above it and this prediction dies there.
- **P7:** loudness still pays the pair — pooled pair exact at
  σ 0.10 at LOUD exceeds pooled pair exact at SIXTEENTH.
- **P8:** the twin cave stays the dearest at both levels — at
  σ 0.10, at EACH level the shallow + shallow, opposed regime's
  pair exact is below BOTH mixed regimes' pair exact.
- **P9:** the rent still gates on width alone at both
  levels — pooled over the three regimes at σ 0.30, at EACH
  level the shallow walls' exact rate is below the wide walls'
  exact rate.
- **P10:** at σ 0.30, pooled, the twinjoint judge's errors are
  still below the naive judge's pooled errors, at both levels.
- **P11:** THE ROW CLOSES WITHIN ONE SIXTEENTH — at
  SIXTEENTH, σ 0.10, B-only is 0 in EVERY regime: both mixed
  caves have crossed, the whole row stands within one
  sixteenth of the loud end, and the twin cave's lead was a
  single sixteenth. If any regime still reads B-only ≥ 1 at
  the sixteenth, that cave lags to (0.5375, 0.55] and this
  prediction dies there.
- **P12:** THE GRID SPEAKS AT QUARTER-STEP — at σ 0.00 at
  SIXTEENTH, the twinjoint judge's ρ error mean and max both
  read 0.0125 (within 0.001) in every regime: the naming
  exact, the strength a quarter-step off, because 0.55 is the
  nearest point the fit grid can land on. Any other clean-cave
  ρ error at the sixteenth kills it.

## What Came Back

All twelve predictions **HELD** — the row closes at the
sixteenth, and the closing was declared before it was read.

In the clean caves the twinjoint judge is exact in the strong
sense at both levels: errors 0 in every regime, pair exact
240/240 in every regime — and at SIXTEENTH its ρ error is
exactly **0.0125**, mean and max, in every regime, the grid's
quarter-step made visible (at LOUD, 0.000). **P12 held as
declared:** the naming is exact on every train while the
strength sits a quarter-step off, because 0.55 is the nearest
point the 0.05 fit grid can land on. The naive judge errs
exactly 120 times in every regime at both levels, every stone
judged a moth. **P3 held with the same substitute on the same
trains:** in the opposed cave the twincurve judge errs **0**
at σ 0.00 at both levels, modal substitute ((6, +1, +2),
(9, −1, −2)) on all 240 trains at both — the echo-swap does
not care about a sixteenth of loudness either.

**P6 and P11 held entire: the gate is back at the sixteenth,
pooled and severally.** At SIXTEENTH, σ 0.10, B-only reads
**0** in every regime: the opposed cave stands pair **99**,
A 99, B 99, both solos 0 — the identity whole there since the
fine grain, kept; wide + shallow stands pair **221**, B 221,
B-only 0 (A-only 10); shallow + wide stands pair **240/240**,
the regime exact on every train at σ 0.10, B-only 0. Pooled:
pair **560**, walls **570** and **560**, solos **10** and
**0** — the second wall's naming is the pair's naming, so the
pooled ceiling lies in (0.525, 0.5375]. The loud control in
the same run stands as the row has always read it: pair
**563**, B **563**, B-only **0**, severally 0 in every regime
(101 / 222 / 240).

**P7 held by three trains:** pooled pair exact at σ 0.10
reads 370 at ρ 0.40, 462 at 0.45, 538 at 0.50, 556 at 0.525,
**560** at 0.5375, **563** at 0.55 — still monotone, and the
climb is nearly spent: the last eighth bought 25, its first
half bought 18, its second half buys 4, and the final
sixteenth to LOUD buys 3. The identity's return and the
pair's enrichment part company at this grain: B-only fell
2 → 0 across the sixteenth while the pair gained almost
nothing, and the opposed cave's pair exact actually *fell*
(102 at the fine grain, 99 here) on a cave whose identity
never wavered — a cave can grow no richer and still finish
crossing. **P8 held at both levels:** the twin cave stays
the dearest (sixteenth 99 against 221 and 240; loud 101
against 222 and 240). **P9 held at both levels:** σ 0.30
pooled shallow against wide exact rates **0.278 against
0.479** at SIXTEENTH and **0.297 against 0.494** at LOUD —
width charges at the sixteenth as at every grain before it.

At depth the crossed cave keeps the gate's shadow: at
SIXTEENTH, σ 0.20, the wide first wall of the wide + shallow
cave is named alone on **76** trains while its shallow
partner is named alone on 3 (fineceiling read 71 against 5
at the fine grain, in a cave that had not crossed; the
crossing at σ 0.10 does not dismiss the shadow at σ 0.20).
And one solo ledger stays open at both levels: A-only pooled
reads **10** at SIXTEENTH and **8** at LOUD — the gate this
row prices was always the second wall's; the first wall is
still named alone on trains at every level yet struck.

Pooled verdicts: σ 0.10 twinjoint **74** (sixteenth) and
**78** (loud) against naive 360 and 360; σ 0.30 twinjoint
294 and 292 against naive 378 and 372 (pair exact pooled at
σ 0.30: 155/720 sixteenth — 40/30/85 by regime; 169/720
loud — 42/34/93).

12/12 tests — including the inherited staged-vs-exhaustive
test, which passes at 0.5375 unamended against midceiling's
repaired staging, and the clean-cave test with the level-
specific ρ caps declared in the grid note above (0.014 at
SIXTEENTH, 0.011 at LOUD).

## The Law

Twenty-second strike: the ceiling stands in the sixteenth,
and the row that priced it cave by cave can now be read
whole. The identity was dead at ρ 0.50 in every cave (B-only
2 / 3 / 3); at 0.525 it stood whole in the twin cave alone —
the dearest cave, first — while both mixed caves stood one
train short; at 0.5375 it stands whole everywhere, so the
twin cave's lead over the row was exactly one sixteenth, and
the pooled ceiling the row kept asking for is an address in
(0.525, 0.5375]. But the sixteenth prices a second lesson
beside the address: the pair's enrichment and the identity's
return are different currencies. Across the last eighth the
pair climbed 538 → 556 → 560 → 563 — each halving buying
less (18, 4, 3) — while the identity arrived entire inside
that flattening climb, in caves whose pair counts barely
moved and in one cave whose count fell. A gate does not
reassemble because the cave grew richer; it reassembles
when the last solo train changes its verdict, and that
train is invisible in the pair's total. And the instrument
betrayed the grain a second way, as declared: at 0.5375 the
clean-cave ρ error is exactly the quarter-step 0.0125 in
every regime — the judge names both walls perfectly and
cannot say how loud they are to within a quarter of its own
grid step, because the level it was asked to price is not a
point its grid can land on. The paired audit's question
extends a twelfth time: before trusting a priced cave, ask
how many walls the judge was allowed to count, whether any
could move, bend, or turn, which of them gates the naming,
whether the losing is charged to width, to loudness, or to
likeness, at what level a gate reassembles, in which cave it
arrives first, whether the judge's fit grid can land on the
level — and now, when the totals have gone flat, whether
you are watching the pair's count at all, or the solo
ledger where the identity actually moves.

**Falsification condition:** the sixteenth claim — that the
gate identity reassembles pooled and severally in
(0.525, 0.5375], with the twin cave's lead exactly one
sixteenth — dies if, at σ 0.10 at SIXTEENTH, any regime's
B-only is ≥ 1 or pooled B-only is ≥ 1 (the ceiling above the
sixteenth after all), or if pooled B's exact count differs
from the pair's at SIXTEENTH, or if the twinjoint judge errs
or misnames in a clean cave at either level, or if its
clean-cave ρ error at SIXTEENTH is anything but the grid's
quarter-step, or if the twin cave stops being the dearest
cave at a level.

## Next Seeds

- The thirty-second grain: both walls at ρ 0.53125, midway
  between the fine grain and the sixteenth. The mixed caves
  crossed somewhere in (0.525, 0.5375] — did they cross in
  the same thirty-second, or staggered (one in
  (0.525, 0.53125], one in (0.53125, 0.5375])? And does the
  twin cave's B-only stay 0 there, as a crossed cave should?
- The first wall's own gate: A-only pooled reads 10 at the
  sixteenth and 8 at LOUD — the second wall's identity is
  whole at both levels while the first wall is still named
  alone. Is there a level at or below LOUD where A-only
  reaches 0 too, or does the first wall's solo ledger close
  only above 0.55, in country this row has not struck?
- The gate's address: at LOUD, swap the walls' addresses —
  the same two shapes with the early and late bases
  exchanged: does the gate follow the later address
  (position) or the wall that held it here (shape)?
  (still standing.)
- The tie census: at the midpoint, in the opposed cave, the
  truth and its echo-swap substitute tied exactly on a
  sampled train at σ 0.20 (0.4140 at the same fitted
  strengths). How many trains tie exactly at each level —
  and does the tie rate peak at the midpoint, where the two
  walls' strengths are nearest the swap's own fitted
  strengths (0.45, 0.55)? (still standing.)
- Longer trains (four or five glints) — a wall with two
  joints, or one joint carried by more returns: how the
  naming's drowning σ moves with one more return per wall
  (the standing Listener seed, still standing).
- A four-line census form in a cave that sometimes holds
  four walls (listener-census's standing seed, still
  standing).
- The reopened shelter, priced in noise: the shallow-opposed
  cave where two bends swap third echoes — at what σ does
  the swap stop paying the curved judge's verdicts, and does
  the twinjoint judge's naming in that cave drown at the
  same σ or a different one? (still standing.)
- The solo ledger at depth: at σ 0.20 and 0.30, do the two
  walls' solo counts stay comparable, or does position (the
  earlier wall against the later) start charging its own
  gate once loudness stops charging one? (still standing.)

## Running It

```
cargo test -p listener-sixteenth
cargo run -p listener-sixteenth -- --headless
```

`--headless` is handled before anything else; there is no TUI.
The headless ledger prints, per level (sixteenth ρ 0.5375, loud
ρ 0.55), per regime and per σ ∈ {0.00, 0.10, 0.20, 0.30}:
arrivals, naive / twincurve / twinjoint verdict errors,
twincurve and twinjoint pair-exact counts, the twinjoint
judge's per-wall exact counts and solo counts (A-only, B-only),
its per-width exact counts, and its ρ error mean/max, and — at
σ 0.00 — the pair the twincurve judge most often names in place
of the truth; then the twelve declared predictions, judged on
identical evidence.
