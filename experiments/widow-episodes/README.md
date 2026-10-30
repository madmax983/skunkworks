# Widow Episodes 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-10-30. Headless ledger below.

## Lineage

- **`neuro-sim`** provides two genuinely different clocks in one
  crate: the Regular Spiking neuron and the Fast Spiking neuron.
  Same equations, different recovery constants — under constant
  current each keeps its own period, and neither period is written
  down anywhere in the parent. The Widow measured them.
- **`widow-subharmonic`** (2:1) and **`widow-island`** (its lock,
  surveyed whole) settled the even ratio. Both set the odd one
  aside: a Fast Spiking follower near **3:2**, probed during the
  island run and put down honestly — gentle inhibition appeared to
  lock it at a 6,000-tick window, but over 12,000–24,000 ticks
  every swept coupling produced only lock *episodes*. This
  instrument is that probe, finished: if the pair will not hold,
  the ledger will say for how long it almost holds.

## Concept

Two neurons, joined by exactly one synapse, whose weight *and*
delay are the only things the Widow is allowed to turn:

- **The driver** — Regular Spiking, constant current I=12. Solo
  measured period: **39.60 ticks** (568 spikes in the window).
- **The follower** — Fast Spiking, constant current I=5. Solo
  measured period: **26.22 ticks** (858 spikes). Three follower
  beats span **78.67** against two driver cycles' **79.20** —
  ratio near 3:2, and the follower runs **early by 0.53 ticks per
  super-cycle**. Early is the whole story: the medicine for this
  pair is *inhibition* (a synapse that delays), and excitation —
  the only medicine the 2:1 pair ever needed — is kept here as a
  control, where it should make things worse.

A **super-cycle** is one pair of consecutive driver cycles. It is
**perfect** when it holds exactly three follower beats AND each
beat's phase on the super-cycle's ring lies within ±0.06 of that
beat's median phase for the regime. Count alone would let a
slowly rotating pattern pass; phase alone would let a dropped
beat pass. An **episode** is a maximal run of consecutive perfect
super-cycles. The ledger is counted over **24,000 ticks** per
regime (first 1,500 discarded as settling; 283 super-cycles) —
four times the window at which this pair once appeared to lock.

## Novel Trait

**The pair does not lock, and the failure has a price list.**
Counted, per coupling:

- **Uncoupled** (0.0): 111/283 perfect, 15 count slips, 157 phase
  slips; 18 episodes, longest **15 super-cycles** (~1,188 ticks),
  mean 6.2. Agreement happens by accident, briefly.
- **Wrong medicine** (+2.0 excitatory, delay 0): 67/283 perfect,
  longest episode **6**, mean 3.4 — WALKING. Hurrying an early
  clock is exactly as bad as the concept predicts.
- **Soft hand** (−1.0, delay 8): 116 perfect, longest 25, mean 6.8.
- **The surveyed hand** (−1.5, delay 8): **186/283 perfect**,
  6 count slips, 91 phase slips; 15 episodes, longest **46
  super-cycles (~3,643 ticks)**, mean **12.4** — the longest
  agreement in the sweep, and the tightest phases (dispersions
  0.0865 / 0.0903 / 0.0785). Still not a lock: 97 super-cycles
  slipped. EPISODIC.
- **Firm hand** (−2.0, delay 8): collapses — 45 perfect, longest
  **8**, mean 2.8, WALKING. More delay-medicine is not more
  agreement; the dose response is not monotone.
- **Heavy hand** (−2.5, delay 8): partially recovers — 133
  perfect, longest 26, mean 8.9 — but its dispersions
  (0.1538 / 0.1559 / 0.1332) are the loosest of any inhibitory
  regime: it holds the *count* while the phases smear.
- **Crushing hand** (−3.5, delay 8): 73 perfect, longest 7 —
  WALKING. The boundary above is as real as the one below.
- **Same hand, wrong hour** (−1.5 at delays 0 / 4 / 12): longest
  episodes 28 / 18 / 25 against delay 8's 46. As with the 2:1
  pair, *when* the correction arrives matters as much as its
  size — but here the best hour buys episodes, not a lock.

Two honest notes. First, the earlier probe's "longest perfect
run 130" was counted on counts alone over a shorter settling;
under this instrument's two-gate definition (count AND phase)
the same coupling's longest episode is 46. The stricter ledger
is the one that ships. Second, no regime in the sweep — ten
couplings, 283 super-cycles each — earned a TRUE LOCK, and the
instrument's own test suite forbids the verdict unless every
super-cycle is perfect. The 3:2 does not hold. It *visits*.

## Falsification Condition

The claim — *near 3:2, agreement is episodic, and a timed
inhibitory thread lengthens the episodes without ever locking* —
is falsified if any of these hold:

- any regime in the sweep reports TRUE LOCK (every super-cycle
  perfect) — then the pair could hold after all, and the episode
  ledger was a short window's artefact;
- the surveyed hand's longest episode does not exceed the
  uncoupled pair's — then the thread buys nothing and the dose
  curve is noise;
- the excitatory control matches or beats the best inhibitory
  regime — then "the follower runs early, so delay it" is the
  wrong reading of the pair's geometry.

All three are pinned by unit tests (6/6).

## Usage

```bash
# Headless (handled before anything else; prints the metronome ledger)
cargo run -p widow-episodes -- --headless

# Tests
cargo test -p widow-episodes
```

## The Ledger

Headless runs report, per regime: both solo clocks (spikes and
mean period), the super-cycle arithmetic (two driver cycles
against three follower beats, and the follower's earliness), and
for each coupling the spike counts, observed follower period,
super-cycles perfect / count-slipped / phase-slipped, episode
count, longest and mean episode length, the three beats' median
phases and dispersions, and the verdict — TRUE LOCK 3:2,
EPISODIC, or WALKING. This run: no lock anywhere; the best hand
holds agreement for 46 super-cycles at a time and lets it go,
fifteen times over.

*Count everything. Forgive nothing that drifts.*
