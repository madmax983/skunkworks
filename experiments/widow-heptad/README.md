# Widow Heptad 🕰️

**Parent:** `crates/neuro-sim` (the Izhikevich spiking-neuron model)
**Built by:** The Clockwork Widow 🕰️
**Status:** Planted 2026-11-05. Headless ledger below.

## Lineage

- **`neuro-sim`** — the two clocks: a Regular Spiking driver and a
  Fast Spiking follower, same equations, different recovery
  constants.
- **`widow-subharmonic` / `widow-island` / `widow-fringe`** settled
  the near-**2:1** pair (FS I=6): a timed excitatory thread locks
  it, and at the long window that lock is episodic under a
  per-cycle phase gate.
- **`widow-episodes`** took up the near-**3:2** pair (FS I=5): the
  follower runs *early* by 0.53 ticks per super-cycle, the medicine
  is inhibition, and no coupling locks — agreement is priced in
  episodes, longest 46 super-cycles at (-1.5, delay 8).
- **`widow-pentad`** and its plane / peak / diagonal / ridge
  surveys settled the near-**5:3** pair (FS I=5.3): the follower
  runs *late* by 1.51 ticks per super-cycle, the medicine is
  excitation, and no coupling locks — its compensation segment is
  finite, crowned at (+1.50, delay 4), longest episode 55.
- This instrument is the next named ratio in the survey: **7:5**,
  the pentad's mirror — an early clock again, but its earliness
  (1.92) is nearer the pentad's lateness (1.51) than the 3:2's
  earliness (0.53).

## Concept

Two neurons, joined by exactly one synapse, whose weight *and*
delay are the only things the Widow is allowed to turn:

- **The driver** — Regular Spiking, I=12. Solo period **39.60**.
- **The follower** — Fast Spiking, I=4.8. Solo period **28.01**.
  Seven follower beats span **196.08** against five driver
  cycles' **198.00** — near 7:5, the follower **early by 1.92
  ticks per super-cycle**.

A **super-cycle** is five driver cycles; **perfect** = exactly
seven follower beats, each within ±0.06 of its median phase
(two-gate rule, unchanged from `widow-episodes`). An **episode**
is a maximal run of perfect super-cycles. Each regime counted
over **24,000 ticks** (first 1,500 discarded; 112 super-cycles).
The sweep mirrors the pentad's row with its signs reversed: an
inhibitory dose ladder at delay 8, one excitatory wrong-medicine
control, and the surveyed hand at delays 0 / 4 / 12. Six
predictions were declared in the source before the sweep ran.
Five held; one cracked, and the crack is kept.

## Novel Trait

**The 7:5 does not lock either — and at this ratio the wrong
medicine keeps the count and kills the run.**

| regime | perfect | longest | mean ep. | verdict |
|--------|---------|---------|----------|---------|
| uncoupled | 83/112 | 17 | 8.3 | EPISODIC |
| wrong medicine (+1.5, 8) | 83/112 | **6** | 3.8 | WALKING |
| gentle hand (-0.5, 8) | 75/112 | 21 | 10.7 | EPISODIC |
| surveyed hand (-1.0, 8) | 76/112 | 22 | 7.6 | EPISODIC |
| firm hand (-1.5, 8) | **89/112** | **45** | 12.7 | EPISODIC |
| heavy hand (-2.5, 8) | 82/112 | 15 | 8.2 | EPISODIC |
| crushing hand (-4.0, 8) | 64/112 | 7 | 3.6 | WALKING |
| surveyed, no delay (-1.0, 0) | 78/112 | 29 | 8.7 | EPISODIC |
| surveyed, early (-1.0, 4) | 70/112 | 32 | 10.0 | EPISODIC |
| surveyed, late (-1.0, 12) | 74/112 | 13 | 6.7 | EPISODIC |

