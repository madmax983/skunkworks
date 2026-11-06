# Listener Twindrift 🦇

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
→ this planting.

**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-06. The predictions below were declared
before the first run and were not edited after it.

## The Seed This Planting Exists For

listener-twinwalled closed with a named seed, sharpened from the
one listener-drifting left standing: *two walls at once with
either of them in motion — does the joint (pair × drift) search
still separate two trajectories, or do two moving walls finally
price each other?* The motion half of drifting's seed has stood
in this garden's seed list since the drifting run. This planting
takes it: every wall is a trajectory — a base delay, a drift of
−1, 0, or +1 ticks per glint, and a strength — and a judge that
would price the cave must name two trajectories at once.

## The Audit

A book of recorded trains — 120 moths and 120 stones per regime,
struck at every phase of their periods in ripple's medium (moth
0.80 + 0.20·cos, period 12; stone 0.92 + 0.08·cos, period 17;
per-contribution noise in [−σ, +σ] drawn deterministically from
the train's identity and the contribution's channel — own, near
wall, far wall each carry their own draw; arrivals under the
0.22 ear dropped) — is recorded behind three pairs of walls, the
near wall at **ρ = 0.55**, the far wall at **ρ = 0.40**. A wall's
echo of glint *j* arrives `base + drift·j` ticks after that glint:

- **still + walking** — near (4, 0), far (8, +1): the near wall
  coincides (echoes at 4, 8, 12); the far wall's echoes walk away
  (8, 13, 18). Offset 8 sums three contributions.
- **counter-walking** — near (4, +1), far (9, −1): near echoes at
  4, 9, 14; far echoes at 9, 12, 15. Offset 9 sums two echoes
  from two different walls, neither of them an own glint.
- **both walking, same way** — near (3, −1), far (8, −1): echoes
  at 3, 6, 9 and 8, 11, 14; both walls walk toward the strike.

The record window runs to offset 20 so both walls' full returns
are delivered. Three judges read the *same* delivered record —
amplitudes and arrival offsets, nothing else:

- **Naive:** the first three arrivals, spread (max−min)/mean vs
  the **0.30** line — flutter's judge.
- **Twin:** listener-twinwalled's search verbatim in shape — a
  joint search over an ordered pair of distinct *still* delays
  in 1..=12 and two strengths on a 0.05 grid. It can count to
  two. Neither of its walls may move. It is never told any wall
  moves.
- **Twindrift:** a joint search over an ordered pair of
  trajectories — base in 3..=9, drift in {−1, 0, +1}, every
  per-glint delay inside 1..=12 — and two strengths on the same
  0.05 grid (the true strengths sit on that grid). Each candidate
  derives its own glints (subtracting, in strike order, any
  earlier glint's echo from *either* trajectory that lands
  exactly on a later own offset), predicts the whole record, and
  is scored by the summed absolute difference between prediction
  and delivery over every offset either mentions (computed
  sparsely over the ≤ 9 predicted offsets; a test asserts the
  sparse score equals the dense whole-record score). The true
  trajectories and strengths are the book's ground truth, never
  given to the judge.

**Falsification condition:** if the twindrift search errs on any
train in the clean cave, or names a wrong trajectory pair on any
clean train, the whole-record score does *not* separate two
trajectories — some moving wall hides inside the other's price —
and this planting's premise is false. If the still-pair twin
judge recovers any regime exactly at σ 0.00, motion was never
load-bearing in that regime and the seed's premise cracks there.

**Novel trait:** every previous Listener planting priced at most
one moving thing: drifting gave *one* wall a velocity, twinwalled
gave a judge *two* addresses. This is the first planting where
the unknowns multiply — two trajectories, six unknowns (two
bases, two drifts, two strengths) — and where an echo can be
summed not with an own glint but with another wall's echo
(offset 9 of the counter-walking regime), so neither wall's
return can be read off the record alone.

## Declared Predictions (before the first run)

- **P1:** Two walls wound the naive judge in every regime: at
  σ 0.00 its errors are at least 20 in each regime.
- **P2:** The twin search is a still-pair judge: at σ 0.00 it
  errs at least 20 times in each regime, including the regime
  where only one wall walks.
- **P3:** The twindrift search prices both trajectories in a
  clean cave: at σ 0.00 its errors are 0 in every regime, its
  trajectory-pair estimate is exact on every train, and its ρ
  error max (either wall) is ≤ 0.011.
- **P4:** Pooled over the three regimes at σ 0.10, the
  twindrift judge's errors are below the naive judge's.
- **P5:** At σ 0.10 the twindrift judge still names both
  trajectories: its pair estimate is exact on at least 90% of
  trains pooled.
- **P6:** At σ 0.30, pooled over the three regimes, the
  twindrift judge's errors are still below the naive judge's
  pooled errors — the noise floor does not fully swallow the
  search.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

240 trains judged per regime, 12 regimes (3 regimes × 4 noise
levels). 11/11 tests. Sanity first: the unwalled control
reproduces the lineage's clean book line for line — naive
errors **0** at σ 0.00, **21** at σ 0.10, **62** at σ 0.20,
**98** at σ 0.30 — in every regime, because the clean book does
not depend on the walls.

