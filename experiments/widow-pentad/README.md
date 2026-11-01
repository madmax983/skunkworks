# Widow Pentad 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-11-01. Headless ledger below.

## Lineage

- **`neuro-sim`** — the two clocks: a Regular Spiking driver and a
  Fast Spiking follower, same equations, different recovery
  constants.
- **`widow-subharmonic` / `widow-island` / `widow-fringe`** settled
  the near-2:1 pair (FS I=6): a timed excitatory thread locks it,
  and at the long window that lock is episodic under a per-cycle
  phase gate.
- **`widow-episodes`** settled the near-3:2 pair (FS I=5): the
  follower runs *early* by 0.53 ticks per super-cycle, the medicine
  is inhibition, and no coupling locks — agreement is priced in
  episodes, longest 46 super-cycles at (-1.5, delay 8).
- This instrument is the next named ratio in the survey: **5:3**.

## Concept

Two neurons, joined by exactly one synapse, whose weight *and*
delay are the only things the Widow is allowed to turn:

- **The driver** — Regular Spiking, constant current I=12. Solo
  measured period: **39.60 ticks** (568 spikes in the window).
- **The follower** — Fast Spiking, constant current I=5.3. Solo
  measured period: **24.06 ticks** (935 spikes). Five follower
  beats span **120.31** against three driver cycles' **118.80** —
  ratio near 5:3, and the follower runs **late by 1.51 ticks per
  super-cycle**. Late is the whole story, reversed from the 3:2
  pair: the medicine for this pair is *excitation* (hurry it),
  and inhibition is kept as the wrong-medicine control.

A **super-cycle** is three consecutive driver cycles. It is
**perfect** when it holds exactly five follower beats AND each
beat's phase on the super-cycle's ring lies within ±0.06 of that
beat's median phase for the regime (the `widow-episodes`
two-gate rule, carried over unchanged). An **episode** is a
maximal run of consecutive perfect super-cycles. The ledger is
counted over **24,000 ticks** per regime (first 1,500 discarded;
188 super-cycles).

## Novel Trait

**The 5:3 does not lock either — but its dose curve is the mirror
of the 3:2's, and its best hand is found, not assumed.**

- **Uncoupled:** 101/188 perfect, longest episode **27**
  super-cycles, mean 6.7 — EPISODIC by accident, as the 3:2 was.
- **Wrong medicine** (-1.5, delay 8 — the 3:2's best hand, applied
  here): longest episode collapses to **9**, mean 4.0, WALKING.
  Delaying a late clock is exactly as bad as hurrying an early
  one was. The sign of the correction must match the sign of
  the drift, and this pair's sign is opposite to its sibling's.
- **The surveyed hand** (+1.0, delay 8): **119/188 perfect**,
  longest episode **47 super-cycles (~5,583 ticks)**, mean 13.2 —
  the longest agreement in the sweep, at the same delay that
  served the 3:2 pair. When the correction arrives matters in
  both directions.
- **Firm hand** (+1.5, delay 8): the most perfect super-cycles
  (**120/188**) but a shorter longest episode (26) — count and
  duration are different currencies, again.
- **Heavy / crushing hands** (+2.5 / +4.0, delay 8): longest
  episodes fall to 19 / 12 — the dose response is not monotone;
  there is a ceiling as well as a floor.
- **Same hand, wrong hour** (+1.0 at delays 0 / 4 / 12): longest
  episodes 20 / 23 / 33 against delay 8's 47.
- **No regime earns TRUE LOCK 5:3** — ten couplings, 188
  super-cycles each, and the test suite forbids the verdict
  unless every super-cycle is perfect.

## Falsification Condition

The claim — *near 5:3, the follower runs late, so a timed
excitatory thread lengthens episodes without ever locking, and
inhibition shortens them* — is falsified if:

- any regime in the sweep reports TRUE LOCK 5:3 (every
  super-cycle perfect) — then the pair could hold after all;
- the wrong-medicine (inhibitory) regime's longest episode
  matches or beats the best excitatory regime's — then "the
  follower runs late, so hurry it" is the wrong reading;
- no excitatory regime outlasts the uncoupled pair — then the
  thread buys nothing and the dose curve is noise.

All three are pinned by unit tests (6/6).

## Usage

```bash
cargo run -p widow-pentad -- --headless
cargo test -p widow-pentad
```

*Count everything. Forgive nothing that drifts.*
