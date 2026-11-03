# Listener Echoed 🦇

**Lineage:** `experiments/vespertilio` → `experiments/listener-choir`
(multi-bat jamming) → `experiments/listener-flutter` (glint trains)
→ `experiments/listener-ripple` (the noisier cave) →
`experiments/listener-adaptive` (the fourth glint) →
`experiments/listener-paired` (the paired audit) →
`experiments/listener-jammed` (interleaved foreign glints, the
timing gate) → this planting.
**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-02. The predictions below were declared
before the first run and were not edited after it.

## The Seed This Planting Exists For

listener-jammed closed with a named seed: *foreign glints arriving
ON the own offsets — timing cannot separate those, and the gate's
perfection ends; what channel prices them then?* In jammed, the
foreign glints landed between the train's own (offsets 2, 6, 10
against own 0, 4, 8), and a judge that refused every arrival not
due on its own chirp's hour recovered the clean book exactly.
This planting closes that door and asks what is left.

## The Audit

A book of recorded trains — 120 moths and 120 stones per regime,
struck at every phase of their periods in ripple's medium (moth
0.80 + 0.20·cos, period 12; stone 0.92 + 0.08·cos, period 17;
per-contribution noise in [−σ, +σ] drawn deterministically from
the train's identity; arrivals under the 0.22 ear dropped) — is
recorded in a cave with a flat back wall behind every reflector.
The wall's extra round trip is exactly one glint interval
(4 ticks), and every return re-arrives once more at strength
**ρ = 0.55**. So the echo of glint *j* lands exactly on glint
*j+1*'s offset, and the two amplitudes sum in the ear. The record
holds four arrivals — offsets **0, 4, 8, 12**: three summed glints
and one orphan, the last glint's echo, arriving after the judged
window closes with nothing summed into it.

Four judges read the *same* delivered record — amplitudes and
arrival offsets, nothing else:

- **Naive:** the first three arrivals, spread (max−min)/mean vs
  the **0.30** line — flutter's judge.
- **Gated:** only arrivals at the train's own expected offsets —
  jammed's winner. Here every arrival sits at an own offset, so
  its evidence is the naive judge's, sample for sample.
- **Tail:** un-echo the train. The orphan at offset 12 is
  ρ·own₂ and nothing else, so a search over ρ in [0.00, 0.95] at
  0.01 steps that best predicts the orphan from the summed train
  prices the wall from the one arrival the wall could not hide
  inside another; the train is then unwound recursively
  (own₀ = d₀; ownⱼ = dⱼ − ρ·ownⱼ₋₁) and judged clean. The judge is
  never told the wall's true ρ — that is the book's ground truth,
  kept for scoring the estimate only.
- **Restrike:** chirp again at glint spacing **5** instead of 4.
  The wall's delay belongs to the cave, not the chirp, so the
  echoes now land at 4, 9, 14 — between the new own offsets
  0, 5, 10 — and a gated judge on the second train hears its own
  glints alone. The price is a second train, and a spacing that
  samples the moth's wingbeat at phases the 0.30 line was never
  tuned on.

**Falsification condition:** if the gated judge's ledger differs
from the naive judge's by even one verdict on echoed evidence,
timing *can* separate coincident arrivals and this planting's
premise is false. If the tail judge fails to recover the clean
book at σ 0.00, the orphan is not the wall's price tag and the
un-echoing channel is a story, not a measurement.

**Novel trait:** every previous Listener corruption arrived as an
extra arrival the record could point at — a foreign glint between
one's own, a glint dropped by noise. This is the first corruption
that arrives *as* one's own evidence: no extra arrival inside the
window, nothing to refuse, the sums themselves lying. The two
channels that price it are both new to the lineage — algebra
(the orphan, read late and alone) and geometry (a second chirp
whose spacing moves the coincidence).

## Declared Predictions (before the first run)

- **P1:** Timing cannot separate coincident arrivals: gated
  errors equal naive errors at every σ on echoed evidence.
