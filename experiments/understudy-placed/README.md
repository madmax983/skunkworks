# Understudy Placed 🎭

**Principal:** `experiments/placed-moths` (The Moth Keeper 🪔)
**Performed by:** The Understudy 🎭
**Status:** Planted 2026-11-01. The principal's four rooms —
solo, witness, chain, placed — performed on claim queues, where
a refusal message can carry the lamp's name exactly.

## The Principal's Role, Extracted

`placed-moths` ran four twin FDTD rooms (24 moths, 3 nights ×
1500 ticks, same seed; honest lamp period 16 at 0.9, feeds;
decoy 2.2 on alternating 6/26 intervals, burns). Caution is
earned; cautious moths stop to time flicker peaks and refuse a
claim whose interval CV exceeds 0.35. The rooms differ only in
what a bystander may learn:

| Room | Fed | Burns | Teaching |
|------|-----|-------|----------|
| solo | 45 | 12 | own burns only |
| witness | 43 | 8 | a burn within 12 cells teaches (13 events, 6 first-cautions) |
| chain | 42 | 13 | a refusal also teaches (495 events, 18 first-cautions) |
| placed | 39 | 13 | a refusal also carries its gradient direction (618 events, 20 first-cautions) |

The principal's verdict, kept whole: *depth is the poison —
each hand the caution passes through keeps the caution and
loses the evidence, and no annotation bolted onto hearsay
restores evidence that was never transmitted.* Its two named
reasons: (1) a refusal's gradient in a reverberant cave points
along the mixture, not the lamp — the "place" carried was
already a rumour; (2) the witness cannot tell a decoy refusal
from an honest-lamp refusal the interference provoked.

## The Understudy's Machinery

No field, no grid, no positions, no gradient, no flight. Each
moth owns a **claim queue** (the stage machinery of
`understudy-moths`: waits 300–500 ticks, claims at equal lamp
odds, checks sample the lamp's next 5 intervals directly inside
the principal's 96-tick cap, tuition −35, feeding +25, aversion
150 — the script is the principal's). Social teaching is a
**message bus**: a burn or refusal posts a message, delivered
to each other moth with a fixed probability — burns quiet at
0.05, refusals public at 0.30, chosen to reproduce the
principal's delivery ratios (~1 witness delivery per burn,
~7 chain deliveries per refusal). A queue is sequential where
the cave was simultaneous: a moth's messages are delivered
before the next moth's turn.

The one thing the new machinery changes about the *evidence*:
a bus message can carry the refused lamp's **identity**,
copied losslessly at every hand — no gradient, no mixture,
no rumour. In the placed room a delivered refusal therefore
hands the witness a **named aversion**: its next claim from
that lamp is refused pre-emptively, without a check, and that
refusal posts again — depth accumulates on the bus exactly
as it did in the cave, but the name survives every hand.

## Tolerances and Predictions — Declared Before the First Run

Comparison bands (totals across three nights):

- **B1.** Solo burns in **6..=26** (this theater's queue
  precedent, `understudy-moths`: 20) and solo survivors
  exactly **24**.
- **B2.** Witness burns **<** solo burns (principal 8 < 12).
- **B3.** Chain refusal events **> 5×** witness burn events
  (principal 495 vs 13), with witness events > 0.
- **B4.** Placed first-cautions **≥** chain first-cautions
  (principal 20 ≥ 18).

Predictions — the performance exists to test these:

- **P1. Identity restores the evidence.** Placed burns
  **<** chain burns **and ≤** witness burns. The principal
  cracked exactly this line in the cave (13, 13, 8). If depth
  itself — not the medium's rumour — is the poison, this
  cracks here too, with the name delivered perfectly.
- **P2. Teaching still widens.** Placed events **>** chain
  events (principal 618 > 495).
- **P3. Aversions are spent.** Placed-room averted claims
  **≥ 10** — the named aversions are used, not merely held.
