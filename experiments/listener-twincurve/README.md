# Listener Twincurve 🦇

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
→ this planting.

**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-07. The predictions below were declared
before the first run and were not edited after it.

## The Seed This Planting Exists For

listener-twindrift closed with a named seed: *a wall that
accelerates — drift itself drifting within the flight, now with a
second wall in the room, priced by a search that must give a wall
a curvature.* This planting takes it: every wall is a curved
trajectory — a base delay, a drift of −1, 0, or +1 ticks per glint,
an acceleration of −1, 0, or +1 (the step from glint *j* to glint
*j+1* is drift + accel·*j*, so the three delays are base,
base + drift, base + 2·drift + accel), and a strength — and a judge
that would price the cave must name two curvatures at once.

## The Audit

A book of recorded trains — 120 moths and 120 stones per regime,
struck at every phase of their periods in ripple's medium (moth
0.80 + 0.20·cos, period 12; stone 0.92 + 0.08·cos, period 17;
per-contribution noise in [−σ, +σ] drawn deterministically from
the train's identity and the contribution's channel — own, near
wall, far wall each carry their own draw; arrivals under the
0.22 ear dropped) — is recorded behind three pairs of walls, the
near wall at **ρ = 0.55**, the far wall at **ρ = 0.40**:

- **curving + still** — near (5, 0, +1), far (9, 0, 0): the near
  wall's echoes land at 5, 9, 14; the far wall's at 9, 13, 17.
  Offset 9 sums the curving wall's second echo with the still
  wall's first echo.
- **curving + walking** — near (4, +1, +1), far (9, −1, 0): echoes
  at 4, 9, 15 and 9, 12, 15. Offset 9 sums two echoes from two
  different walls; offset 15 sums both walls' third echoes.
- **both curving, opposite bends** — near (7, −1, −1), far
  (5, −1, +1): echoes at 7, 10, 12 and 5, 8, 12. Offset 12 sums
  both walls' third echoes; offset 8 is an own glint carrying the
  far wall's second echo.

The record window runs to offset 20 so both walls' full returns
are delivered. Three judges read the *same* delivered record —
amplitudes and arrival offsets, nothing else:

- **Naive:** the first three arrivals, spread (max−min)/mean vs
  the **0.30** line — flutter's judge.
- **Twindrift:** listener-twindrift's search — a joint search over
  an ordered pair of *linear* trajectories (base in 3..=9, drift
  in {−1, 0, +1}, accel fixed at 0) and two strengths on a 0.05
  grid. It can move either wall. Neither of its walls may bend.
  It is never told any wall bends.
- **Twincurve:** a joint search over an ordered pair of curved
  trajectories — base in 3..=9, drift in {−1, 0, +1}, accel in
  {−1, 0, +1}, every per-glint delay inside 1..=12 — and two
  strengths on the same 0.05 grid (the true strengths sit on that
  grid). Each candidate derives its own glints (subtracting, in
  strike order, any earlier glint's echo from *either* trajectory
  that lands exactly on a later own offset), predicts the whole
  record, and is scored by the summed absolute difference between
  prediction and delivery over every offset either mentions
  (computed sparsely over the ≤ 9 predicted offsets, with an exact
  abort bound: a candidate is abandoned only when its partial
  lower bound — residuals summed plus delivered mass at offsets
  it never mentions — provably exceeds the incumbent's score; a
  test asserts the abort score equals the dense whole-record
  score whenever it completes, and that a limit below the true
  score abandons rather than misreports). The true trajectories
  and strengths are the book's ground truth, never given to the
  judge.

**Falsification condition:** if the twincurve search errs on any
train in the clean cave, or names a wrong trajectory pair on any
clean train, the whole-record score does *not* separate two
curvatures — some bending wall hides inside the other's price —
and this planting's premise is false. If the linear twindrift
judge recovers any regime exactly at σ 0.00, curvature was never
load-bearing in that regime and the seed's premise cracks there.

**Novel trait:** every previous Listener planting priced at most
one bend short of this: twindrift gave two walls each a velocity
(six unknowns). This is the first planting where a wall's velocity
is itself unknown within the flight — eight unknowns (two bases,
two drifts, two accelerations, two strengths) — and where the
curving wall's echoes can sum with the other wall's echoes at two
different offsets in the same train (offsets 9 and 15 of the
curving + walking regime), so neither wall's bend can be read off
any single return.

## Declared Predictions (before the first run)

- **P1:** Two walls wound the naive judge in every regime: at
  σ 0.00 its errors are at least 20 in each regime.
