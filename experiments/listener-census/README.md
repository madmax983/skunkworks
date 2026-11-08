# Listener Census 🦇

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
→ this planting.

**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-07. The predictions below were declared
before the first run and were not edited after it.

## The Seed This Planting Exists For

listener-twincurve closed with a named seed: *the counting problem
in a moving, bending cave — how many walls, before pricing any.*
Every Listener judge before this one was told, in its machinery if
not in its prose, how many walls the cave held: one, then two. This
cave does not say. The novel trait is a judge whose first question
is the count itself — and the audit of what the previous judges,
each exact in a cave of the size it assumed, do in caves of every
other size.

## The Audit

A book of recorded trains — 120 moths and 120 stones per cave,
struck at every phase of their periods in ripple's medium (moth
0.80 + 0.20·cos, period 12; stone 0.92 + 0.08·cos, period 17;
per-contribution noise in [−σ, +σ] drawn deterministically from
the train's identity and the contribution's channel; arrivals
under the 0.22 ear dropped) — is recorded in four caves. Every
wall is a curved trajectory (base delay, drift −1/0/+1,
acceleration −1/0/+1; the step from glint *j* to glint *j+1* is
drift + accel·*j*), strengths by position **ρ = 0.55 / 0.45 /
0.40**:

- **one bending wall** — [(5, 0, +1)]: echoes at 5, 9, 14.
- **two walls, one bending** — [(4, +1, +1), (9, −1, 0)]:
  twincurve's curving + walking regime; echoes at 4, 9, 15 and
  9, 12, 15.
- **three walls, one bending** — [(4, +1, +1), (9, −1, 0),
  (6, 0, −1)]: the third wall's echoes land at 6, 10, 13.
- **three walls, all bending** — [(7, −1, −1), (5, −1, +1),
  (9, 0, +1)]: echoes at 7, 10, 12; 5, 8, 12; 9, 13, 18.

Three judges read the same delivered record — amplitudes and
arrival offsets, nothing else:

- **Naive:** the first three arrivals against the 0.30 flutter
  line.
- **Pair:** listener-twincurve's search, carried whole — a joint
  search over an ordered pair of curved trajectories and two
  strengths. It assumes exactly two walls, because its machinery
  has no way to hold any other number.
- **Census:** a beam search over counts. At each count up to
  three it keeps the six best forms in hand, each expanded from
  the previous count's beam by every trajectory it does not
  already hold, with the set's strengths fitted *jointly* —
  coordinate descent over the 0.05 grid, each wall's strength
  re-priced beside the others. The count is where the buying
  stops: the smallest count whose successor buys less than 0.25
  of whole-record score. The chosen form is then refined — each
  wall re-read beside the others — while that helps.

## Predictions (declared before the first run)

- **P1:** the naive judge is wounded in every cave — at σ 0.00
  its errors are at least 20 in each cave.
- **P2:** the pair judge is a two-wall judge — at σ 0.00 it
  prices the two-wall cave exactly (errors 0, trajectory set
  exact on every train) and names the true trajectory set on
  zero trains in the one-wall and both three-wall caves.
- **P3:** the census counts a clean cave exactly — at σ 0.00
  its count is right on every train in every cave, its
  trajectory set is exact on every train, and its verdict
  errors are 0 in every cave.
- **P4:** at σ 0.10 the census still counts: pooled over the
  four caves, its count is exact on at least 90% of trains.
- **P5:** pooled at σ 0.10, the census judge's verdict errors
  are below the naive judge's.
- **P6:** at σ 0.30, pooled, the census judge's verdict errors
  are still below the naive judge's pooled errors.

## What Came Back

All six predictions **HELD**.

In the clean cave the census is exact in the strong sense: count
240/240 and trajectory set 240/240 in all four caves, verdict
errors 0 everywhere, and the count histogram is a single spike on
the truth in each cave (240 counted at the true count, zero at
every other). The pair judge, read beside it, shows the shape of
the assumption it carries: in its home cave it is perfect (errors
0, set exact 240/240); in the one-wall cave its *verdicts* are
also perfect (errors 0 — a spare wall at near-zero strength is a
harmless fiction for the verdict) while its set is exact on zero
trains; in the three-wall caves it comes apart — errors 160 and
150 of 240, worse than naive's 120 in both, refusing and fooled
in both directions. A judge with the wrong count does not merely
fail to name the cave; in a crowded cave it judges *worse than
the judge that prices nothing*.

