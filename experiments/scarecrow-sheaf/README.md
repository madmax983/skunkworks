# Scarecrow Sheaf 🌾

**Host:** `experiments/widow-heptad-peak` (The Clockwork Widow 🕰️) —
its champion cell, surveyed whole in `widow-heptad-plane` and at
fine grain in `widow-heptad-peak`: a near-7:5 pair on the parent
crate `crates/neuro-sim` — driver Regular Spiking (I=12), follower
Fast Spiking (I=4.8), one synapse, weight -1.0, delay 10, counted
over 24,000 ticks (first 1,500 discarded) by the Widow's own
instrument (super-cycle = five driver cycles; perfect = exactly
seven follower beats, each within ±0.06 of its median phase). The
host's machinery, constants, and instrument are copied faithfully;
a test pins this crate's live cell to the host's published centre
(perfect 92/112, count slips 3, phase slips 17, longest 58, mean
episode 18.4).

**Principle:** Journal and replay — Harvest's rule that work which
is written down can be stopped, stored, and replayed, and the
replay is the same work. Learned from Harvest's durable execution:
the journal is the run; everything else is derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-07. The comparison and predictions
below were declared before the first run and were not edited
after it. Verdicts are at the bottom, as found.

## The fragile point

The pair is two clocks and one road between them. The driver
receives nothing — it cannot be influenced, only heard. The
follower receives exactly one thing: the synapse's kicks, each
-1.0, each in transit for the synapse's delay. Everything the
follower will ever know about the driver crosses that one road,
one kick at a time.

So the coupling fits in a journal. Not the neurons' insides, not
the field — there is no field. Just the ticks the kicks landed.
Nothing in the host writes them down: stop the run and the road's
whole history is gone, though it was only ever a list of ticks.

One arithmetic question rides along, and this planting measures
it instead of assuming it: the parent crate pushes a spike into
transit on the step *after* the spike and counts the delay down
in that same pass, so a delay-10 kick should land on step
spike + 11. The first delivery of the season says whether the
road is as long as its sign claims.

## The declared comparison

Four runs of the follower, counted by the host's instrument,
plus a spike-train comparison over all 24,000 ticks:

- **Live** — the host pair, whole, writing two journals as it
  steps: driver spike ticks, and delivery ticks read off the
  synapse's own `active` flag.
- **Replay (deliveries)** — the follower alone (one neuron, no
  synapse, no driver), its constant current plus a kick of -1.0
  on each journaled landing tick.
- **Replay (spikes)** — the follower alone, fed landings
  re-derived from the spike journal as spike + delay + 1.
- **Replay (immediate)** — the spike journal with the delay
  forgotten: kicks land on the spike tick itself.
- **Replay (coarse-5)** — the delivery journal with every
  landing rounded to the nearest 5 ticks: a journal written
  from memory, by a keeper who remembers the shape of the road
  but not its exact stones.

Paper sizes are printed in the ledger, counted honestly: one
whole checkpoint of this pair is 56 bytes (two neurons × six
f32, plus flags; the transit queue never holds more than one
kick, because the driver's period (~39.6) exceeds the road's
length (11)). A season's journal is 607 events × 2 bytes =
1,214 bytes, for either journal.

**Predictions (declared before the first run):**

- **B1 (host check):** the live cell reproduces the host's
  published centre exactly — perfect 92/112, count slips 3,
  phase slips 17, longest 58, mean episode 18.4 (±0.05).
- **P1:** the delivery journal replays the follower *exact* —
  spike train identical on all 24,000 ticks, cell ledger field
  for field.
- **P2:** the spike journal replays exact too, once the road's
  arithmetic is paid: landings = spike + delay + 1, neither
  +delay nor +delay + 2.
- **P3:** the immediate journal diverges — a first follower
  divergence exists, and its cell differs from the live cell
  (perfect ≠ 92 or longest ≠ 58). A journal that drops the
  delay is a different coupling, not a shorter spelling.
- **P4:** the coarse journal diverges too, yet keeps the
  EPISODIC verdict — rounding the stones costs exactness
  before it costs the season's character.
- **P5:** the journal is *not* the small paper here: the spike
  journal (≥ 10× the 56-byte checkpoint) loses the size
  comparison outright. The sheaf's claim is not cheapness;
  it is that the replay never opens the follower.

**Falsification condition:** if P1 fails — if the landing ticks
alone, replayed into a lone follower, part from the live train
at any tick — then something besides the kicks crosses that
road (a shared state, a hidden current), the journal is not
the coupling, and this seed was a decoration. If P3 fails and
the immediate journal also replays exact, the delay carries
nothing and the road's length is a rumour.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

- **B1 HELD, exact.** Live: perfect 92/112, count slips 3,
  phase slips 17, episodes 5, longest 58, mean episode 18.4,
  EPISODIC — the host's centre, field for field.
- **P1 HELD, exact.** 607 driver spikes, 607 deliveries, 850
  follower spikes. The lone follower, fed only the landing
  ticks, spikes on the live run's ticks at every one of the
  24,000 — first divergence: none. The journal is the coupling,
  whole.
- **P2 HELD, exact.** The first delivery lands at tick 14, the
  first spike at tick 3: measured lag 11 = delay + 1, and the
  derived landings equal the journaled landings at all 607
  events; that replay is also exact. The road is as long as
  its sign claims, plus the one step the crate spends noticing
  the spike.
- **P3 HELD.** The immediate journal parts at tick 36 — the
  season's third kick, landed 11 ticks early, moves a follower
  spike — and its cell reads perfect 84/112, count slips 6,
  phase slips 22, longest 37, mean 14.0. Same kicks, same
  weights, wrong road: the delay is not bookkeeping, it is
  half the coupling.
- **P4 HELD — with a wink the prediction did not declare.**
  The coarse journal parts at tick 64 and its cell reads
  perfect 90/112, count slips 7, phase slips 15, longest
  **59** — one *above* the true season's 58. A journal written
  from memory can outscore the truth on a single column while
  being wrong from tick 64. Verdict EPISODIC, as declared;
  but let the ledger say plainly: longest-episode alone
  would have crowned the misremembered season.
- **P5 HELD.** Spike journal 1,214 B against the pair's whole
  checkpoint at 56 B — the journal is 21× the paper it
  replaces, and loses. For a system this small, with a state
  this cheap, writing the *inputs* down is the expensive
  option; writing the *state* down is nearly free. The sheaf
  earns its keep only where the follower may not be opened —
  or where the season outlasts any single checkpoint's right
  to be trusted. Size was never its claim, and now that is
  measured, not apologised for.

**The law, eleventh under the Scarecrow's name:** before
journaling a coupling, count what actually crosses it. If
nothing crosses but a list of ticks, the list is the whole
story and the replay is exact — but price the list against
the state it replaces, because for a small system the story
costs more than the thing itself, and a story retold from
memory can beat the truth on any single column while parting
from it at tick 64.

**Next seed:** the same sheaf for a pair whose road carries
two kinds of kick (an excitatory and an inhibitory synapse on
one follower) — does the journal stay a list of ticks when
each entry must also carry its sign, and does the replay stay
exact when two roads land on the same tick?
