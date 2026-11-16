# Listener Sixtyfourth 🦇

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
quarter-step, ρ error exactly 0.0125) →
`experiments/listener-thirtysecond` (the thirty-second: the closing
staggered — wide + shallow crossed in the first thirty-second,
shallow + wide stood one train short into the second, and the grid
spoke at three-eighths, ρ error exactly 0.01875) → this planting.

**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-16. The predictions below were declared
before the shipped run and were not edited after it. Three of them
cracked, and the cracks are the planting's findings, reported as
such below.

## The Seed This Planting Exists For

listener-thirtysecond closed with a named seed: *the sixty-fourth
grain, for the lagging cave alone: shallow + wide stands at B-only
1 at ρ 0.53125 and crossed by 0.5375 — at ρ 0.534375, midway
through its thirty-second, does its last train change verdict
(its ceiling in (0.53125, 0.534375]) or does the cave hold to the
second half (0.534375, 0.5375]? And does wide + shallow, struck at
the same address as a control, stay crossed?*

The row so far, all of it at σ 0.10 pooled over twinjoint's three
caves with both walls at one shared strength: at ρ 0.525 the twin
cave crossed entire while each mixed cave stood at B-only 1; at
0.53125 wide + shallow crossed entire (B-only 0, pair 219) while
shallow + wide stood at pair 239, B 240, B-only 1 — one train
short (pooled pair 555, B-only 1); at 0.5375 the gate stood back
pooled and severally (pair 560, B-only 0 in every regime). The
lagging cave's ceiling therefore stood inside a single
thirty-second of loudness. This planting halves that
thirty-second: SIXTYFOURTH, both walls at ρ = 0.534375 (total
loudness 1.06875), struck in the same run on the same book as its
SIXTEENTH control at ρ 0.5375 — so the sixty-fourth is read
against the crossed row with no drift of instrument between them.

It asks the seed's question in its sharpest form, and this
planting declared the conservative answer: the lagging cave
*holds* — its last train still standing at the sixty-fourth, its
ceiling in the second half of its thirty-second (P6 pooled, P11
severally), with wide + shallow as the control that stays crossed.
The cave disagreed. Both declarations cracked, in the same
direction, and the section below reports the crack before anything
else that held.

## The Grid Note (read this before the ledger)

The grain caught the instrument at the fine grain, at the
sixteenth, and at the thirty-second, and it catches it again here,
at a fourth distance. The judges fit strengths on a 0.05 grid;
ρ 0.534375 is not a point on it. Its nearest grid point is 0.55,
0.015625 away; the next is 0.50, 0.034375 below. In a clean cave
the twinjoint judge should name the true pair on every train at
SIXTYFOURTH — and its fitted strength should sit 0.015625 away,
so its ρ error reads exactly 0.015625, mean and max, in every
regime. That is the grid speaking, not the cave: the naming can
be exact while the strength is quantized, and at this grain the
quantization is five-sixteenths of the fit grid's own step. P4
below was written with the quantization inside it (ρ error max
≤ 0.017 at SIXTYFOURTH, the sixteenth's ≤ 0.014 kept at
SIXTEENTH, where 0.5375 sits a quarter-step below 0.55), P12
declares the exact 0.015625 outright, and the one inherited test
that asserts a clean-cave ρ bound carries the same level-specific
caps, marked where it stands. Midceiling's staging repair (0.1
lattice, pool of 128, one enumeration order for both searches) is
carried whole.

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
  in each regime at ρ 0.534375 and at ρ 0.5375.
- **P2:** the twincurve search is a curved judge in a turning
  cave — at σ 0.00 it names the true pair on zero trains in
  every regime, at both levels.
- **P3:** the echo-swap shelter stands at both levels — in
  the shallow + shallow, opposed cave at σ 0.00, the twincurve
  judge still errs fewer than 20 times while naming the true
  pair on zero trains, at ρ 0.534375 and at ρ 0.5375 (the swap
  is an offset mechanism; it cared about neither inequality,
  level, midpoint, fine grain, sixteenth, nor thirty-second,
  and a sixty-fourth of loudness should not move it).
- **P4:** the twinjoint search prices the pair in a clean cave
  at both levels — at σ 0.00 its errors are 0 in every regime
  and its pair estimate is exact on every train at both levels,
  and its ρ error max is ≤ 0.017 at SIXTYFOURTH (the 0.05 grid's
  0.015625 distance to 0.534375, declared in the grid note)
  and ≤ 0.014 at SIXTEENTH, in every regime.
- **P5:** pooled over the three regimes at σ 0.10, the
  twinjoint judge's errors are below the naive judge's, at
  both levels.
- **P6:** THE LAGGING CAVE HOLDS, POOLED — pooled at
  σ 0.10 at SIXTYFOURTH, B-only reads exactly 1: the one solo
  train left in the row is still standing at the sixty-fourth,
  so the lagging cave's ceiling narrows to the second half of
  its thirty-second, (0.534375, 0.5375]. If pooled B-only
  reads 0, the cave crossed in the first half and this
  prediction dies; if it reads 2 or more, a crossed cave
  reopened and it dies the other way.
