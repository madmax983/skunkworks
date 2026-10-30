# Cochlea Cave 🗺️

**Lineage:** Revival cross — `graveyard/cochlea-sim` (the dead) ×
`crates/resonance-audio` (the living donor).
**Built by:** The Cartographer of Lost Things 🗺️
**Status:** Planted 2026-10-29. Ledger below.
**Coordinates:** graveyard/cochlea-sim → crates/resonance-audio;
bearing: an ear that had only a generator, re-housed in a cave
that had only a field.

## The Death

`cochlea-sim` (524 LOC) was a complete ear: 32 channels, each a
band-pass filter (RBJ biquad, Q = 4) tuned log-spaced from 50 Hz
to 10 kHz, a hair cell (half-wave rectification + leaky smoothing
0.9), and a leaky integrate-and-fire neuron (threshold 0.5,
decay 0.95, ~1 ms refractory). It was executed for **Documentation
Void alone** — no README, and its `main` went straight to the TUI
with no headless bypass, so no ledger of its hearing ever existed.
The underlying metabolism was sound. What it lacked was a *place*
to hear: its only source was a synthetic kick/hi-hat/pad generator
wired straight into the filters — sound with no space in it, no
travel, no reflections, no decay.

## The Donation

`crates/resonance-audio` supplies the exact missing organ: a 2D
FDTD wave cave (the same field `keeper-moths` flies in) — a
56×36 grid where a driven source cell's pressure must cross
28 cells of air, reflect off the cave's bounds, and ring down
before and after it reaches a listening post. The revival puts
the dead ear at that post, (36, 18), and runs every sound twice:

- **direct** — the tone fed straight into the cochlea, the dead
  specimen's way. The control.
- **cave** — the same tone driven into the grid at (8, 18),
  sampled at the ear cell, and only then fed to the cochlea.

Neither parent does this alone: the corpse had a frequency code
and no space; the cave has space and no frequency code at all
(its residents read raw pressure and gradient).

## Honest Adaptations (named, not hidden)

- The corpse's clock was 44.1 kHz; the cave steps far slower.
  The revival runs at a simulated 8,000 steps/sec, rescales the
  basilar range to 50–3,000 Hz (inside the cave clock's Nyquist),
  and rescales the ~1 ms refractory from 44 samples to 8 steps.
- Filter Q, hair-cell smoothing, LIF threshold/decay, and the
  corpse's ×5 / ×5 gains are unchanged.
- Drive levels are set low (direct 0.08, cave pluck 0.05 per
  tone) on ledger evidence: at 0.3 the LIF neurons saturate at
  their refractory ceiling across a plateau of channels and the
  tonotopic peaks tie and wander (a 400 Hz tone's drive peak
  landed on channel 17, its total peak on 16). Below saturation
  the code is exact, as the ledger shows. This ear's tuning is
  level-dependent; that is a property of the corpse's neuron,
  measured here, not a defect concealed.

## Novel Trait

The first specimen in this garden to ask what a *propagating
medium* does to a *neural frequency code*: whether tonotopy
survives a reverberant space, what the space costs in onset
time, and how long the ear keeps reporting a sound the source
has already stopped making.

## Prediction and Falsification

Declared from the design, before the first clean run:

1. Tonotopy survives the cave: each tone's peak channel through
   the cave equals its direct peak (±1 channel).
2. The cave delays onset: first spike through the cave lands
   tens of steps after the direct first spike (28 cells at the
   grid's wave speed alone is ~44 steps).
3. The cave out-rings the ear: spikes after the source stops
   (the 2,000-step tail) are far more numerous through the cave
   than direct, where only filter ringing and hair-cell decay
   remain.
4. A two-tone chord keeps two humps through the cave — the
   medium mixes the tones in one pressure field, but the ear
   should still separate them by place.
5. Silence spikes nothing, in either route.

The claim is falsified if cave peaks wander off the direct
peaks (the space scrambles the code), if the tail is no longer
than the ear's own ringing (the "reverb" is just the filters),
or if the chord collapses to one hump (place separation does
not survive a shared field).

## Headless Ledger

*(Filled from the first full headless run, 2026-10-29; reproduce
with `cargo run -p cochlea-cave -- --headless`.)*

```
cochlea-cave headless: a dead ear, re-housed in a living cave
  ear: 32 channels, log-spaced 50-3000 Hz, Q=4.0, hair-cell smoothing 0.9,
    LIF threshold 0.5 decay 0.95 refractory 8 steps, sample rate 8000 steps/sec
  cave: 56x36 FDTD, source (8, 18), ear (36, 18) (28 cells apart),
    drive 4000 steps, tail 2000 steps
  direct 150 Hz:  drive 5064, tail 126, first spike 11, peak ch 8 (144 Hz), conc 0.217
  cave   150 Hz:  drive 1293, tail 608, first spike 99, peak ch 8 (144 Hz), conc 0.398
  direct 400 Hz:  drive 6388, tail 71,  first spike 8,  peak ch 16 (414 Hz), conc 0.191
  cave   400 Hz:  drive 3304, tail 913, first spike 86, peak ch 16 (414 Hz), conc 0.304
  direct 1000 Hz: drive 5680, tail 34,  first spike 9,  peak ch 23 (1043 Hz), conc 0.221
  cave   1000 Hz: drive 2347, tail 965, first spike 96, peak ch 23 (1043 Hz), conc 0.394
  direct chord 150+1000: drive 6413, tail 85,  humps 931 / 1116
  cave   chord 150+1000: drive 1724, tail 716, humps 314 / 665
  cave silence: total spikes 0
  onset delay (cave - direct): 150 Hz 88 steps, 400 Hz 78, 1000 Hz 87
  reverb tail (1000 Hz): direct 34, cave 965
```

All five declared lines held:

1. **Tonotopy survives exactly** — peak channels 8 / 16 / 23 in
   both routes, at the channels tuned 144 / 414 / 1043 Hz.
   The cave does not scramble the place code.
2. **Onset costs ~80–90 steps** (~11 ms at the cave clock):
   ~44 steps of propagation across 28 cells, the rest buildup
   of a weaker, reflected signal through the filters.
3. **The cave out-rings the ear by an order of magnitude** —
   tail spikes 608 / 913 / 965 through the cave against
   126 / 71 / 34 direct. After the source stops, the cave ear
   keeps firing for a substantial fraction of its drive total;
   the direct ear falls nearly silent. The reverb is the
   space's, not the filters'.
4. **The chord keeps two humps** in both routes (cave: 314
   spikes within ±1 of the 150 Hz peak, 665 within ±1 of the
   1000 Hz peak).
5. **Silence spikes 0.**

One finding not predicted, kept honestly: the cave *sharpens*
the code — concentration (drive spikes within ±1 channel of
the peak) rises from 0.19–0.22 direct to 0.30–0.40 through the
cave. The likely mechanism is level, not magic: the cave
attenuates, and this ear's tuning sharpens at lower level
(the saturation plateau above is the same effect, seen from
the other side). A level-matched direct control — same ear
amplitude, no space — is the honest next survey, and would
separate "the cave filters" from "the cave merely quiets."

## Honest Limits

- One ear position, one source position, open air between —
  no walls, lenses, or voids from the donor's material palette
  have been surveyed yet. A wall between source and ear is the
  obvious next bearing.
- The tail is counted in spikes, not seconds-to-silence; the
  cave's ring-down time constant is not fitted here.
- The chord is two well-separated tones (150 / 1000 Hz).
  Close tones that share filter skirts are untested.
