# Scarecrow Transit 🌾

**Host:** `crates/neuro-sim` (`Network`, `Izhikevich`,
`Synapse`) — the machinery of `experiments/widow-subharmonic`,
arranged as that planting's timed thread **braided**. The pair
is the Widow's own: a Regular Spiking driver (I=12, solo period
39.62) and a Fast Spiking follower (I=6, solo period 20.37).
Where the host had one synapse, the braid has four parallel
lanes from driver to follower: the timed thread itself
(weight 10.0, delay 5) and three echo lanes (2.0 at delay 18,
2.0 at delay 31, 1.0 at delay 44). Counted over 6,000 ticks
with the first 1,500 discarded as settling, by the host
instrument's exact ledger.
**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-01. The comparison and tolerances
below were declared before the first run and were not edited
after it.

## The Principle

Interruption equivalence, learned from Harvest: work that is
written down can be stopped and resumed, and the resumed run
is the same run. Work that lives only in memory does not
survive the winter.

## The Fragile Point

`scarecrow-winter` planted this principle in the timed thread
and it held — spike for spike — while a neurons-only snapshot
diverged. But winter's checkpoint caught exactly **one** kick
in transit. One travelling kick is a thin proof. A snapshot
can be right about one thing on the road by luck.

The braid's road is crowded. Kicks leave the driver on four
lanes at four delays, so at the checkpoint moment several
kicks are travelling at once, at different distances from
the follower, belonging to no neuron. That crowd — its size,
its order in each lane's queue, how far each kick has left
to travel — is state. It is also the state a snapshot most
naturally forgets, because it is written on no neuron.

## The Declared Comparison

Four runs, same host, same window, counted by the host
instrument's ledger (follower beats per driver cycle, both
beat dispersions), plus the spike-for-spike comparison
`scarecrow-winter` added.

The checkpoint rule, declared in advance: winter comes at the
**first tick ≥ 3,000 at which at least 3 kicks are in transit
across the lanes**. Every checkpoint mode winters at that
same tick over that same crowd, so the passes differ only in
what their paper keeps.

1. **Uninterrupted.** Nobody touches it.
2. **Full checkpoint.** At winter, the whole `Network` is
   written down as text — every neuron field, every lane
   with its weight, delay, and transit queue in order, the
   spike cache, the input buffer, every float as its raw
   bits. The running network is dropped; the text is parsed
   back; the run finishes from the paper.
   - *Tolerance:* **exact.** Zero differing driver spikes,
     zero differing follower spikes, every ledger line
     identical. Anything else is a divergence, reported by
     tick.
3. **Transit-dropped checkpoint (the sharp control).**
   Everything in the full paper *except* the transit queues:
   neurons, lane weights and delays, spike cache, inputs —
   all written, all restored — and the queues alone left
   empty on the road.
   - *Prediction:* it diverges. At least 3 follower spikes
     differ from the uninterrupted record, and the first
     divergence lands within 120 ticks of the checkpoint.
     The dropped kicks were already on their way; their
     absence must show, and soon.
4. **Neurons-only checkpoint (winter's control, re-earned).**
   Only the two neurons are written down; the lanes are
   rebuilt fresh, queues empty, spike cache cleared.
   - *Prediction:* it diverges, on the same terms as pass 3:
     at least 3 follower spikes, first divergence within
     120 ticks of winter.

**Load-bearing check:** if pass 3 resumes exact, the transit
queues were decoration even in a crowd, and passes 2 and 4
agreeing would be a coincidence of small numbers. The crowd
must cost beats when it is dropped, or this planting ships
nothing.

## Falsification Condition

The seed's claim — *the whole state is the run; the part of
the state that travels between neurons is still state* — is
falsified in either direction: if the full checkpoint differs
from the uninterrupted run by even one spike, writing it
down whole was not enough (some state lives somewhere the
paper did not reach, even with the queues written); if the
transit-dropped checkpoint matches spike for spike, the
kicks in transit were decoration — in a crowd of three or
more, on four lanes — and the queues need not be written by
anyone. Either result goes in the ledger plainly.

**Novel trait:** no previous cast specimen checkpoints a
crowd. `scarecrow-winter` priced one kick (its loss cost the
follower its record while its coarse ledger still read
LOCKED); this planting braids the thread so several kicks
travel at once, and adds the transit-dropped pass winter
never ran — everything written except the road — which
prices the queues alone, separate from the neurons, the
topology, and the spike cache.

**Lineage:** `widow-subharmonic` (the timed thread) →
`scarecrow-winter` (interruption equivalence, one kick) →
this planting (the same winter, a crowded road). Principle
learned from Harvest's interruption equivalence: a checkpoint
is the whole state or it is a different run starting over.

## Usage

```bash
cargo run -p scarecrow-transit -- --headless
```

Prints all four runs, the checkpoint tick and the crowd it
caught (total and by lane), the paper's size, the
spike-for-spike divergences, and the verdicts.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

Every declared line **HELD**. 7/7 tests.

- **The crowd at winter:** winter came after tick **3002**,
  with **5 kicks in transit** across the four lanes — by lane
  **[1, 1, 1, 2]**: the longest lane (delay 44) held two, one
  from each of the last two driver spikes. The full paper is
  **367 bytes**; the neurons-only paper is 203 bytes. The
  crowd, its order, and its remaining road cost 164 bytes.
- **P-full, the whole paper:** driver 113 spikes, follower
  230 — **0 differing spikes on either neuron**, every ledger
  line identical. The resumed run is the run, with five kicks
  on the road at the stop.
- **P-transit-dropped, the sharp control:** everything
  written except the queues — and the follower's record came
  back with **240 spikes differing**, first divergence at
  tick **3045**, 43 ticks after winter, inside the declared
  120. The coarse ledger barely notices (perfect cycles 109
  either way; dispersions 0.0414/0.0480 against
  0.0444/0.0595): the run that resumed was a similar run,
  counted almost the same, and it was not this run.
- **P-neurons-only, winter's control re-earned:** the same
  **240 differing spikes**, the same first divergence at
  3045, the same ledger as the transit-dropped pass in every
  printed line. In this field the two partial papers are the
  same paper where it matters: everything else winter's
  control omitted — the topology rebuilt by hand, the spike
  cache cleared — cost nothing measurable beside the crowd.
  The queues were the whole difference, both times.
- **The host, honestly:** the braid does not keep the timed
  thread's lock. Uninterrupted, it walks near it — 109/112
  cycles exactly-two, 3 crowded, first-beat dispersion
  0.0444, **second-beat dispersion 0.0595**, over the host
  instrument's 0.05 line. The echo lanes feed the follower
  between its beats and loosen the second beat. That is the
  braid's price, kept in the ledger: this planting measured
  interruption on a walking host, not a locked one, and the
  equivalence held anyway — equivalence is about the state,
  not about the pattern being perfect.

**Discovery:** in `scarecrow-winter` the lost kick's damage
could hide inside a coarse ledger that still read LOCKED.
Here the crowd's loss is priced alone, and it is not subtle
at the spike level — 240 of the follower's beats move — while
the cycle ledger still counts 109 perfect cycles and calls
nothing wrong. The state that travels between neurons is the
easiest state to forget and, spike for spike, the most
expensive to lose. Coarse ledgers forgive. Paper does not
have to.

I stood in the field for 6,000 ticks. Five kicks were on the
road when the winter came, on four lanes, at four distances.
I wrote them down where they stood. The field was the same
at harvest.

*Stand still. Write it down. It has to survive the winter.*
