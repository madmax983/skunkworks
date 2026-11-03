# Listener Adrift 🦇

**Lineage:** `experiments/vespertilio` → `experiments/listener-choir`
(multi-bat jamming) → `experiments/listener-flutter` (glint trains)
→ `experiments/listener-ripple` (the noisier cave) →
`experiments/listener-adaptive` (the fourth glint) →
`experiments/listener-paired` (the paired audit) →
`experiments/listener-jammed` (interleaved foreign glints, the
timing gate) → `experiments/listener-echoed` (the coincident wall,
the orphan tail) → this planting.
**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-03. The predictions below were declared
before the first run and were not edited after it.

## The Seed This Planting Exists For

listener-echoed closed with a named seed: *a wall whose delay is
not one glint interval — ρ and delay both unknown, priced from a
record whose orphan may land inside the window on a later glint
or beyond it entirely.* In echoed, the wall stood at exactly one
glint interval (delay 4): every echo landed on the next own glint,
and the last glint's echo arrived alone at offset 12 as an orphan
that priced ρ exactly. That judge knew the wall's address and
only had to read the price tag. This planting moves the wall.

## The Audit

A book of recorded trains — 120 moths and 120 stones per regime,
struck at every phase of their periods in ripple's medium (moth
0.80 + 0.20·cos, period 12; stone 0.92 + 0.08·cos, period 17;
per-contribution noise in [−σ, +σ] drawn deterministically from
the train's identity; arrivals under the 0.22 ear dropped) — is
recorded behind three walls, all at **ρ = 0.55**, at delays
**3, 4, and 8 ticks**. The record window is extended to offset
20 so every wall's full return is delivered:

- **Delay 3:** echoes at 3, 7, 11 — interleaved just *before*
  each following own glint; no arrival at offset 12 at all.
- **Delay 4:** echoes at 4, 8, 12 — coincidence on the next own
  glint, the echoed case, orphan at 12.
- **Delay 8:** echoes at 8, 12, 16 — the first echo coincides
  with the *last* own glint, and two orphans arrive after the
  judged window closes.

Three judges read the *same* delivered record — amplitudes and
arrival offsets, nothing else:

- **Naive:** the first three arrivals, spread (max−min)/mean vs
  the **0.30** line — flutter's judge.
- **Legacy tail:** listener-echoed's judge verbatim. It assumes
  delay 4, searches ρ from an assumed orphan at offset 12, and
  unwinds recursively. It is never told the wall moved.
- **Search:** a joint search over delay in 1..=12 ticks and ρ in
  [0.00, 0.95] at 0.01 steps. For each candidate it derives the
  own glints the candidate implies (correcting the own offsets
  for coincidence when the candidate delay is 4 or 8), predicts
  the whole delivered record that candidate would produce, and
  scores it by the summed absolute difference between prediction
  and delivery over every offset either mentions. The cheapest
  candidate wins; its derived own glints are judged on the same
  0.30 line. The judge is never told the wall's true delay or
  ρ — those are the book's ground truth, kept for scoring its
  estimates only.

**Falsification condition:** if the legacy tail judge recovers
the clean book at a delay other than 4, knowing the wall's
address was never load-bearing and this planting's premise is
false. If the search judge fails to recover the clean book at
σ 0.00 at any of the three delays, a whole-record joint search
does not price an unpinned wall, and the channel is a story,
not a measurement.

**Novel trait:** every previous Listener wall had a fixed,
known address — corruption could be priced because the price
tag's location was given. This is the first planting where the
wall's *address itself* is part of what must be heard: the
orphan may be interleaved, coincident, absent from its assumed
offset, or beyond the window, and the judge that survives is
the one that prices delay and strength together against the
whole record instead of reading one late arrival.

## Declared Predictions (before the first run)

- **P1:** The wall wounds the naive judge at every address: at
  σ 0.00 its errors are at least 20 at each of delays 3, 4, 8.
- **P2:** The legacy tail is a one-address judge: at σ 0.00 it
  recovers the book exactly at delay 4 (errors 0), judges no
  train at all at delay 3 (no arrival at its assumed orphan
  offset 12), and errs at least 20 times at delay 8.
- **P3:** The search prices both unknowns in a clean cave: at
  σ 0.00 its errors are 0 at every delay, its delay estimate
  is exact on every train, and its ρ error max is ≤ 0.011.