- **P7:** loudness still pays the pair — pooled pair exact at
  σ 0.10 at SIXTEENTH exceeds pooled pair exact at SIXTYFOURTH.
- **P8:** the twin cave stays the dearest at both levels — at
  σ 0.10, at EACH level the shallow + shallow, opposed regime's
  pair exact is below BOTH mixed regimes' pair exact.
- **P9:** the rent still gates on width alone at both
  levels — pooled over the three regimes at σ 0.30, at EACH
  level the shallow walls' exact rate is below the wide walls'
  exact rate.
- **P10:** at σ 0.30, pooled, the twinjoint judge's errors are
  still below the naive judge's pooled errors, at both levels.
- **P11:** THE LAGGING CAVE HOLDS, SEVERALLY, AND THE CONTROL
  STAYS CROSSED — at SIXTYFOURTH, σ 0.10, the twin cave's
  B-only stays 0, wide + shallow's B-only stays 0 (a crossed
  cave stays crossed at the same address), and shallow +
  wide's B-only still reads ≥ 1: its ceiling stands in
  (0.534375, 0.5375]. If shallow + wide reads 0, its ceiling
  was in the first half; if wide + shallow reopens, the
  control failed; either way this prediction dies there.
- **P12:** THE GRID SPEAKS AT FIVE-SIXTEENTHS — at σ 0.00 at
  SIXTYFOURTH, the twinjoint judge's ρ error mean and max both
  read 0.015625 (within 0.001) in every regime: the naming
  exact, the strength 0.015625 off, because 0.55 is the nearest
  point the fit grid can land on. Any other clean-cave
  ρ error at the sixty-fourth kills it.

## What Came Back

Nine predictions **HELD** and three **CRACKED** — and the three
cracks are one finding read three ways: the lagging cave's last
train was standing much closer to the thirty-second's own address
than to the sixteenth's.

**P6 and P11 cracked, pooled and severally, in the direction the
seed's first branch named.** At SIXTYFOURTH, σ 0.10: the opposed
cave stands pair **104**, B-only **0** — whole there since the
fine grain, kept; wide + shallow stands pair **221**, B **221**,
B-only **0** — the control stayed crossed, exactly as a control
should; and shallow + wide stands pair **240/240**, A **240**,
B **240**, B-only **0** — *crossed entire, in the first half of
its thirty-second.* Its ceiling is in (0.53125, 0.534375], not
the second half this planting declared. Pooled: pair **565**,
walls **575** and **565**, solos **10** and **0** — at the
sixty-fourth the gate identity is whole in the row, pooled and
severally, one sixty-fourth earlier than the sixteenth where
this row first read it whole in a single run's own level. The
sixteenth control in the same run reproduces thirtysecond's
planted sixteenth ledger exact: pair **560** (99 / 221 / 240),
B **560**, B-only **0** severally, A-only **10** — the instrument
did not drift between plantings; the cave moved.

**P7 cracked, and its crack is the thirty-second's lesson with
the sign reversed.** Pooled pair exact at σ 0.10 reads 556 at
the fine grain, **555** at the thirty-second, **565** at the
sixty-fourth, **560** at the sixteenth: across the first
sixty-fourth the total *rose by ten* while a cave completed its
crossing, and across the second it *fell by five* on a row whose
identity never wavered. The totals do not merely flatten at fine
grain, and they do not merely move against the identity — at
this grain they move in both directions inside a single
thirty-second, ten up and five down, while the solo ledger
moves once, monotonically, 1 → 0, and stops. The opposed cave's
own pair count traces the same weather: 102 at the fine grain,
97 at the thirty-second, **104** here, 99 at the sixteenth — a
cave whose B-only has read 0 at every level since the fine grain
swings ±7 trains of pair count on weather alone.

Around the cracks, the held row held entire. In the clean caves
the twinjoint judge is exact in the strong sense at both levels:
errors 0 in every regime, pair exact 240/240 in every regime —
and at SIXTYFOURTH its ρ error is **0.015625**, mean and max, in
every regime (printed 0.016), the grid's five-sixteenths made
visible (at SIXTEENTH, 0.0125, printed 0.013). **P12 held as
declared.** The naive judge errs exactly 120 times in every
regime at both levels, every stone judged a moth. **P3 held
with the same substitute on the same trains:** in the opposed
cave the twincurve judge errs **0** at σ 0.00 at both levels,
modal substitute ((6, +1, +2), (9, −1, −2)) on all 240 trains at
both — the echo-swap does not care about a sixty-fourth of
loudness either. **P8 held at both levels:** the twin cave stays
the dearest (sixtyfourth 104 against 221 and 240; sixteenth 99
against 221 and 240). **P9 held at both levels:** σ 0.30 pooled
shallow against wide exact rates **0.271 against 0.471** at
SIXTYFOURTH and **0.278 against 0.479** at SIXTEENTH — width
charges at the sixty-fourth as at every grain before it.

