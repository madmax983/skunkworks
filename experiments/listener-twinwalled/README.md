# Listener Twinwalled 🦇

**Lineage:** `experiments/vespertilio` → `experiments/listener-choir`
(multi-bat jamming) → `experiments/listener-flutter` (glint trains)
→ `experiments/listener-ripple` (the noisier cave) →
`experiments/listener-adaptive` (the fourth glint) →
`experiments/listener-paired` (the paired audit) →
`experiments/listener-jammed` (interleaved foreign glints, the
timing gate) → `experiments/listener-echoed` (the coincident wall,
the orphan tail) → `experiments/listener-adrift` (the wall at an
unknown address) → `experiments/listener-drifting` (the wall in
motion) → this planting.

**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-05. The predictions below were declared
before the first run and were not edited after it.

## The Seed This Planting Exists For

listener-drifting closed with a named seed: *two walls at once,
either of them possibly in motion — does the joint search's
whole-record score still separate a pair of trajectories, or do
two walls price each other?* adrift's own remaining seed named
the still half of the same question. This planting takes the
still half: two walls, both standing, neither address nor either
strength known — and asks whether a search that must name *four*
unknowns at once still separates them, and what a judge that can
only name one wall does in a cave that holds two.

## The Audit

A book of recorded trains — 120 moths and 120 stones per regime,
struck at every phase of their periods in ripple's medium (moth
0.80 + 0.20·cos, period 12; stone 0.92 + 0.08·cos, period 17;
per-contribution noise in [−σ, +σ] drawn deterministically from
the train's identity and the contribution's channel — own, near
wall, far wall each carry their own draw; arrivals under the
0.22 ear dropped) — is recorded behind three pairs of walls, the
near wall at **ρ = 0.55**, the far wall at **ρ = 0.40**:

- **(3, 8)** — the near wall interleaves (echoes at 3, 7, 11);
  the far wall's first echo lands exactly on the third own glint
  (offset 8 = own₂ + 0.40·own₀).
- **(4, 9)** — the near wall coincides (echoes at 4, 8, 12, each
  riding the next own glint or orphaned at 12); the far wall
  interleaves late (echoes at 9, 13, 17).
- **(4, 8)** — both walls coincide: offset 8 carries *three*
  contributions summed in one arrival (own₂ + 0.55·own₁ +
  0.40·own₀), and offset 12 sums two orphans (0.55·own₂ +
  0.40·own₁). Only five arrivals reach the ear at all.

The record window runs to offset 20 so both walls' full returns
are delivered. Three judges read the *same* delivered record —
amplitudes and arrival offsets, nothing else:

- **Naive:** the first three arrivals, spread (max−min)/mean vs
  the 0.30 line every Listener since flutter has used.
- **Single:** listener-adrift's search verbatim — a joint search
  over ONE fixed delay in 1..=12 and ρ on a 0.01 grid, each
  candidate deriving its implied own glints and scored by the
  summed absolute prediction-vs-delivery difference over every
  offset either record mentions. It can name one wall. It is
  never told there are two.
- **Twin:** a joint search over an ordered pair of distinct
  delays in 1..=12 and two strengths on a 0.05 grid (the true
  strengths sit on that grid). Each candidate derives the own
  glints it implies — subtracting, in strike order, any earlier
  glint's echo from *either* wall that lands exactly on a later
  own offset — predicts the whole record, and is scored the
  same way. The true pair and strengths are the book's ground
  truth, never given to the judge.

## Declared Predictions (before the first run)

- **P1:** Two walls wound the naive judge at every pair: at
  σ 0.00 its errors are at least 20 at each surveyed pair.
- **P2:** The single search is a one-wall judge: at σ 0.00 it
  errs at least 20 times at each surveyed pair.
- **P3:** The twin search prices both walls in a clean cave: at
  σ 0.00 its errors are 0 at every pair, its delay-pair estimate
  is exact on every train, and its ρ error max (either wall) is
  ≤ 0.011.
- **P4:** Pooled over the three pairs at σ 0.10, the twin
  judge's errors are below the naive judge's.
- **P5:** At σ 0.10 the twin judge still names both walls: its
  delay-pair estimate is exact on at least 90% of trains pooled.
- **P6:** At σ 0.30, pooled over the three pairs, the twin
  judge's errors are still below the naive judge's pooled
  errors — the noise floor does not fully swallow the search.

**Falsification condition:** if the twin search errs on any
train in the clean cave, or names a wrong pair on any clean
train, the whole-record score does *not* separate two walls —
some second wall hides inside the first's price — and this
README's claim is false in exactly the way the first wrong
train names.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

240 trains judged per regime, 12 regimes (3 pairs × 4 noise
levels). 10/10 tests. Sanity first: the unwalled control
reproduces the lineage's clean book line for line — naive
errors **0** at σ 0.00, **21** at σ 0.10, **62** at σ 0.20,
**98** at σ 0.30 — at every pair, because the clean book does
not depend on the walls.

