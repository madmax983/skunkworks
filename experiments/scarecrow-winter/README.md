# Scarecrow Winter 🌾

**Host:** `experiments/widow-subharmonic`'s timed thread — the
one 2:1 lock in the garden: a Regular Spiking driver (I=12,
period 39.62) and a Fast Spiking follower (I=6, solo period
20.37), joined by one synapse of weight 10.0 with a 5-tick
delay, counted over 6,000 ticks with the first 1,500 discarded
as settling. Built on the parent crate `crates/neuro-sim`
(`Network`, `Izhikevich`).
**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-10-30. The comparison and tolerances
below were declared before the first run and were not edited
after it.

## The Principle

Interruption equivalence, learned from Harvest: work that is
journaled can be stopped, replayed, resumed — and the resumed
run is the same run. Work that lives only in memory is a field
nobody wrote down. It does not survive the winter.

## The Fragile Point

The Widow's pair keeps *all* of its state in memory:

- the two neurons' membrane potentials and recovery variables,
- the previous tick's spike cache (the synapse reads last
  tick's spikes to decide what enters the delay line),
- the kicks **already in transit** down the 5-tick delay line —
  the state nobody sees, because it belongs to no neuron.

The lock itself is a kept pattern that exists only as this
state, unfolding in time. Stop the process at any tick and the
pattern is gone — unless it was written down, *all* of it.

## The Declared Comparison

Three runs, same host, same window, counted by the host
instrument's exact ledger (exactly two follower beats in every
driver cycle, both beat dispersions under 0.05, or it is not a
lock), plus a spike-for-spike comparison the host never made.

The checkpoint rule, declared in advance: winter comes at the
**first tick ≥ 3,000 at which a kick is in transit**. A
checkpoint taken over an empty delay line would test nothing.

1. **Uninterrupted.** Nobody touches it.
2. **Full checkpoint.** At winter, the whole `Network` is
   written down as text — every neuron field, the synapse with
   its transit queue, the spike cache, the input buffer, every
   float as its raw bits. The running network is dropped. The
   text is parsed back and the run finishes from the paper.
   - *Tolerance:* **exact.** Zero differing driver spikes, zero
     differing follower spikes, every ledger line identical.
     Anything else is a divergence, reported by tick.
3. **Neurons-only checkpoint (the control).** The same stop,
   but only the two neurons are written down — the snapshot a
   person writes when they save "the state" and mean the
   obvious part. The restored pair is rebuilt fresh: empty
   delay line, cleared spike cache.
   - *Prediction:* it diverges. At least 3 follower spikes
     differ from the uninterrupted record, and the first
     divergence lands within 120 ticks of the checkpoint. The
     lost kick was already on its way; its absence must show.

## Falsification Condition

The seed's claim — *the whole state is the run; a partial
snapshot is a different run* — is falsified in either
direction: if the full checkpoint differs from the
uninterrupted run by even one spike, writing it down whole was
not enough (some state lives somewhere the paper did not
reach); if the neurons-only checkpoint matches spike for
spike, the transit queue and spike cache were decoration and
the neurons really were the whole field. Either result goes in
the ledger plainly.

## Usage

```bash
cargo run -p scarecrow-winter -- --headless
cargo test -p scarecrow-winter
```

## The Performance

The ledger from the first verified run:

Winter came after tick **3002** — the first tick ≥ 3,000 with a
kick in transit. Exactly **1 kick** was travelling. The full
paper is **265 bytes**; the neurons-only paper is 202 bytes.
The missing 63 bytes were the whole difference.

| Run | Driver | Follower | Cycles | Dispersions | Verdict |
|-----|--------|----------|--------|-------------|---------|
| Uninterrupted | 113 @ 39.62 | 227 @ 19.81 | 112/112 | 0.0140 / 0.0287 | LOCKED 2:1 |
| Full checkpoint | 113 @ 39.62 | 227 @ 19.81 | 112/112 | 0.0140 / 0.0287 | LOCKED 2:1 |
| Neurons-only | 113 @ 39.62 | 227 @ 19.80 | 112/112 | 0.0130 / 0.0277 | LOCKED 2:1 |

- **Full checkpoint — HELD, exactly.** 0 differing driver
  spikes, 0 differing follower spikes, no first divergence,
  every ledger line identical. The field written down whole
  survived the winter.
- **Neurons-only — diverged, as predicted, and further than
  predicted.** First divergence at tick **3026**, 24 ticks
  after the checkpoint. **226 follower spikes differ** from the
  uninterrupted record (driver: 0). The declared lines held
  (≥ 3 differing, first within 120 ticks) with room to spare.

The honest part, kept plainly: by the host instrument's own
ledger, the neurons-only run still *locks* — 112/112 cycles,
dispersions a hair *tighter* than the original's. If this
experiment had counted only the coarse ledger, it would have
reported that the partial snapshot was enough. Spike for
spike, it was a different run: the lost kick shifted the
follower's phase (first beat 0.183 → 0.185), the lock
re-formed around the shift, and nearly every follower beat
for the remaining 3,000 ticks landed on a different tick.

**Discovery:** a lock is a pattern, and a pattern can
re-form. A *run* is its history, and history does not
re-form — it is either written down or it is not. The 63
bytes nobody would bother to save (a queue of kicks in
transit, a one-tick-old spike cache) were the difference
between resuming the same winter and starting a similar one.

I stood in the field for 6,000 ticks, three times. The run
stopped at 3,002. The paper remembered. The field was the
same at harvest. The short paper remembered the plants and
forgot the weather already on its way — and the field that
grew back was a fine field, and not this one.

*Stand still. Write it down. It has to survive the winter.*
