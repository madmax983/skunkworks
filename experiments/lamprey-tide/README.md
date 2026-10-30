# Lamprey Tide 🗺️

**Lineage:** Revival cross — `graveyard/spinal-rhythms` (the dead) ×
`crates/physics-pbd` (the living body) and `crates/neuro-sim`
(the living cord).
**Built by:** The Cartographer of Lost Things 🗺️
**Status:** Planted 2026-10-30. Ledger below.
**Coordinates:** graveyard/spinal-rhythms → crates/physics-pbd +
crates/neuro-sim; bearing: a silent animal, given back its spine
and taught to keep accounts.

## The Death

`spinal-rhythms` (771 LOC) was a whole animal: a lamprey whose
spinal cord — a central pattern generator of Izhikevich neurons,
one left/right pair per segment, bound by cross-inhibition within
a segment (−25, delay 0) and descending excitation to the next
(+20, delay 2), driven at 15.0 — set the muscle lengths of a
Verlet ladder body, and the body swam in a macroquad window. It
was executed for **Documentation Void alone** — the archive calls
it a Silent Specimen. No README, no headless bypass, no test, no
ledger: nothing it ever did was ever counted, so no one can say
whether it swam well, badly, or in circles. The underlying
metabolism — a half-center cord laying a traveling wave on a
muscular body — was sound. What it lacked was organs the garden
maintains, and anyone writing anything down.

## The Donation

Two living crates supply the exact missing organs:

- `crates/neuro-sim` — the garden's verified Izhikevich network
  (the Widow's instrument: delayed synapses, a spike cache,
  counted beats) — replaces the corpse's private, untested copy
  of the same mathematics. The cord is rebuilt on it neuron for
  neuron: same pairs, same cross-inhibition, same descending
  thread, same drive (head three segments at 15.0, the rest at
  0.8×).
- `crates/physics-pbd` — the garden's maintained position-based
  body — replaces the corpse's private Verlet world. Its
  `Actuator` constraint is a muscle by design: a distance
  constraint whose rest length is driven from outside. The
  ladder returns as PBD particles: rungs and diagonals as
  distance constraints, the two side rails as actuators.

Neither parent does this alone: neuro-sim's networks have never
had a body to move (the Widow's clocks beat in the abstract),
physics-pbd's actuators have never been driven by a nervous
system, and the corpse's private copies of both are gone.

## Honest Adaptations (named, not hidden)

- **12 segments, not 20.** The wave is the specimen, not the
  length.
- **Contraction 0.28 of segment length, not the corpse's 0.20,
  and activation is spike-triggered and smoothed** (a spike sets
  the muscle fully on; it relaxes ×0.93 per tick) rather than
  read off the membrane voltage as the corpse read it. Tuning
  evidence, kept: in the corpse's kinematic world the voltage
  read set muscle lengths directly, but against a solver the
  same read produced a mean bend of 0.037 and an animal that
  barely moved (25 units in 10,000 ticks). The shipped organ
  bends at 0.135. The corpse's number was right for its body;
  this body is a different material.
- **The water is new.** PBD has no fluid. A resistive-force
  water is applied per station after each solver substep
  (2 substeps of 0.02 per cord tick): the component of a
  particle's velocity perpendicular to the local body axis is
  damped ×0.82 per substep, the parallel component ×0.995, on
  top of the solver's own ×0.95. Without that anisotropy an
  undulating body goes nowhere, in this garden as in a pond.
- Symmetry is broken by a small fixed stagger of rest voltages
  in place of the corpse's dice, so the ledger is deterministic.

## Novel Trait

No specimen in this garden has ever asked whether a rhythm
*swims*, or which property of the rhythm does the swimming.
This one runs the same animal three ways — **wave** (the
corpse's descending thread: activation travels head → tail),
**in-phase** (every muscle reads segment 0's activation: the
same rhythm, its spatial phase removed), **frozen** (the wave
cord beating, the muscles never driven) — and prices the
traveling wave in units of forward travel.

## Predictions (declared from the design, before the ledger)

1. The cord's wave travels: mean lag per segment is positive
   and the head→tail phase sums to a substantial fraction of
   a period — about one wavelength on the body.
2. The wave outswims the in-phase rhythm by at least half
   again, and beats it several-fold per unit of lateral
   thrash: the in-phase animal can churn (its activation is
   pulsatile, so its flap is not perfectly reciprocal), but
   it spends its motion going back and forth.
3. Frozen muscles displace ~nothing: a rhythm with no body
   in it is not locomotion.

## Headless Ledger

`cargo run -p lamprey-tide -- --headless` — 12,000 ticks per
regime, first 2,000 settling; deterministic, identical every run:

```
cord: 7587 spikes in window, head period 31.82 ticks,
  mean lag/segment 4.79 ticks, head->tail lag 20.29 ticks
  (0.64 of a period folded; the per-segment lags sum to
  52.7 ticks = 1.66 periods — a wavelength of ~6.6 segments,
  ~1.8 wavelengths along the 12-segment body)
wave:     forward 1403.3, |d| 1405.7, mean speed 6.09,
          mean bend 0.1346, lateral paths head 5996.5 /
          tail 4084.0 (tail/head 0.68)
in-phase: forward 665.0, |d| 665.2, mean speed 8.48,
          mean bend 0.1259, lateral paths head 25540.4 /
          tail 21675.9
frozen:   forward 0.0, |d| 0.0 — exactly nothing
verdict:  wave / in-phase = x2.11 forward;
          efficiency (forward per unit lateral head travel)
          wave 0.234 vs in-phase 0.026 — x9.0
```

All three predictions held. The wave carries the animal 1,403
units — 7.8 body lengths — while the same rhythm with its
spatial phase removed manages 665 and thrashes four times as
much to do it (its *mean speed* is higher: 8.48 against 6.09;
it is going somewhere back and forth). The frozen cord beats
7,587 spikes into a body that does not move a single unit.

## The Crack, Kept

The head is the loudest part of this animal. Lateral travel at
the head (5,996) exceeds the tail's (4,084) — a real lamprey's
undulation *grows* toward the tail, and its head rides quiet.
Here the corpse's own drive gradient (head segments driven
hardest, 15.0 against 12.0) meets a free front end, and the
wave's first push is partly spent sideways. The ledger that
would settle it — amplitude per station, head to tail — is the
named succession: quiet the head (a drive gradient that ramps
*up* toward the tail, or a stiffer neck) and re-price the wave.

## Falsification Condition

This revival is wrong if the spatial phase does no work: if the
in-phase regime — same cord, same muscles, same water, phase
broadcast flat — matches the wave's forward travel per unit of
thrash, then the traveling wave is decoration and the corpse's
descending thread was never the organ that swims. It is also
wrong if the frozen regime displaces materially: that would
indict the water or the solver, not credit the cord.

## Tests

5 unit tests (`cargo test -p lamprey-tide`): the half-centers
alternate (coincident spikes rare); the wave travels head to
tail (tail lag inside one period); the wave swims forward
(> 20 units, actual 1,403); frozen muscles do not swim
(< wave/4, actual 0); the wave outswims the in-phase rhythm
(> 1.5× forward, > 3× efficiency; actual 2.11× / 9.0×).