- **P4:** Pooled over the three walls at σ 0.10, the search
  judge's errors are below the naive judge's.
- **P5:** At σ 0.10 the search still names the wall's address:
  its delay estimate is exact on at least 90% of trains pooled.
- **P6:** At σ 0.30, pooled over the three walls, the search
  judge's errors are still below the naive judge's pooled
  errors — the noise floor does not fully swallow the joint
  search.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

240 trains judged per regime, 12 regimes (3 walls × 4 noise
levels). 12/12 tests. Sanity first: the unwalled control
reproduces the lineage's clean book line for line — naive
errors **0** at σ 0.00, **21** at σ 0.10, **62** at σ 0.20,
**98** at σ 0.30 — at every wall, because the clean book does
not depend on the wall.

- **σ 0.00:** naive **120** at delay 3 (wrong 0, fooled 120 —
  every stone, and no moth refused: the interleaved echo is a
  step, and a step only ever reads as swing), **140** at delay
  4 (echoed's ledger exactly, wrong 20, fooled 120), **140**
  at delay 8 (wrong 20, fooled 120). Legacy tail **0 errors at
  delay 4 on 240 judged**, **judged 0 at delay 3** (no arrival
  ever sits at its assumed orphan offset — its silence there
  is not accuracy), **120 errors at delay 8** (it reads the
  delay-8 orphan at offset 12, which is the echo of own₁, as
  if it were the echo of own₂, and prices the wall wrong on
  every train). Search **0 errors at every delay**, delay
  exact **240/240** at each, ρ error mean and max **0.000**.
- **σ 0.10:** search pooled **109** vs naive pooled **391**.
  By wall: delay 3 search **21** — exactly the clean book's
  errors at this σ, because once the address is known the
  interleaved echoes are simply not own arrivals and the own
  glints were never summed into; delay 4 search **52**,
  identical to the legacy tail's 52 (at its own address the
  joint search and the orphan search are the same instrument);
  delay 8 search **36** against the legacy tail's **125**.
  Delay exact pooled **720/720**; ρ error mean 0.045 / 0.115 /
  0.067 by wall — the coincident wall (delay 4) still prices
  its strength worst, because its ρ rides on summed samples.
- **σ 0.20:** search **62 / 90 / 91** by wall (again exactly
  clean at delay 3), delay exact 239 / 232 / 232 of 240.
- **σ 0.30:** search pooled **309** vs naive pooled **359**,
  against a pooled clean floor of 294 — the joint search is
  nearly on the noise floor even here. Delay exact falls to
  229 / 199 / 196 of 240: at this σ roughly one train in six
  has lost an echo under the ear (arrivals 1,369 / 930 / 1,146
  against the clean-wall counts 1,440 / 960 / 1,200), and a
  record with a missing arrival can wear a wrong address.

**Verdicts:** every declared line HELD. P1 (120 / 140 / 140,
all ≥ 20). P2 (legacy 0 at delay 4, judged 0 at delay 3, 120
at delay 8 — the one-address judge in full). P3 (search 0
everywhere in a clean cave, both unknowns exact). P4 (109 <
391). P5 (720/720, not merely 90%). P6 (309 < 359).

**The law this row adds:** a wall's damage is priced by its
address, not its strength. The same ρ = 0.55 wall is three
different diseases at three delays: interleaved (delay 3) it
is *fully* removable once named — the search judge's ledger
there is the clean book's ledger at every σ, because nothing
was ever summed; coincident (delay 4) it is removable only
through a strength estimate, and that estimate is the noisy
part at every σ; orphaned-forward (delay 8) it sits between.
And a judge built for one address does not degrade gracefully
at another: the legacy tail does not get *worse* as the wall
moves, it gets *confidently wrong* at delay 8 and *mute* at
delay 3. Before trusting a priced wall, ask which arrivals the
pricing assumed would be where — the address is a prior, and
an unexamined prior is a second wall.

**Next seed:** a wall whose delay *drifts* within a train's
flight — each glint's echo at a different delay, as from a
moving reflector or a sloped wall — priced by a search that
must give the wall a velocity, not an address; or two walls at
once (two delays, two strengths, the record a sum of three
trains): does the joint search's whole-record score still
separate the pair, or do two unpinned walls price each other?
Or listener-jammed's remaining open door — the gate whose
timing is learned, not given, now in a room whose wall has
also moved.
