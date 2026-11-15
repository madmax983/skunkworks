# Listener Thirtysecond 🦇

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
each — the identity arriving cave by cave, dearest first) →
`experiments/listener-sixteenth` (the sixteenth: the gate back
pooled and severally at ρ 0.5375, B-only 0 in every regime, the
twin cave's lead exactly one sixteenth, and the grid speaking at
quarter-step, ρ error exactly 0.0125) → this planting.

**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-15. The predictions below were declared
before the shipped run and were not edited after it.

## The Seed This Planting Exists For

listener-sixteenth closed with a named seed: *the thirty-second
grain — both walls at ρ 0.53125, midway between the fine grain
and the sixteenth: the mixed caves crossed somewhere in
(0.525, 0.5375] — did they cross in the same thirty-second, or
staggered (one in (0.525, 0.53125], one in (0.53125, 0.5375]),
and does the twin cave's B-only stay 0 there, as a crossed cave
should?*

The row so far, all of it at σ 0.10 pooled over twinjoint's three
caves with both walls at one shared strength: at ρ 0.525 the twin
cave crossed entire (B-only 0 there, pair 102) while each mixed
cave shed two of its three and stood at B-only 1 (pooled pair
556, B 558, B-only 2); at 0.5375 the gate stood back pooled and
severally (pair 560, walls 570 and 560, solos 10 and 0; severally
99 / 221 / 240, B-only 0 in every regime). Each mixed cave
therefore crossed somewhere inside a single sixteenth of
loudness. This planting halves that sixteenth: THIRTYSECOND,
both walls at ρ = 0.53125 (total loudness 1.0625), struck in the
same run on the same book as its SIXTEENTH control at ρ 0.5375 —
so the thirty-second is read against the crossed row with no
drift of instrument between them.

It asks the seed's question in its sharpest form. Each mixed cave
stands exactly one train short of the identity at the fine grain.
One thirty-second of loudness either takes that last train in
both caves (the crossings share the first thirty-second), in
neither (both crossings wait for the second), or in exactly one
— the closing staggered, as the row's arrivals have always been
staggered, cave by cave, dearest first. This planting declares
the stagger (P6 pooled, P11 severally) — and declares, in P12,
that the instrument will betray the grain in a second, quieter
way.

## The Grid Note (read this before the ledger)

The grain caught the instrument at the fine grain and at the
sixteenth, and it catches it again here, at a third distance.
The judges fit strengths on a 0.05 grid; ρ 0.53125 is not a point
on it. Its nearest grid point is 0.55, 0.01875 away; the next is
0.50, 0.03125 below. In a clean cave the twinjoint judge should
name the true pair on every train at THIRTYSECOND — and its
fitted strength should sit 0.01875 away, so its ρ error reads
exactly 0.01875, mean and max, in every regime. That is the grid
speaking, not the cave: the naming can be exact while the
strength is quantized, and at this grain the quantization is
three-eighths of the fit grid's own step. P4 below was written
with the quantization inside it (ρ error max ≤ 0.020 at
THIRTYSECOND, the sixteenth's ≤ 0.014 kept at SIXTEENTH, where
0.5375 sits a quarter-step below 0.55), P12 declares the exact
0.01875 outright, and the one inherited test that asserts a
clean-cave ρ bound carries the same level-specific caps, marked
where it stands. Midceiling's staging repair (0.1 lattice, pool
of 128, one enumeration order for both searches) is carried
whole.

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
  in each regime at ρ 0.53125 and at ρ 0.5375.
- **P2:** the twincurve search is a curved judge in a turning
  cave — at σ 0.00 it names the true pair on zero trains in
  every regime, at both levels.
- **P3:** the echo-swap shelter stands at both levels — in
  the shallow + shallow, opposed cave at σ 0.00, the twincurve
  judge still errs fewer than 20 times while naming the true
  pair on zero trains, at ρ 0.53125 and at ρ 0.5375 (the swap is
  an offset mechanism; it cared about neither inequality, level,
  midpoint, fine grain, nor sixteenth, and a thirty-second of
  loudness should not move it).
- **P4:** the twinjoint search prices the pair in a clean cave
  at both levels — at σ 0.00 its errors are 0 in every regime
  and its pair estimate is exact on every train at both levels,
  and its ρ error max is ≤ 0.020 at THIRTYSECOND (the 0.05 grid's
  0.01875 distance to 0.53125, declared in the grid note)
  and ≤ 0.014 at SIXTEENTH, in every regime.
- **P5:** pooled over the three regimes at σ 0.10, the
  twinjoint judge's errors are below the naive judge's, at
  both levels.
- **P6:** THE CLOSING IS STAGGERED, POOLED — pooled at
  σ 0.10 at THIRTYSECOND, B-only reads exactly 1: at least one
  mixed cave has crossed and at least one still stands, so the
  pooled ceiling narrows to a thirty-second but does not close
  there. If pooled B-only reads 0, both crossings share the
  first thirty-second; if it reads 2, neither does, and this
  prediction dies either way it is wrong.