- **P4. The honest lamp is never named.** Honest refusals
  total exactly **0** across all four queue rooms: on exact
  delivery a check never refuses the honest lamp, so no
  message can carry its name — the principal's second reason
  (honest refusals poisoning the chain) has no queue analog.

These lines were written before this crate was built or run
and are not edited afterward. The headless ledger prints the
principal's numbers beside this performance's and marks each
line HELD or CRACKED.

## Falsification Condition

The principal's closing claim — *depth is the poison, because
the evidence is never transmitted* — splits in two on this
stage, and either half can fall:

- If **P1 holds**, the cave's verdict was the medium's, not
  depth's: third-hand caution *can* protect when the message
  carries the evidence itself (the lamp's name), and "depth"
  was standing in for "lossy transmission."
- If **P1 cracks** despite exact, lossless naming at every
  hand, depth is convicted on its own: caution transmitted
  at any fidelity fails to protect, and the failure lives in
  the teaching, not the channel.

## What the Ledger Found

Four twin queue rooms, same seed, 24 moths, 3 nights × 1500
ticks. The solo room reproduced this theater's queue precedent
exactly (`understudy-moths`: fed 61, burns 20, checks 60,
refusals 26 — the solo script is unchanged when nobody teaches).

| Room | Fed | Burns | Checks | Refusals | Events (learners) | Averted |
|------|-----|-------|--------|----------|-------------------|---------|
| solo | 61 | 20 | 60 | 26 | 0 | 0 |
| witness | 50 | 13 | 92 | 55 | 15 burn (10) | 0 |
| chain | 56 | 5 | 99 | 45 | 305 refusal (17) | 0 |
| placed | 58 | 4 | 59 | 52 | 360 refusal (18) | 50 |

**Every declared line HELD.** B1 (solo 20 in 6..=26, alive
24), B2 (witness 13 < solo 20), B3 (chain 305 > 5× witness
15), B4 (placed first-cautions 18 ≥ chain 17), P2 (placed
events 360 > chain 305), P3 (50 averted claims — the named
aversions were spent, and each spent aversion posted again),
P4 (honest refusals exactly 0 in all four rooms — no message
ever named the honest lamp).

**P1 HELD — and the margin is the honest part.** Placed
burns **4**, chain **5**, witness **13**: the line the
principal cracked in the cave (13, 13, 8) holds on the bus,
so a refusal that carries its lamp's name *does* protect at
third hand. But it protects by one burn over the chain room,
not by the witness room's margin in reverse — because on
this stage the chain room itself already collapsed from the
solo 20 to 5, a protection the cave's chain never found
(12 → 13). Two medium effects were hiding inside the
principal's one verdict:

1. **The check, not just the teaching, was lossy in the
   cave.** A cautious queue moth samples the lamp's intervals
   exactly and is never fooled (fooled-check burns 0 in every
   queue role this theater has run); a cautious cave moth
   reads a mixture and burns through its own check. Caution
   alone is nearly sufficient where the evidence at the lamp
   is exact — most of P1's margin was already spent before
   the placed annotation arrived.
2. **The teaching was lossy only in the cave.** On the bus,
   chain caution spread (305 events, 17 first-cautions) and
   burns fell night by night (5 → 0 → 0); the placed name
   then converted refusals into pre-emptive aversions (50
   spent) and held burns at 4 (4 → 0 → 0).

So the falsification condition resolves on its first clause,
with a correction to both gardens' phrasing: "depth is the
poison" was the medium's verdict twice over. Depth on a
lossless bus is not poison — it is merely *redundant* once
the check itself is exact, which is why the name's margin is
thin. What the cave was measuring was never how many hands
the caution passed through; it was how much evidence each
hand — and the lamp itself — was forced to lose.

## Usage

```bash
cargo run -p understudy-placed -- --headless
cargo test -p understudy-placed
```

*Give me the role. Keep your machinery — and ask what your
messages can carry that your wavefronts could not.*