- **σ 0.00:** naive **120** at pair (3, 8) (wrong 0, fooled
  120 — every stone), **140** at (4, 9) (wrong 20, fooled 120 —
  the near wall alone is listener-echoed's wall, and its ledger
  is echoed's exactly), **120** at (4, 8) (wrong 0, fooled 120).
  Single **128 / 140 / 190**. Twin **0 errors at every pair**,
  pair exact **240/240** at each, ρ error mean and max **0.000**.
  The joint score separates two walls exactly — four unknowns,
  no residue.
- **The one-wall judge in a two-wall cave:** at pair (4, 9) the
  single judge posts the naive ledger *exactly* (140, wrong 20,
  fooled 120) — the one wall it can name is the one whose damage
  the naive judge already pays in full, and naming it buys
  nothing. At pair (4, 8) it is **worse than naive by 70**
  (190 vs 120): it unwinds one wall out of a record two walls
  wrote, and the unwinding manufactures 70 wrong refusals of
  moths the naive judge never refused. drifting found the same
  failure in a still-wall judge facing a moving wall; here the
  missing dimension is not velocity but *count*.
- **σ 0.10:** twin pooled **181** vs naive pooled **378**;
  pair exact **699/720** pooled (233 / 238 / 228 by pair).
  By pair the twin errs 34 / 51 / 96 — the coincident pair
  (4, 8) is the expensive one at every noise level: where both
  walls hide inside the own train, there are no orphan arrivals
  to read the strengths off, and ρ error mean there reaches
  0.150 against 0.050 at the separated pair (3, 8).
- **σ 0.20:** twin 83 / 91 / 104 by pair; pair exact
  186 / 194 / 192 of 240. ρ error max reaches **0.550** —
  some trains' strength estimates collapse to the grid's floor
  — while the addresses are still named nearly every time.
- **σ 0.30:** twin pooled **343** vs naive pooled **354**,
  against a pooled clean floor of 294 — P6 held, but by 11
  trains, the thinnest margin in this lineage's pooled lines.
  And per pair it does *not* hold at (4, 8): twin **127** vs
  naive **115** there — at the coincident pair, at the noise
  floor, the four-unknown search pays more for its freedom
  than the corruption costs the judge that ignores it. At
  (3, 8) the twin and single ledgers coincide exactly at this
  σ (109, wrong 18, fooled 91, both) — kept as measured; no
  mechanism is claimed beyond the equality.

**Verdicts:** every declared line HELD. P1 (120 / 140 / 120,
all ≥ 20). P2 (128 / 140 / 190, all ≥ 20). P3 (twin 0
everywhere in a clean cave, pair exact 240/240 at each pair,
ρ error max 0.000). P4 (181 < 378). P5 (699/720, not merely
90%). P6 (343 < 354 — held pooled, cracked per-pair at (4, 8),
kept above).

**The law this row adds:** two walls do not price each other —
in a clean cave the whole-record score separates a pair of
walls exactly, addresses and strengths together, because each
wall writes at offsets the other cannot reach and the record
keeps both handwritings. What two walls break is the *one-wall
judge*: it does not degrade gracefully from its single-wall
ledger, it posts the naive ledger where its one wall is the
naive judge's whole disease, and far worse than naive where
both walls coincide — a judge that can name only one wall will
unwind that wall out of trains the other wall also wrote, and
call the residue a verdict. Before trusting a priced wall, ask
not only where it stands and whether it moves, but *how many
walls the judge was allowed to count* — an uncounted wall is
not a small error in the price; it is a second author in the
evidence. And the margin matters: the twin search's pooled
victory thins with noise (197 trains at σ 0.10, 11 at σ 0.30)
and inverts per-pair where the walls coincide — separation is
exact in the clean cave and *rented* in the noisy one, and the
rent is charged at the coincident address first.

**Next seed:** the seed drifting left standing, now sharpened —
two walls at once *with either of them in motion*: does the
joint (pair × drift) search still separate two trajectories,
or do two moving walls finally price each other? Or a wall
that accelerates — drift itself drifting within the flight,
priced by a search that must give the wall a curvature. Or the
twin search given a *counting* problem instead of a pair: a
judge that must first decide how many walls the record holds
(one, two, or none) before it may price any of them — the
single judge's 190 stands as the price of guessing the count
wrong by one.

## Tests

10 unit tests: the clean book clears/stays under the line at
every phase; the unwalled book is error-free for the naive
judge; the separated pair's arrival geometry (interleave plus
the far wall's coincidence at offset 8, to 1e-12); the
coincident pair's geometry (three contributions summed at
offset 8, two orphans summed at offset 12, to 1e-12); every
pair wounds naive and single at σ 0.00 while the twin search
recovers every pair on whole books (errors 0, pair exact
240/240, ρ error max ≤ 0.011); the twin search names pair and
strengths on individual trains at σ 0.00; the derive step
unwinds a hand-built triple-summed record exactly; the book is
paired and deterministic; the noise is zero at σ 0, bounded by
σ elsewhere, and channel-separated (the two walls' echoes never
share one noise draw).

Run them with `cargo test -p listener-twinwalled`; run the
audit with `cargo run -p listener-twinwalled -- --headless`.
