# Listener Vpair 🦇

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
→ `experiments/listener-vdrift` (one wall that turns)
→ this planting.

**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-09. The predictions below were declared
before the shipped run and were not edited after it; one design
correction from the draft run is kept in the open below.

## The Seed This Planting Exists For

listener-vdrift closed with a named seed: *a V beside a second
wall — the joint in a pair: does the pair search's rent land on
the jointed wall first, as the census's rent landed on the
count?* Every Listener pair cave before this one held walls from
one family at a time — still, walking, or bending. This cave
mixes the families: a wall that turns, standing beside a wall
that only walks. The novel trait is a pair search whose two
walls come from different families — and the audit of where its
noise rent is charged, wall by wall.

## The Audit

A wall here is a two-step trajectory: a base delay and two steps
(s0, s1), one per glint gap — delay(0) = base, delay(1) =
base + s0, delay(2) = base + s0 + s1. A V wall turns at the
joint (s1 = −s0); a LINEAR wall walks straight (s1 = s0,
|s0| ≤ 1); a CURVED wall bends by at most one tick of accel
(s1 = s0 + accel, accel in {−1, 0, +1}, |s0| ≤ 1). A book of
recorded trains — 120 moths and 120 stones per regime, struck at
every phase of their periods in ripple's medium (moth
0.80 + 0.20·cos, period 12; stone 0.92 + 0.08·cos, period 17;
per-contribution noise in [−σ, +σ] drawn deterministically from
the train's identity and the contribution's channel; arrivals
under the 0.22 ear dropped) — is recorded in three caves, each
holding a V wall at **ρ = 0.55** beside a linear wall at
**ρ = 0.40**:

- **shallow V + still** — V (6, +1, −1): delays 6, 7, 6, echoes
  at 6, 11, 14; still wall (9, 0, 0): echoes at 9, 13, 17.
- **wide V down-up + walking away** — V (7, −2, +2): delays
  7, 5, 7, echoes at 7, 9, 15; walking wall (8, +1, +1): delays
  8, 9, 10, echoes at 8, 13, 18 — its first echo rides the third
  own glint.
- **wide V up-down + walking home** — V (5, +2, −2): delays
  5, 7, 5, echoes at 5, 11, 13; walking wall (9, −1, −1):
  delays 9, 8, 7, echoes at 9, 12, 15.

