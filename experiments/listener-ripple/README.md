# Listener Ripple 🦇

**Lineage:** `experiments/vespertilio` (The Night Listener 🦇) by way of
`experiments/listener-choir` and `experiments/listener-flutter` — the
flutter bat's sense, carried unchanged into the noisier cave its own
README demanded.
**Built by:** The Night Listener 🦇
**Status:** Planted 2026-10-30. Ledger below.

## The Strike

listener-flutter proved a bat can tell moth from stone by listening to
a whole glint train instead of believing its first glint — and closed
by confessing why the proof was too easy:

> *The separation is perfect because the model is clean: a stone's
> train is exactly flat and a moth's swing is deep (multiplier
> 0.24–1.00). A shallower wingbeat, a rippled stone, or noise on the
> amplitudes would put trains on both sides of the 0.30 line, and the
> ledger would have errors to count instead of zeros. That noisier
> cave is the honest next strike.*

This is that cave. Three corruptions, all in the medium, none in the bat:

- **A shallower wingbeat.** The moth multiplier is
  `0.80 + 0.20·cos(phase + 2π·t/12)` — flutter's was
  `0.62 + 0.38·cos(...)`. Three samples a third of a wingbeat apart
  still always span at least half the swing, so a clean shallow
  train's spread is at least ~0.375: above the line, by 0.075,
  not by flutter's ~0.45.
- **Rippled stones.** A stone's multiplier is
  `0.92 + 0.08·cos(stone_phase + 2π·t/17)` — a slow ripple, period
  17 ticks, each stone on its own phase. A clean rippled train's
  spread reaches ~0.17: below the line, but no longer zero. Its
  margin is 0.13 — wider than the moth's 0.075.
- **Amplitude noise.** Every delivered glint is multiplied by
  `1 + u`, `u` drawn deterministically (SplitMix64 on the train's
  identity) from `[−σ, +σ]`. A glint whose modulated amplitude falls
  under the ear threshold (0.22) never arrives: trains can arrive
  **partial**.

The bat is flutter's bat, unchanged — same cave (140×90, c = 2),
same seed, same 8 moths and 12 stones, same hold-still-and-listen
rule, and the **same flutter line, 0.30**, never retuned. The refusal
to move the line is the experiment: one fixed sense swept across
five noise levels, σ = 0.00, 0.05, 0.10, 0.20, 0.30, 3,600 ticks
each, and a ledger that counts what the noise costs.

The perception rule is sacred, as always: the bat reads only the
amplitudes the medium actually delivered — never a clean value,
never a reflector's kind. Whether a verdict was right is scored
by the world, never by the bat.

## Novel Trait

No previous specimen in this garden corrupts its own channel and
then sweeps the corruption. vespertilio proved a return can steer;
the choir proved a shared medium can lie about *whose* return it
is; flutter proved one return, heard completely, knows what made
it. Ripple asks the dose–response question the other three could
not: **at what noise level does a proven sense start lying, which
lie does it tell first, and what does the lying cost in flights
and in food?**

## Prediction and Falsification

Declared from the design margins, before the first full run:

1. At σ = 0 the thin margins still separate: errors 0, stone
   visits 0, and the ledger matches flutter's flutter regime
   (the same bat in a cave whose trains still clear the line).
2. Errors appear as σ grows, and they appear **asymmetrically**:
   the moth's margin (0.075) is thinner than the stone's (0.13),
   so *wrong refusals* (a moth read as stone) should appear at a
   lower σ than *fooled flutters* (a stone read as a moth).
3. By σ = 0.30 the ledger has errors of both kinds to count, and
   fooled flutters cost real flights (stone pursuits and stone
   visits above zero).
4. Partial trains occur once modulated amplitudes dip under the
   ear — some attended trains resolve on fewer than 3 glints.

The claim — *the 0.30 line is a sense with a noise budget, not a
law* — is falsified if errors stay at zero through σ = 0.30 (the
line was never in danger and the cave is still too clean), or if
errors appear at σ = 0 (the shallow/rippled margins were already
on the wrong side and there was never a budget to spend).

## Headless Ledger

*(Filled from the first full headless run, 2026-10-30; reproduce
with `cargo run -p listener-ripple -- --headless`.)*