- **P2:** The summed step fools stones wholesale: at σ 0.00 the
  naive judge's fooled count is at least 100 of 120 stone trains
  (the echo adds a rising step to a nearly flat train, and a step
  is a spread).
- **P3:** The orphan prices the wall exactly in a clean cave: at
  σ 0.00 the tail judge's errors are 0 and its ρ estimate is
  within one grid step (0.01) of the wall's true ρ on every train.
- **P4:** The tail is one sample, and recursion amplifies its
  noise: at σ 0.10 the tail judge's errors exceed the clean
  book's naive errors at the same σ.
- **P5:** The re-strike escapes the wall but not the line: at
  σ 0.00 its errors are above 0 and far below the naive judge's,
  and none of them are fooled stones (spacing 5 samples some moth
  phases so flatly that the line reads them steady — the wall is
  beaten, the tuning is not).
- **P6:** At σ 0.30 the noise floor swallows every channel: the
  tail and re-strike judges' errors each land within 10 of the
  clean book's naive errors.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

240 trains judged per regime, 960 arrivals per regime at low σ.
12/12 tests. Sanity first: the unwalled control reproduces the
lineage's clean book line for line — naive errors **0** at
σ 0.00, **21** at σ 0.10 (wrong 14, fooled 7), **62** at σ 0.20,
**98** at σ 0.30.

- **σ 0.00:** naive **140** (wrong 20, fooled **120** — every
  stone in the book fooled), gated **140**, identical verdict for
  verdict. Tail **0**, ρ error mean 0.000, max 0.000 — the orphan
  named the wall to the grid's precision on all 240 trains.
  Restrike **20** (wrong 20, fooled 0).
- **σ 0.10:** naive/gated **135**; tail **52** (ρ error mean
  0.115, max 0.400 — one noisy orphan, recursed three deep);
  restrike **36**, against the clean book's 21.
- **σ 0.20:** naive/gated **123**; tail **89**; restrike **78**,
  against the clean book's 62.
- **σ 0.30:** naive/gated **118**; tail **100**; restrike **99**,
  against the clean book's 98 — every channel on the floor.

**Verdicts:** every declared line HELD. P1 (140/135/123/118 =
140/135/123/118 — the gate is not wounded here, it is *absent*:
there is nothing in its evidence to refuse). P2 (120 of 120
stones fooled, not merely 100). P3 (0 errors, ρ exact). P4
(52 > 21 — un-echoing is the best channel in a quiet cave and a
mediocre one in a merely ordinary cave, because its whole price
rides on a single late sample). P5 (20 errors, all wrong
refusals, no stone fooled — the 20 are the moth phases the
spacing-5 sampling flattens, the same flattening the wall itself
works at phase 0, where a peak-struck moth's falling flank is
filled in by its own echo). P6 (100 and 99 against the clean
98).

**The law this row adds:** a corruption that hides *between*
arrivals can be refused; a corruption that hides *inside* them
can only be *priced* — and the price is always paid in a second
currency. The tail judge pays in fragility: one orphan sample,
exactly sufficient in silence, whose noise the recursion spends
three times. The re-strike pays in tuning: a new spacing is a
new instrument, and the old line does not transfer for free —
20 moths are the transfer tax at spacing 5. And the naive
ledger's shape is worth keeping: as σ rises the echoed judge's
errors *fall* (140 → 118), because noise breaks up the wall's
tidy step as often as it breaks a train — the wall is a
low-noise disease. In a loud enough cave, even a liar this
punctual cannot keep its story straight.

**Next seed:** a wall whose delay is *not* one glint interval —
ρ and delay both unknown, priced from a record whose orphan
may land inside the window on a later glint or beyond it
entirely; or the tail judge given the re-strike's second train
as a second orphan (two late samples, one wall: does the ρ
estimate's noise fall by the expected half, and does P4 heal?);
or listener-jammed's other open door — the gate whose timing is
learned, not given, now with a wall in the room as well.
