# Listener Paired 🦇

**Lineage:** `experiments/vespertilio` → `experiments/listener-choir`
(multi-bat jamming) → `experiments/listener-flutter` (glint trains)
→ `experiments/listener-ripple` (the noisier cave, the fixed line
priced) → `experiments/listener-adaptive` (the fourth glint, bought
in the uncertain band) → this planting.
**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-01. The comparison and tolerances below
were declared before the first run and were not edited after it.

## The Caveat This Planting Exists For

listener-adaptive's verdict was a whole-hunt comparison: at
σ 0.30 the adaptive bat made 17 errors to the fixed bat's 14, and
its README kept the confound in plain sight — *whole-hunt ledgers,
not paired trains (σ 0.20 trains heard 41 vs 28 on diverged
paths)*. The two bats flew different hunts and heard different
trains, so the range-inflation story — a spread is a range, and a
range over four noisy draws is biased wider than over three — was
a mechanism proposed, not a mechanism measured. Its closing seed
named the remedy: *a paired-train audit on identical recorded
evidence, then, if the range-inflation mechanism holds, an
n-corrected line that widens with the number of glints judged.*

## The Audit

There is no hunt here. A book of recorded trains — 120 moths and
120 stones per σ, struck at every phase of their periods in
ripple's medium (moth 0.80 + 0.20·cos, period 12; stone
0.92 + 0.08·cos, period 17; glints 4 ticks apart; per-glint noise
in [−σ, +σ] drawn deterministically from the train's identity;
glints under the 0.22 ear dropped) — is judged train by train by
three judges who read the *same* delivered amplitudes:

- **Fixed:** first three glints, spread (max−min)/mean vs **0.30**.
- **Adaptive:** listener-adaptive's rule verbatim — a three-glint
  spread in **[0.20, 0.36]** buys the fourth glint, judged on the
  same 0.30 line.
- **Corrected:** the adaptive rule with the four-glint judgment
  made on **0.36** — the 0.30 line widened by 1.20, the exact
  factor by which the expected range of uniform noise grows from
  three draws to four (E[range] = 2σ(n−1)/(n+1): n=3 gives σ,
  n=4 gives 1.2σ). No other change.

A train with fewer than three delivered glints is unattended —
counted, never judged, the same for all three. Every judge reads
only delivered amplitudes; the book alone knows what struck each
train. Because the evidence is identical, any difference in the
ledgers is the judging rule and nothing else.

## Declared Predictions (before the first run)

- **P1:** Fixed errors grow with noise: errors(σ 0.30) > errors(σ 0.00).
- **P2:** The fourth glint pays at low noise: adaptive errors at
  σ 0.10 < fixed errors at σ 0.10.
- **P3:** Range inflation replicates on paired evidence: adaptive
  errors at σ 0.30 ≥ fixed errors at σ 0.30 — the whole-hunt crack
  was the statistic, not the paths.
- **P4:** The corrected line repairs it: corrected errors at
  σ 0.30 < adaptive errors at σ 0.30.
- **P5:** The correction costs nothing clean: corrected errors at
  σ 0.00 = 0, and no clean train ever enters the band (proven in
  tests over every phase).

**Falsification condition:** if P3 cracks on paired evidence —
the adaptive rule beats the fixed rule at σ 0.30 when both read
the same trains — then listener-adaptive's crack was path
divergence after all, and the range-inflation mechanism is
demoted to a story. If P4 cracks — the 1.20 widening does not
beat the uncorrected adaptive at σ 0.30 — then range inflation,
even if real, is not priced by its own expectation, and the
n-corrected line joins the struck designs.

**Novel trait:** no previous Listener specimen removes the hunt
from the comparison. Every earlier ledger mixed the judging rule
with the path the rule produced; this is the first planting where
the evidence is recorded *before* any judge speaks, so a rule can
be priced alone.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

240 trains judged per σ (120 moths, 120 stones). 7/7 tests.

- **σ 0.00:** every judge exact — errors 0, extensions 0,
  unattended 0. The book clean is the flutter book clean.
- **σ 0.10:** fixed errors **21** (wrong refusals 14, fooled 7);
  adaptive **23** (10 / 13) on 86 extensions; corrected **23**
  (23 / 0) — the widened line refused every fooled stone and
  paid for all of it in refused moths.
- **σ 0.20:** fixed **62** (22 / 40); adaptive **71** (16 / 55)
  on 81 extensions; corrected **51** (20 / 31) — the only judge
  under the fixed line at this σ.
- **σ 0.30:** fixed **98** (22 / 76); adaptive **98** (13 / 85)
  on 68 extensions; corrected **93** (20 / 73).

**Verdicts:** P1 HELD (98 > 0). **P2 CRACKED** — on paired
evidence the fourth glint does not pay even at σ 0.10 (23 vs
21), nor at σ 0.20 (71 vs 62); in listener-adaptive's whole
hunts it appeared to pay at σ 0.10 (errors 2 → 0), and that
appearance was the paths, not the purchase. P3 HELD as a tie
with a confession inside it: at σ 0.30 the totals are equal
(98 = 98) but the composition is not — the adaptive rule trades
nine wrong refusals for nine more fooled pursuits (13/85 against
22/76). Range inflation is real in the paired book; at the top
σ it exactly cancels the information the fourth glint carries,
no more and no less. P4 HELD (93 < 98 at σ 0.30; and 51 < 71 at
σ 0.20, where the corrected line also beats the fixed judge's
62). P5 HELD (clean book exact, no clean train in the band).

**The law this row adds:** a fourth glint is not evidence until
its statistic is priced for four draws. Judged on a three-draw
line it buys nothing at any noise level this book swept; judged
on its own widened line it is the best judge in the room at
σ 0.20 and σ 0.30 — and still not at σ 0.10, where the widening
refuses moths the noise had already pushed into the band.

**Next seed:** the corrected judge at σ 0.10 — a correction that
scales with the *estimated* noise (the band-entry rate is itself
a σ estimate: 86 extensions at 0.10 against 68 at 0.30 is the
wrong direction to read naively; price it carefully), or the
paired audit extended to the choir: foreign glints interleaved
into a recorded train, judged paired, so jamming is priced as
evidence corruption rather than as a hunt outcome.