- **P2:** The twindrift search is a linear-pair judge: at σ 0.00
  it errs at least 20 times in each regime, including the regime
  where only one wall bends, and it names the true trajectory
  pair on zero trains in every regime (the true pair is not in
  its search space — every regime's near wall bends).
- **P3:** The twincurve search prices both trajectories in a
  clean cave: at σ 0.00 its errors are 0 in every regime, its
  trajectory-pair estimate is exact on every train, and its ρ
  error max (either wall) is ≤ 0.011.
- **P4:** Pooled over the three regimes at σ 0.10, the twincurve
  judge's errors are below the naive judge's.
- **P5:** At σ 0.10 the twincurve judge still names both
  trajectories: its pair estimate is exact on at least 90% of
  trains pooled.
- **P6:** At σ 0.30, pooled over the three regimes, the twincurve
  judge's errors are still below the naive judge's pooled
  errors — the noise floor does not fully swallow the search.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

240 trains judged per regime, 12 regimes (3 regimes × 4 noise
levels). 12/12 tests. Sanity first: the unwalled control
reproduces the lineage's clean book line for line — naive
errors **0** at σ 0.00, **21** at σ 0.10, **62** at σ 0.20,
**98** at σ 0.30 — in every regime, because the clean book does
not depend on the walls.

- **σ 0.00:** naive **120 / 140 / 120** by regime (curving +
  still / curving + walking / both curving). Twindrift **111 /
  115 / 120**. Twincurve **0 errors in every regime**, trajectory
  pair exact **240/240** in each, ρ error mean and max **0.000**.
  Eight unknowns — two bases, two drifts, two accelerations, two
  strengths — separated exactly by the whole-record score. Two
  bending walls do not price each other in a clean cave.
- **The linear judge in a bending cave:** the twindrift search
  names the true pair on **zero trains at every σ in every
  regime** — structurally, since no regime's pair lives in its
  space — and its verdicts are barely better than guessing the
  motion away: at σ 0.00 it still errs 111 / 115 / 120 against
  naive's 120 / 140 / 120, and at σ 0.10 it errs **122 in the
  curving + still regime, two more than the naive judge's 120**,
  and posts the naive ledger exactly (120) in the opposite-bends
  regime. One bend is enough: a judge that can move its walls
  but not bend them confabulates straighter caves, exactly as
  twindrift's still-pair judge confabulated still ones.
- **σ 0.10:** twincurve pooled **92** vs naive pooled **371**;
  trajectory exact **669/720** pooled (220 / 232 / 217 by
  regime). By regime the twincurve judge errs **22 / 29 / 41** —
  the opposite-bends regime is the expensive one at this σ,
  while the highest ρ error mean sits in curving + walking
  (**0.075** against 0.047 / 0.054): where both walls' third
  echoes sum (offset 15), the bend is read off *where* arrivals
  land and stays named, while strength is read off the summed
  loudness and pays.
- **σ 0.20:** twincurve **75 / 84 / 82** by regime (pooled 241
  vs naive 363); trajectory exact falls to 140 / 196 / 152 of
  240 — curvature is the first unknown noise eats, because a
  bend is carried by a single return (the third echo's address)
  where a drift is carried by two.
- **σ 0.30:** twincurve pooled **319** vs naive pooled **367**,
  against a pooled clean floor of 294 — the eight-unknown search
  is 25 trains off the noise floor even here, the same distance
  twindrift's six-unknown search posted in its own cave.
  Trajectory exact falls to 103 / 149 / 75 of 240.

**Verdicts:** every declared line HELD. P1 (120 / 140 / 120, all
≥ 20). P2 (twindrift 111 / 115 / 120, all ≥ 20, and trajectory
exact 0 in every regime — the true pair is not in its space).
P3 (twincurve 0 everywhere in a clean cave, trajectory exact
240/240 in each regime, ρ error max 0.000). P4 (92 < 371). P5
(669/720, not merely 90%). P6 (319 < 367).

**The law this row adds:** a pair of curvatures separates the
same way a pair of trajectories did — exactly, in a clean cave —
because a bend, like a drift, is a schedule of addresses: two
schedules that differ anywhere write somewhere the other cannot
follow, and the third echo is where a curvature writes its name.
What bending breaks is, again, every judge one dimension short
of the record: the linear-pair judge, which twindrift proved
exact against two walking walls, posts the naive ledger or
worse the moment one wall bends — it does not degrade, it
*straightens*, unwinding curved walls into straight ones and
refusing moths on their authority. Before trusting a priced
cave, ask not only how many walls the judge was allowed to
count, and whether any of them was allowed to move, but whether
any of them was allowed to *accelerate*. And the rent is charged
in the order the unknowns are carried: strength first (read off
loudness, all noise can eat it), then curvature (carried by one
return's address), then drift (carried by two), then the verdict
itself — at σ 0.30 the search is still 25 trains off the floor
while naming the bend on barely a third of trains.

**Next seed:** the counting problem in a moving, bending cave —
a judge that must first decide how many walls the record holds
(one bending, two, or none) before it may price any of them,
with the linear judge's 122 standing as the price of guessing
the shape wrong; or a wall whose drift changes sign mid-flight,
whose trajectory is a V in the record and whose first half
prices like a different wall entirely (the seed twindrift also
left standing); or a third glint's worth of patience — trains of
four or five glints, where a curvature is carried by two returns
and a cubic term becomes nameable.

## Tests

12 unit tests: the clean book clears/stays under the line at
every phase; the unwalled book is error-free for the naive judge;
the curving + walking regime's arrival geometry (two walls
summing at offsets 9 and 15, to 1e-12); the curving + still
regime's geometry (two walls summing at offset 9, the curving
wall's delays 5/5/6, to 1e-12); the opposite-bends regime's
geometry (both third echoes summing at offset 12, an own glint
carrying an echo at offset 8, to 1e-12); the sparse score and
the abort score both equal the dense whole-record score on real
records, and a limit below the true score abandons rather than
misreports; every regime wounds naive and twindrift at σ 0.00
while the twincurve search recovers every regime on whole books
(errors 0, trajectory exact 240/240, ρ error max ≤ 0.011, and
the twindrift judge's pair exact 0); the twincurve search names
trajectories and strengths on individual trains at σ 0.00; the
derive step unwinds a hand-built curving + walking record
exactly; the book is paired and deterministic; the noise is
zero at σ 0, bounded by σ elsewhere, and channel-separated (the
two walls' echoes never share one noise draw).

Run them with `cargo test -p listener-twincurve`; run the audit
with `cargo run -p listener-twincurve -- --headless`.