- **No lock (P1 HELD).** Ten regimes, zero TRUE LOCK 7:5. The
  firm hand's 89/112 is the sweep's best count and it still
  slips 14 by count and 9 by phase.
- **The wrong medicine, priced exactly (P2 HELD).** Excitation
  (+1.5, delay 8) ties the uncoupled pair's perfect count —
  83/112, field for field the same total — and collapses its
  longest episode from 17 to **6**, across 22 episodes against
  the uncoupled pair's 10. Hurrying an early clock does not
  reduce how often it agrees; it reduces how *long* it can keep
  agreeing. A ledger that reports only the count would call the
  poison a peer.
- **The medicine's sign is inhibition (P3 HELD).** The firm hand
  (-1.5, delay 8) holds the longest episode, **45 super-cycles**
  (~8,909 ticks), and the best perfect count, 89 — the first
  ratio in the survey where one hand leads both columns. Its
  follower period, 28.36, is pulled past the solo 28.01 to just
  beyond the exact 7:5 period (28.29): the dose that works
  overshoots the metronome's arithmetic by a hair and holds
  anyway.
- **The dose curve is not monotone (P4 HELD).** Past the firm
  hand it falls on both sides of the ledger: heavy (-2.5)
  longest 15, crushing (-4.0) longest **7** and perfect 64 —
  delaying an early clock too hard is its own disease.
- **The delay-8 hour is not privileged here (P5 CRACKED).** At
  the surveyed weight the undelayed hand holds longest 29 and
  the early hand (delay 4) holds **32**, both beating delay 8's
  22; only the late hand (delay 12, longest 13) falls below it.
  At 3:2 and 5:3 the surveyed hand's best hour was delay 8; at
  7:5 the same weight prefers to arrive at once or soon. Delay
  still selects — the late hand proves the axis is real — but
  it does not select the hour the earlier ratios taught. Note
  the limit of the crack: the sweep's champion, the firm hand,
  still stands at delay 8. What cracked is the surveyed hand's
  hour, not the axis.
- **Accidental agreement, again (P6 HELD).** Uncoupled, the
  pair is EPISODIC by accident: 83/112 perfect, longest 17 —
  a 1.92-tick earliness is slow enough that the pattern holds
  for stretches before it walks off the ring.

The survey's four ratios now stand in a row: 2:1 locks; 3:2,
5:3, and 7:5 do not, and are priced in episodes (46, 55, 45 at
their best hands). Earliness and lateness remain opposite
diseases with opposite medicines — and the wrong medicine at
7:5 is the subtlest failure yet counted in this row: it leaves
the count standing and takes the duration.

## Falsification Condition

The claim — *the near-7:5 pair (FS I=4.8 against RS I=12) does
not lock under any regime in this sweep; inhibition is its
medicine (firm hand longest 45, perfect 89); excitation ties the
uncoupled perfect count (83) while collapsing its longest
episode (17 → 6); and at the surveyed weight delays 0 and 4
outlast delay 8* — is falsified if:

- any regime reports TRUE LOCK 7:5 — then the pair could hold
  after all, somewhere in this sweep;
- the solo clocks fail to reproduce 39.60 / 28.01, or their
  super-cycle earliness falls outside 0.5–3.5 ticks — then this
  is not the pair the ledger describes;
- the wrong medicine's longest episode reaches the uncoupled
  pair's 17 on a recount — then excitation did not shatter the
  runs, and the count/duration split is this run's artefact;
- (-1.0, delay 8) outlasts both (-1.0, delay 0) and
  (-1.0, delay 4) on a recount — then P5 stands after all and
  the crack was noise.

## Headless Ledger

`cargo run -p widow-heptad -- --headless` prints the metronome
ledger: both solo clocks, the super-cycle earliness, and per
regime the perfect count, slips split by kind (count vs phase),
episode count, longest and mean episode, median beat phases and
dispersions, and the verdict. The numbers in the table above
are that ledger, verbatim in kind.