No V in these caves is a curved wall (the shallow V needs
accel −2; the wide Vs open with a two-tick step, outside the
curved family's reach), so the true pair is not in the curved
pair family's space in any cave — asserted by test.

Three judges read the same delivered record — amplitudes and
arrival offsets, nothing else:

- **Naive:** the first three arrivals against the 0.30 flutter
  line.
- **Twincurve:** listener-twincurve's search, carried whole —
  an exhaustive joint search over an unordered pair of CURVED
  trajectories and two strengths on a 0.05 grid. It can bend
  either wall by one tick. Neither of its walls may turn.
- **Vpair:** an ordered pair search — the first wall JOINTED
  (base 3..=9, each step free in {−2, −1, 0, +1, +2}, every
  per-glint delay inside 1..=12), the second LINEAR — with the
  strengths fitted beside each other: every ordered pair is
  first fitted by coordinate descent over the 0.05 grid (two
  rounds, each strength scanned holding the other), and the
  eight best-fitting pairs are then re-fitted on the full grid.
  A test asserts this two-stage search returns the same
  verdict, pair, and strengths as the exhaustive full-grid
  ordered search on sampled trains. Each candidate derives the
  own glints it implies, predicts the whole record, and is
  scored by the summed absolute difference over every offset
  either side mentions (sparse score with an abort bound that
  preserves exactness; a test asserts the sparse score equals
  the dense whole-record score).

## Predictions (declared before the shipped run)

- **P1:** the walled caves wound the naive judge in every
  regime — at σ 0.00 its errors are at least 20 in each regime.
- **P2:** the twincurve search is a curved judge in a turning
  cave — at σ 0.00 it names the true pair on zero trains in
  every regime (the V is not in its family).
- **P3:** no cave shelters the twincurve judge — at σ 0.00 it
  errs at least 20 times in every regime. (Re-declared after
  the draft; see the correction below.)
- **P4:** the vpair search prices the jointed pair in a clean
  cave — at σ 0.00 its errors are 0 in every regime, its pair
  estimate is exact on every train, and its ρ error max is
  ≤ 0.011 in every regime.
- **P5:** pooled over the three regimes at σ 0.10, the vpair
  judge's errors are below the naive judge's.
- **P6:** at σ 0.10 the vpair judge still names the pair: its
  pair estimate is exact on at least 90% of trains pooled.
- **P7:** the rent lands on the joint first — pooled over the
  three regimes at σ 0.30, the vpair judge names the V wall
  exactly on fewer trains than it names the second, linear
  wall exactly.
- **P8:** at σ 0.30, pooled, the vpair judge's errors are still
  below the naive judge's pooled errors.

## What Came Back

All eight predictions **HELD** (P3 as re-declared; its draft
form cracked, and the crack is kept below).

In the clean caves the jointed pair judge is exact in the
strong sense: errors 0 in every regime, pair exact 240/240 in
every regime, ρ error mean and max 0.000 — a turn beside a
straight wall is still six returns, and six returns are enough
to name both walls, when the search is allowed to hold a turn
in one hand and a walk in the other.

The naive judge is not merely wounded, it is *conscripted*: at
σ 0.00 it errs exactly 120 times in every regime — every stone
in the book judged a moth, no moth refused. Two walls' echoes
land inside its first three arrivals in every cave, and the
sums swing like a wingbeat.

The twincurve judge fails everywhere, and fails in the
verdicts. At σ 0.00 it errs 123 / 130 / 140 and names the true
pair on zero trains — at every σ surveyed, in fact. Its modal
substitutes are curved pairs that steal echoes through own
glints: ((4, +1, +1), (6, −1, −2)) on 64 trains in the
shallow-V cave; ((4, +1, +2), (9, 0, +1)) on 162 in the
down-up cave — neither wall true; and in the up-down cave
((8, −1, −2), (9, −1, −1)) on 75, where it names the true
straight wall exactly and spends its second wall straightening
the V. There is no shelter in a pair cave: vdrift's wide
down-up V sheltered the *single-wall* linear judge (errors 0,
verdict-perfect, trajectory-blind), but a judge with two walls
to spend always finds a substitute pair that pays for the
turn with a theft, and the theft lands in the verdicts.

Under noise the pair's rent is charged jointly, then severally.
At σ 0.10 the vpair judge posts 20 / 36 / 19 errors (the
unwalled book's own floor is 21 per regime) and names the pair
on 683/720 pooled; per wall, the V is named on 686 trains
pooled and the second wall on 683 — together. At σ 0.20 they
still drown together (477 against 465 pooled, the joint
nominally ahead). At σ 0.30 the bill separates: the V wall is
named on **293** trains pooled, the second wall on **352** —
the rent lands on the joint first, in every regime severally
(87 < 119, 104 < 128, 102 < 105), and only at depth. Pooled
verdicts at σ 0.30: vpair 304 against naive's 371 and
twincurve's 358.

## Design Correction Kept From the Planting

One, found by the cave itself before anything shipped: the
first draft declared the twincurve judge's failure would split
by regime on vdrift's pattern — the wide down-up V sheltering
it, errors under 20 there, as that V sheltered vdrift's linear
judge. The draft's own ledger refused it: twincurve errs
123 / 130 / 140, at least 20 in every regime, a verdict failure
in all three caves. The regime set was kept, the declaration
was re-made (P3 above: no cave shelters a pair judge), and the
predictions were re-declared before the shipped run; the
shipped run reproduced the draft's ledger line for line. The
shelter was never a property of the V — it was a property of a
judge with only one wall to misname. Give the judge a second
wall and it will spend it on the theft.

## The Law

Sixteenth strike: in a pair cave, ask which wall the rent
lands on — and at what depth the bill separates. A judge one
shape short of a pair does not shelter: with two walls to
spend it always finds a substitute that steals an echo through
an own glint, so its failure lives in its verdicts in every
regime, not merely in its naming. The judge that can hold the
pair pays its noise rent jointly at first — at σ 0.10 and
σ 0.20 the turning wall and the straight wall are named and
lost together, within a dozen trains of each other pooled —
and only at depth (σ 0.30) does the joint drown first,
293 against 352, in every regime severally. The paired
audit's question extends a sixth time: before trusting a
priced cave, ask how many walls the judge was allowed to
count, whether any could move, bend, or turn — and now, when
two walls stand together, which of them the judge loses
first when the record roughens, and whether the losing
started at the surface or only in the deep water.

**Falsification condition:** the vpair claim — that a turning
wall beside a straight one is recoverable from the delivered
record alone by an ordered search whose first wall's steps are
free and whose second wall walks straight — dies if, in a
clean cave, its pair estimate or verdict is wrong on any
train, or if at σ 0.10 its pooled pair-exact rate falls under
90%, or if a cave is built (same families, same ear, strengths
clearing the ear) whose records the curved pair judge prices
with pair exactness above zero — a bending judge naming a
turning pair exactly would kill the law's premise, not just
its numbers.

## Next Seeds

- Two joints in one cave — a V beside a second *turning* wall:
  does the rent still separate at depth, and which turn is
  lost first?
- Longer trains (four or five glints) — a wall with two joints,
  or one joint carried by more returns: how the naming's
  drowning σ moves with one more return per wall (the standing
  Listener seed, sharpened by this run's joint bill).
- A four-line census form in a cave that sometimes holds four
  walls (listener-census's standing seed, still standing).

## Running It

```
cargo test -p listener-vpair
cargo run -p listener-vpair -- --headless
```

`--headless` is handled before anything else; there is no TUI.
The headless ledger prints, per regime and per σ ∈ {0.00, 0.10,
0.20, 0.30}: arrivals, naive / twincurve / vpair verdict errors,
twincurve and vpair pair-exact counts, the vpair judge's
per-wall exact counts and ρ error mean/max, and — at σ 0.00 —
the pair the twincurve judge most often names in place of the
truth; then the eight declared predictions, judged on
identical evidence.