Under noise the count is the first thing the medium eats, and it
is eaten from the top: at σ 0.10 the census counts exactly on
904/960 pooled (one-wall 240/240, two-wall 233/240, three-wall
230/240 and 201/240), and the errors are overwhelmingly
*undercounts* — in the all-bending cave 39 of 240 trains are
counted as two walls, almost none as anything else. A wall lost
to noise is not invented elsewhere; it is folded into its
neighbours' strengths (set-exact falls faster than count-exact:
103/240 in that cave at σ 0.10, 7/240 at σ 0.30). Pooled verdict
errors: σ 0.10 census 116 against naive 491; σ 0.30 census 417
against naive 494 — the census still wins the pooled ledger at
the deepest noise surveyed, though in the two-wall cave alone at
σ 0.30 the pair judge (98) beats it (109): where the count is
known, the counter's freedom is pure rent.

Two wrinkles kept honestly. In the clean two- and three-wall
caves the census's ρ error is not exactly zero (mean 0.019 /
0.013, max 0.05) even where set and score are exact: at coincident
offsets, two walls' echoes sum, and neighbouring strength pairs
along the sum's ridge fit the record equally — the record prices
the *sum* exactly and the split only to the grid's mercy. And the
headless run takes about four minutes, most of it the pair
judge's exhaustive grid, re-priced in every cave to prove the
point about its assumptions.

## Design Corrections Kept From the Planting

Three, all found by the cave itself before anything shipped:

1. **Strengths are fitted jointly.** The first census fitted one
   wall at a time and froze its strength. In the two-wall cave
   the near wall, fitted alone, takes ρ 0.95 — stealing the far
   wall's third echo at offset 15, which it can reach and the
   record cannot refuse — and the frozen theft then blocks the
   true pair (best reachable score 0.48 against the truth's 0).
   A strength is only ever priced beside the walls it stands
   with.
2. **The count is searched by beam, not by greed.** With joint
   strengths the staged census still stuck in the three-wall
   cave: a mirage pair — (4, +1, +1) at ρ 0.95 beside (6, −1, +1),
   one wall drawn through two walls' returns — scored 0.778 and
   blocked every single addition, though the true three walls
   score 0. Six forms in hand, and the true form survives to be
   expanded even when a mirage leads it.
3. **Every wall must clear the ear.** The first cave put the
   third wall at ρ 0.30; its faintest echoes (0.30 × 0.60 = 0.18,
   and 0.21 at common amplitudes) fall under the 0.22 ear and
   never arrive — a wall the medium swallows whole cannot be
   counted, only mourned. Strengths by position are therefore
   0.55 / 0.45 / 0.40, whose weakest possible echo (0.24) still
   arrives.

## The Law

Fourteenth strike: before pricing a cave, count it — and count
it with more than one form in hand. A judge that assumes its
count judges worse than the judge that prices nothing wherever
the assumption is wrong in the crowded direction; a judge that
counts greedily is hostage to its first line, because a single
wall fitted alone will steal strength for echoes that are not
its own and a mirage pair will bar the door against the truth.
What the count costs under noise is also now priced: the count
drowns before the verdict does, it drowns downward (walls fold
into their neighbours, they are not invented), and where the
count is already known the counter's freedom is rent — the pair
judge's home cave is the one room where assuming beats counting
at σ 0.30.

**Falsification condition:** the census's claim — that the wall
count is recoverable from the delivered record alone, by a
search that is never told it — dies if, in a clean cave, its
count or trajectory set is wrong on any train, or if at σ 0.10
its pooled count-exact rate falls under 90%, or if a cave is
built (same trajectory family, same ear, strengths clearing the
ear) whose records it systematically overcounts — invented walls
would kill the law's direction, not just its numbers.

## Next Seeds

- A wall whose drift changes sign mid-flight (a V in the
  record) — twindrift's standing seed, still standing: the
  trajectory family itself gains a joint.
- Longer trains (four or five glints): a third wall is carried
  by fewer returns than a second; price how the count's
  drowning σ moves with one more return per wall.
- A census whose form has four lines, in a cave that sometimes
  holds four walls — does the beam's rent grow linearly, and
  does the undercount's direction survive?

## Running It

```
cargo test -p listener-census
cargo run -p listener-census -- --headless
```

`--headless` is handled before anything else; there is no TUI.
The headless ledger prints, per cave and per σ ∈ {0.00, 0.10,
0.20, 0.30}: arrivals, naive / pair / census verdict errors,
pair and census set-exact counts, the census count histogram,
its count-exact count, and its ρ error mean/max — then the six
declared predictions, judged on identical evidence.