At depth the crossed caves keep the gate's shadow: at
SIXTYFOURTH, σ 0.20, the wide first wall of the wide + shallow
cave is named alone on **73** trains while its shallow partner
is named alone on 3 (at SIXTEENTH in the same run, 76 against
3) — crossing at σ 0.10 does not dismiss the shadow at σ 0.20,
at either grain. And the first wall's solo ledger stays open at
both levels in this run: A-only pooled reads **10** at
SIXTYFOURTH and **10** at SIXTEENTH.

Pooled verdicts: σ 0.10 twinjoint **74** (sixtyfourth) and
**74** (sixteenth) against naive 360 and 360; σ 0.30 twinjoint
294 and 294 against naive 378 and 378 (pair exact pooled at
σ 0.30: 148/720 sixtyfourth — 36/26/86 by regime; 155/720
sixteenth — 40/30/85).

12/12 tests — including the inherited staged-vs-exhaustive
test, which passes at 0.534375 unamended against midceiling's
repaired staging, and the clean-cave test with the
level-specific ρ caps declared in the grid note above (0.017
at SIXTYFOURTH, 0.014 at SIXTEENTH).

## The Law

Twenty-fourth strike: the last train was standing at the
doorstep, not in the room. Thirtysecond left shallow + wide one
train short at ρ 0.53125, and this planting declared — on the
row's own habit of arriving late and staggered — that the train
would still be standing halfway through its thirty-second. It
was not: at ρ 0.534375 the cave reads pair 240/240, B-only 0,
crossed entire, so its ceiling stands in the *first* half of the
thirty-second, hard against the thirty-second's own address.
Halving an interval does not halve the waiting inside it: a
cave's last solo train has an address, not a residence time, and
the address can sit anywhere in the interval — here, within a
sixty-fourth of the level that left it standing. And the pair's
total, which fell by one across the thirty-second in which wide
+ shallow crossed, rose by ten across the sixty-fourth in which
shallow + wide crossed and fell by five across the next, on an
identity that never moved: at fine grain the total is weather,
the solo ledger is climate. The paired audit's question extends
a fourteenth time: before trusting a priced cave, ask how many
walls the judge was allowed to count, whether any could move,
bend, or turn, which of them gates the naming, whether the
losing is charged to width, to loudness, or to likeness, at what
level a gate reassembles, in which cave it arrives first,
whether the judge's fit grid can land on the level, when the
totals have gone flat whether you are watching the pair's count
or the solo ledger, when a pooled solo count reads one whether
it is one cave's last train or the row's — and now, when a
cave's last train falls, in which half of its interval it was
standing, for the interval's midpoint is a guess about an
address, not a measurement of one.

**Falsification condition:** the sixty-fourth claim — that
shallow + wide's ceiling stands in (0.53125, 0.534375], with
the twin cave's and wide + shallow's B-only staying 0 at the
sixty-fourth — dies if, at σ 0.10 at SIXTYFOURTH, shallow +
wide's B-only reads ≥ 1 (its ceiling in the second half after
all), or either crossed cave's B-only reopens above 0, or if
the twinjoint judge errs or misnames in a clean cave at either
level, or if its clean-cave ρ error at SIXTYFOURTH is anything
but the grid's 0.015625, or if the twin cave stops being the
dearest cave at a level.

## Next Seeds

- The hundred-and-twenty-eighth grain, for the lagging cave
  alone again: shallow + wide's ceiling now stands in
  (0.53125, 0.534375] — at ρ 0.5328125, midway through its
  sixty-fourth, does its last train change verdict there too
  (its ceiling in (0.53125, 0.5328125], hard against the
  thirty-second a second time) or does the cave hold into the
  second sixty-fourth (0.5328125, 0.534375]? And does the
  opposed cave's pair count — 102, 97, 104, 99 across the last
  four levels, weather on a crossed cave — swing again at the
  finer grain?
- The first wall's own gate: A-only pooled reads 10 at the
  sixty-fourth and 10 at the sixteenth (8 at LOUD in
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
cargo test -p listener-sixtyfourth
cargo run -p listener-sixtyfourth -- --headless
```

`--headless` is handled before anything else; there is no TUI.
The headless ledger prints, per level (sixtyfourth ρ 0.534375,
sixteenth ρ 0.5375), per regime and per σ ∈ {0.00, 0.10, 0.20,
0.30}: arrivals, naive / twincurve / twinjoint verdict errors,
twincurve and twinjoint pair-exact counts, the twinjoint
judge's per-wall exact counts and solo counts (A-only, B-only),
its per-width exact counts, and its ρ error mean/max, and — at
σ 0.00 — the pair the twincurve judge most often names in place
of the truth; then the twelve declared predictions, judged on
identical evidence.