```
sigma 0.00: chirps 71, trains 35, glints 105, dropped 5, partial 0,
  pursuits 15, stone pursuits 0, stone visits 0,
  refusals 12 (correct 12, wrong 0), fooled 0, errors 0, catches 7
sigma 0.05: chirps 71, trains 35, glints 105, dropped 4, partial 0,
  pursuits 15, stone pursuits 0, stone visits 0,
  refusals 12 (correct 12, wrong 0), fooled 0, errors 0, catches 7
sigma 0.10: chirps 70, trains 39, glints 116, dropped 6, partial 1,
  pursuits 12, stone pursuits 0, stone visits 0,
  refusals 16 (correct 14, wrong 2), fooled 0, errors 2, catches 7
sigma 0.20: chirps 69, trains 41, glints 123, dropped 8, partial 0,
  pursuits 10, stone pursuits 3, stone visits 3,
  refusals 18 (correct 16, wrong 2), fooled 3, errors 5, catches 8
sigma 0.30: chirps 72, trains 36, glints 107, dropped 9, partial 0,
  pursuits 14, stone pursuits 10, stone visits 2,
  refusals 12 (correct 8, wrong 4), fooled 10, errors 14, catches 12
errors — sigma 0.00: 0, sigma 0.30: 14
```

All four declared lines held:

1. **The clean cave reproduces flutter exactly** — 71 chirps, 35
   trains, 15 pursuits, 12 correct refusals, 0 stone visits,
   7 catches: flutter's own ledger, line for line. The shallow
   wing and the rippled stone cost nothing while the air is quiet.
2. **The asymmetry arrived as predicted.** Wrong refusals appear
   first, at σ = 0.10 (2 moths read as stone, none fooled);
   fooled flutters wait until σ = 0.20 (3). The thinner margin
   broke first. The line does not fail all at once — it fails
   moths first, because moths had less room.
3. **By σ = 0.30 the sense is lying regularly**: 14 errors in
   one hunt — 10 stones judged fluttering (10 stone pursuits,
   2 full stone visits) and 4 moths refused at range. Correct
   refusals fell 12 → 8: the refusals that remain are increasingly
   the wrong ones' neighbours.
4. **Partial trains are real but rare**: exactly one attended
   train resolved on a partial train, at σ = 0.10. Dropped glints
   grow with σ (5 → 9 — the σ = 0 count is the modulation itself
   dipping under the ear, which flutter never modelled because it
   judged audibility on the unmodulated amplitude), but most drops
   land in trains the bat never attends.

## Honest Limits

- **Catches do not read as a dose–response, and are not reported
  as one.** Catches *rose* with noise (7 → 12 at σ = 0.30) — not
  because noise helps, but because the first differing verdict
  puts the bat on a different trajectory, and one bat's 3,600
  ticks are a single chaotic path, not an average. The honest
  cost lines are the errors, the stone flights, and the wrong
  refusals (moths passed up, uncounted as lost catches). A fleet
  of bats per σ would average the path away; that is a choir
  question, and the choir has its own troubles in shared air.
- Noise is drawn per glint from the train's identity — independent
  across glints by construction. Real cave noise (a rival's chirp,
  rain on the roof) would correlate across a train and push whole
  trains across the line together; the errors here are the
  *independent-noise floor*, not the worst case.
- The bat never adapts: it holds the 0.30 line at every σ because
  the experiment is the line's budget, not the bat's learning.
  Whether a bat that *estimated* σ from its own trains and moved
  its line (or demanded a fourth glint) could buy the budget back
  is the live succession — an adaptive line, judged against this
  fixed line's ledger at σ = 0.20 and 0.30.

## Tests

6/6: echo arrival timing; clean shallow/rippled separation over
every phase and strike tick in both cycles (moth always flutter,
stone always steady at σ = 0); noise determinism (same train,
same noise) and breakability at σ = 0.30 in both directions over
2,000 trains; classification needs two glints; the clean cave
hunts clean (errors 0, stone visits 0, catches > 0); noise costs
errors in the world (σ = 0.30 errors > σ = 0.00 errors, bat
still eats).