- **P7:** loudness still pays the pair — pooled pair exact at
  σ 0.10 at SIXTEENTH exceeds pooled pair exact at THIRTYSECOND.
- **P8:** the twin cave stays the dearest at both levels — at
  σ 0.10, at EACH level the shallow + shallow, opposed regime's
  pair exact is below BOTH mixed regimes' pair exact.
- **P9:** the rent still gates on width alone at both
  levels — pooled over the three regimes at σ 0.30, at EACH
  level the shallow walls' exact rate is below the wide walls'
  exact rate.
- **P10:** at σ 0.30, pooled, the twinjoint judge's errors are
  still below the naive judge's pooled errors, at both levels.
- **P11:** THE CLOSING IS STAGGERED, SEVERALLY — at
  THIRTYSECOND, σ 0.10, the twin cave's B-only stays 0 (a
  crossed cave stays crossed), and exactly ONE of the two mixed
  regimes reads B-only 0 while the other still reads ≥ 1: the
  two crossings keep separate addresses, one in (0.525, 0.53125]
  and one in (0.53125, 0.5375]. If both mixed caves read 0, or
  both still stand, or the twin cave reopens, this prediction
  dies there.
- **P12:** THE GRID SPEAKS AT THREE-EIGHTHS — at σ 0.00 at
  THIRTYSECOND, the twinjoint judge's ρ error mean and max both
  read 0.01875 (within 0.001) in every regime: the naming
  exact, the strength 0.01875 off, because 0.55 is the nearest
  point the fit grid can land on. Any other clean-cave
  ρ error at the thirty-second kills it.

## What Came Back

All twelve predictions **HELD** — the closing was staggered
at the thirty-second grain, and the stagger was declared before
it was read.

In the clean caves the twinjoint judge is exact in the strong
sense at both levels: errors 0 in every regime, pair exact
240/240 in every regime — and at THIRTYSECOND its ρ error is
**0.01875**, mean and max, in every regime (printed 0.019), the
grid's three-eighths made visible (at SIXTEENTH, 0.0125, printed
0.013). **P12 held as declared:** the naming is exact on every
train while the strength sits 0.01875 off, because 0.55 is the
nearest point the 0.05 fit grid can land on. The naive judge errs
exactly 120 times in every regime at both levels, every stone
judged a moth. **P3 held with the same substitute on the same
trains:** in the opposed cave the twincurve judge errs **0**
at σ 0.00 at both levels, modal substitute ((6, +1, +2),
(9, −1, −2)) on all 240 trains at both — the echo-swap does not
care about a thirty-second of loudness either.

**P6 and P11 held entire: the closing is staggered, pooled and
severally.** At THIRTYSECOND, σ 0.10: the opposed cave stands
pair **97**, B-only **0** — the identity whole there since the
fine grain, kept; wide + shallow stands pair **219**, B **219**,
B-only **0** — crossed, in the first thirty-second; shallow +
wide stands pair **239**, B **240**, B-only **1** — one train
short, its crossing waiting for the second thirty-second.
Pooled: pair **555**, walls **565** and **556**, solos **10**
and **1** — the pooled ceiling narrows to (0.525, 0.53125] for
one mixed cave and stands in (0.53125, 0.5375] for the other.
The sixteenth control in the same run reproduces sixteenth's
planted ledger exact: pair **560** (99 / 221 / 240), B **560**,
B-only **0** severally, A-only **10** — the instrument did not
drift between plantings.

**P7 held by five trains, and the climb's shape is the
sixteenth's lesson repeated at finer grain:** pooled pair exact
at σ 0.10 reads 556 at the fine grain, **555** at the
thirty-second, **560** at the sixteenth — the pair's total
actually *fell* by one across a thirty-second in which a whole
cave finished crossing, and the opposed cave's pair fell again
too (102 at the fine grain, **97** here, 99 at the sixteenth)
on a cave whose identity never wavered. The solo ledger is
where the closing lives; the pair's total cannot even keep its
sign. **P8 held at both levels:** the twin cave stays the
dearest (thirtysecond 97 against 219 and 239; sixteenth 99
against 221 and 240). **P9 held at both levels:** σ 0.30 pooled
shallow against wide exact rates **0.265 against 0.467** at
THIRTYSECOND and **0.278 against 0.479** at SIXTEENTH — width
charges at the thirty-second as at every grain before it.

At depth the crossed cave keeps the gate's shadow: at
THIRTYSECOND, σ 0.20, the wide first wall of the wide + shallow
cave is named alone on **70** trains while its shallow partner
is named alone on 5 (at SIXTEENTH in the same run, 76 against
3) — crossing at σ 0.10 does not dismiss the shadow at σ 0.20,
at either grain. And the first wall's solo ledger stays open at
both levels in this run: A-only pooled reads **10** at
THIRTYSECOND and **10** at SIXTEENTH.