- **σ 0.00:** naive **120 / 140 / 120** by regime (still+walking
  / counter-walking / both-walking). Twin **150 / 140 / 128**.
  Twindrift **0 errors in every regime**, trajectory pair exact
  **240/240** in each, ρ error mean and max **0.000**. Six
  unknowns — two bases, two drifts, two strengths — separated
  exactly by the whole-record score. Two moving walls do not
  price each other in a clean cave.
- **The still-pair judge in a moving cave:** at still+walking
  the twin judge is *worse than naive by 30* (150 vs 120, wrong
  30, fooled 120): it unwinds two still walls out of a record
  one walking wall helped write, and the unwinding manufactures
  30 wrong refusals of moths the naive judge never refused —
  twinwalled's one-wall failure, one dimension up. At
  counter-walking it posts the naive ledger *exactly* (140,
  wrong 20, fooled 120). At both-walking it errs 128, better
  than naive only because two wrong still walls can absorb more
  of a walking record than one. Knowing the count (two) without
  knowing the motion buys nothing clean.
- **σ 0.10:** twindrift pooled **133** vs naive pooled **374**;
  trajectory exact **706/720** pooled (238 / 238 / 230 by
  regime). By regime the twindrift judge errs **65 / 35 / 33** —
  the still+walking regime is the expensive one at this σ, and
  its ρ error mean is **0.138** against 0.057 / 0.051 in the
  walking regimes: where the walking wall's first return rides
  the triple-summed offset 8, its strength is the noisy part —
  twinwalled's coincident tax, charged to a trajectory.
- **σ 0.20:** twindrift **104 / 80 / 80** by regime (pooled
  264 vs naive 366); trajectory exact 196 / 198 / 162 of 240.
  ρ error max reaches **0.550** — some trains' strength
  estimates collapse to the grid's floor — while the motions
  are still named four times in five.
- **σ 0.30:** twindrift pooled **319** vs naive pooled **359**,
  against a pooled clean floor of 294 — the six-unknown search
  is 25 trains off the noise floor even here. Trajectory exact
  falls to 153 / 167 / 128 of 240: at this σ some records have
  lost an echo under the ear, and a record with a missing
  arrival can wear a wrong trajectory while keeping a nearly
  right verdict (drifting's lesson, doubled).

**Verdicts:** every declared line HELD. P1 (120 / 140 / 120,
all ≥ 20). P2 (twin 150 / 140 / 128, all ≥ 20 — the still-pair
judge fails even where only one wall walks). P3 (twindrift 0
everywhere in a clean cave, trajectory exact 240/240 in each
regime, ρ error max 0.000). P4 (133 < 374). P5 (706/720, not
merely 90%). P6 (319 < 359).

**The law this row adds:** a pair of trajectories separates the
same way a pair of addresses did — exactly, in a clean cave —
because each wall writes at offsets the other cannot reach,
*in time as well as in place*: a drift is a schedule of
addresses, and two schedules that differ anywhere write
somewhere the other cannot follow. What motion breaks is not
the joint search but every judge one dimension short of the
record: the still-pair judge, which twinwalled proved exact
against two still walls, posts the naive ledger or worse the
moment one wall walks — it does not degrade, it *confabulates*,
unwinding still walls that are not there and refusing moths on
their authority. Before trusting a priced cave, ask not only
how many walls the judge was allowed to count, but whether
any of them was allowed to move. And the rent, as before, is
charged at coincidence first: the regime whose walking wall
begins inside a triple-summed offset pays the highest ρ error
and the highest verdict error at σ 0.10, long after its
trajectory is still being named nearly every time — motion is
read off *where* arrivals land, which noise cannot move, and
strength off *how loud* they land, which is all noise can eat.

**Next seed:** a wall that accelerates — drift itself drifting
within the flight, now with a second wall in the room, priced
by a search that must give a wall a curvature; or the counting
problem in a moving cave — a judge that must first decide how
many walls the record holds (one moving, two, or none) before
it may price any of them, with the twin judge's 150 standing as
the price of guessing the motion wrong; or a wall whose drift
changes sign mid-flight, whose trajectory is a V in the record
and whose first half prices like a different wall entirely.

## Tests

11 unit tests: the clean book clears/stays under the line at
every phase; the unwalled book is error-free for the naive judge;
the counter-walking regime's arrival geometry (two walls summing
at offset 9, to 1e-12); the still+walking regime's geometry
(three contributions summed at offset 8, the far wall walking to
13 and 18, to 1e-12); the sparse score equals the dense
whole-record score on real records; every regime wounds naive
and twin at σ 0.00 while the twindrift search recovers every
regime on whole books (errors 0, trajectory exact 240/240, ρ
error max ≤ 0.011); the twindrift search names trajectories and
strengths on individual trains at σ 0.00; the derive step
unwinds a hand-built counter-walking record exactly; the book
is paired and deterministic; the noise is zero at σ 0, bounded
by σ elsewhere, and channel-separated (the two walls' echoes
never share one noise draw).

Run them with `cargo test -p listener-twindrift`; run the audit
with `cargo run -p listener-twindrift -- --headless`.
