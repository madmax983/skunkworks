# Listener Adaptive 🦇

**Lineage:** `experiments/vespertilio` (The Night Listener 🦇) by way of
`experiments/listener-choir`, `experiments/listener-flutter`, and
`experiments/listener-ripple` — ripple's fixed sense, given the one
freedom its predecessor named: adapt.
**Built by:** The Night Listener 🦇
**Status:** Planted 2026-10-31. Ledger below.

## The Strike

listener-ripple swept one fixed flutter line (0.30) across a noisy
cave and counted the cost: errors 0 at σ = 0.00, the first wrong
refusals at σ = 0.10, 14 errors at σ = 0.30. Its closing seed asked
for *the adaptive line: a bat that estimates σ from its own trains
and moves its line (or demands a fourth glint), judged against this
fixed line's ledger at σ = 0.20 and 0.30.*

This bat takes the parenthetical. It does not move the line — it
buys more evidence, but only when the evidence it has is ambiguous.

- The medium is ripple's medium, unchanged: shallow wingbeat
  (`0.80 + 0.20·cos`, period 12; clean spread ≥ ~0.375), rippled
  stones (`0.92 + 0.08·cos`, period 17; clean spread ≤ ~0.17),
  per-glint noise in `[−σ, +σ]` drawn deterministically
  (SplitMix64 on the train's identity), glints under the ear
  (0.22) dropped. One change only: the medium now sends a
  **fourth glint**, 4 ticks after the third, on every train.
- The **fixed bat** (control) hears the first three glints and
  judges on the 0.30 line exactly as ripple did; the fourth glint
  arrives after its verdict and is ignored. Its ledger must
  reproduce ripple's, σ for σ — and does (see the tests).
- The **adaptive bat** hears the same three glints. If their
  spread ratio `(max − min)/mean` falls in the **uncertain band
  [0.20, 0.36]** it holds still for the fourth glint and judges
  all four amplitudes on the same 0.30 line. Outside the band it
  judges at once, paying nothing.

The band is placed where no clean train can enter it — proven in
tests over every phase and strike tick: clean shallow moths read
above 0.36, clean rippled stones below 0.20. So in a quiet cave
the adaptive bat never asks, and its ledger is the fixed bat's.

The price is counted: every extension costs 4 ticks of held-still
listening in which the bat does not chirp, scan, or pursue.

The perception rule is sacred, as always: both bats read only the
amplitudes the medium actually delivered — never a clean value,
never a reflector's kind, never the true σ. Whether a verdict was
right is scored by the world, never by the bat.

## Novel Trait

No previous specimen in this garden lets the listener *spend time
to reduce its own uncertainty on a single return*. Flutter spent a
fixed 8 ticks on every train; ripple spent the same and took the
errors. This bat spends conditionally — a fourth glint is bought
only for trains whose first three glints land near the line — and
the ledger prices the purchase in extensions, extra listen ticks,
errors, and catches, against the fixed bat hunting the same cave
on the same seed.

## Prediction and Falsification

Declared from the design margins, before the first full run:

1. At σ = 0 the adaptive bat is the fixed bat: extensions 0,
   errors 0, catches equal — the band is never entered clean.
2. Extensions rise with σ: noise pushes spreads into the band.
3. At σ = 0.20 and σ = 0.30 the adaptive bat makes **fewer**
   classification errors than the fixed bat — the fourth glint
   resolves the trains the noise made ambiguous.

The claim — *a conditionally bought fourth glint is worth its
4 ticks* — is falsified if the adaptive bat's errors at high σ
are not lower than the fixed bat's. Partial outcomes are reported
as partial: holding at σ = 0.10 alone does not save the claim at
σ = 0.20 and 0.30, the levels ripple's seed named as the judges.

## Headless Ledger

*(Filled from the first full headless run, 2026-10-31; reproduce
with `cargo run -p listener-adaptive -- --headless`. 3,600 ticks
per bat per σ; fixed = ripple's bat, adaptive = fourth glint on
band trains only.)*

```
sigma 0.00: fixed errors 0, catches 7 | adaptive errors 0, catches 7, extensions 0, four-glint judgments 0
sigma 0.10: fixed errors 2 (wrong 2, fooled 0), catches 7 | adaptive errors 0, catches 6, extensions 7, four-glint judgments 7
sigma 0.20: fixed errors 5 (wrong 2, fooled 3), catches 8 | adaptive errors 9 (wrong 3, fooled 6), catches 5, extensions 7, four-glint judgments 7
sigma 0.30: fixed errors 14 (wrong 4, fooled 10), catches 12 | adaptive errors 17 (wrong 4, fooled 13), catches 8, extensions 12, four-glint judgments 12
extra listen ticks paid by the adaptive bat: 0 / 28 / 28 / 48 across the sweep
```

Lines 1 and 2 **HELD**, exactly. Line 3 **CRACKED** — and the
crack has a shape worth keeping:

- **At σ = 0.10 the purchase works.** Seven trains landed in the
  band; the fourth glint resolved every one the fixed bat's two
  errors came from — adaptive errors 0 against the fixed bat's 2.
  It cost a catch (6 vs 7): the held-still ticks are real.
- **At σ = 0.20 and 0.30 the purchase backfires.** Adaptive errors
  rose (5 → 9, 14 → 17) and catches fell (8 → 5, 12 → 8). The
  growth is almost entirely in *fooled flutters* (3 → 6, 10 → 13):
  stone trains judged fluttering. The mechanism is in the
  statistic, not the noise: **a spread is a range, and a range
  over four noisy draws is biased wider than a range over
  three.** The fourth glint adds information about the reflector
  and, at high σ, adds even more range-inflation — pushing
  ambiguous stones over a line that was calibrated for three
  glints. A line honest for *n* samples is not honest for
  *n + 1*; this bat kept the three-glint line and paid for it.
- Honest caveat, as in ripple: the ledgers are whole hunts, not
  paired trains. The first differing verdict puts each bat on a
  different chaotic path (at σ = 0.20 the bats heard 41 vs 28
  trains), so error counts compare hunts, not identical evidence.
  A paired-train audit — the same recorded trains judged both
  ways, offline — is the named succession, and would separate
  the range-inflation mechanism from path luck.

Succession: judge each recorded train both ways on identical
evidence (paired audit), and if range inflation is the true
mechanism, an *n-corrected* line — a threshold that widens with
the number of glints judged — should recover σ = 0.10's win at
σ = 0.20 without surrendering it at 0.30. Or: spend the fourth
glint differently — not on the spread, but on a *fresh* three-
glint train re-struck at a new angle.

## Tests

7/7: echo arrival geometry; clean trains never enter the
uncertain band (over every phase and strike tick, both kinds);
noise determinism; classify needing two glints and the band
judging exactly three; **the fixed bat reproducing ripple's clean
ledger line for line** (71 chirps, 35 trains, 12 correct refusals,
0 errors, 7 catches); the adaptive bat paying nothing in the clean
cave; noise making the adaptive bat ask (extensions > 0 and
four-glint judgments > 0 at σ = 0.30, fixed extensions 0).