Pooled verdicts: σ 0.10 twinjoint **70** (thirtysecond) and
**74** (sixteenth) against naive 362 and 360; σ 0.30 twinjoint
293 and 294 against naive 385 and 378 (pair exact pooled at
σ 0.30: 144/720 thirtysecond — 36/26/82 by regime; 155/720
sixteenth — 40/30/85).

12/12 tests — including the inherited staged-vs-exhaustive
test, which passes at 0.53125 unamended against midceiling's
repaired staging, and the clean-cave test with the
level-specific ρ caps declared in the grid note above (0.020
at THIRTYSECOND, 0.014 at SIXTEENTH).

## The Law

Twenty-third strike: the row leaves cave by cave, as it
arrived. The identity returned at the fine grain in one cave
alone — the dearest, first — and at the thirty-second grain
the closing repeats the arrival's shape exactly: of the two
mixed caves that stood one train short, wide + shallow took
its last train in the first thirty-second (its ceiling in
(0.525, 0.53125]) and shallow + wide kept its train into the
second (its ceiling in (0.53125, 0.5375]). There is no pooled
ceiling at this grain, only cave ceilings a thirty-second
apart, and pooled B-only 1 is the fingerprint of a staggered
closing: a pooled count of one solo train is not a row one
train from closing, it is one cave closed and one cave open.
The sixteenth's lesson sharpens here: across the first
thirty-second the pair's pooled total fell (556 → 555) while
an entire cave crossed — the totals do not merely flatten at
fine grain, they can move against the identity while the
identity completes. Watch the solo ledger, cave by cave; it
is the only ledger that has ever told this row's truth. And
the instrument betrayed the grain a third way, as declared:
at 0.53125 the clean-cave ρ error is exactly 0.01875 in every
regime — the judge names both walls perfectly and misstates
their loudness by three-eighths of its own grid step, because
the level it was asked to price is not a point its grid can
land on. The paired audit's question extends a thirteenth
time: before trusting a priced cave, ask how many walls the
judge was allowed to count, whether any could move, bend, or
turn, which of them gates the naming, whether the losing is
charged to width, to loudness, or to likeness, at what level a
gate reassembles, in which cave it arrives first, whether the
judge's fit grid can land on the level, when the totals have
gone flat whether you are watching the pair's count or the
solo ledger — and now, when a pooled solo count reads one,
whether it is one cave's last train or the row's.

**Falsification condition:** the thirty-second claim — that the
two mixed caves' crossings keep separate addresses, one in
(0.525, 0.53125] and one in (0.53125, 0.5375], with the twin
cave's B-only staying 0 at the thirty-second — dies if, at
σ 0.10 at THIRTYSECOND, both mixed caves read B-only 0 (the
crossings shared the first thirty-second), or both still read
≥ 1 (both waited for the second), or the twin cave's B-only
reopens above 0, or if the twinjoint judge errs or misnames in
a clean cave at either level, or if its clean-cave ρ error at
THIRTYSECOND is anything but the grid's 0.01875, or if the twin
cave stops being the dearest cave at a level.

## Next Seeds

- The sixty-fourth grain, for the lagging cave alone:
  shallow + wide stands at B-only 1 at ρ 0.53125 and crossed
  by 0.5375 — at ρ 0.534375, midway through its thirty-second,
  does its last train change verdict (its ceiling in
  (0.53125, 0.534375]) or does the cave hold to the second
  half (0.534375, 0.5375]? And does wide + shallow, struck at
  the same address as a control, stay crossed?
- The first wall's own gate: A-only pooled reads 10 at the
  thirty-second and 10 at the sixteenth (8 at LOUD in
  sixteenth's own run) — the second wall's identity is whole
  at every level yet struck while the first wall is still
  named alone. Is there a level at or below LOUD where A-only
  reaches 0 too, or does the first wall's solo ledger close
  only above 0.55, in country this row has not struck?
  (still standing.)
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
cargo test -p listener-thirtysecond
cargo run -p listener-thirtysecond -- --headless
```

`--headless` is handled before anything else; there is no TUI.
The headless ledger prints, per level (thirtysecond ρ 0.53125,
sixteenth ρ 0.5375), per regime and per σ ∈ {0.00, 0.10, 0.20,
0.30}: arrivals, naive / twincurve / twinjoint verdict errors,
twincurve and twinjoint pair-exact counts, the twinjoint
judge's per-wall exact counts and solo counts (A-only, B-only),
its per-width exact counts, and its ρ error mean/max, and — at
σ 0.00 — the pair the twincurve judge most often names in place
of the truth; then the twelve declared predictions, judged on
identical evidence.
