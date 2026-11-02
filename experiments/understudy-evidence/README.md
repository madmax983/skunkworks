# Understudy Evidence 🎭

**Principal:** `experiments/evidence-moths` (The Moth Keeper 🪔)
**Performed by:** The Understudy 🎭
**Status:** Planted 2026-11-02. The principal's five rooms —
solo, witness, chain, placed, evidence — performed on claim
queues, where a check's evidence is the lamp's own clock, exact.

## The Principal's Role, Extracted

`evidence-moths` ran five twin FDTD rooms (24 moths, 3 nights ×
1500 ticks, same seed; honest lamp period 16 at 0.9, feeds;
decoy 2.2 on alternating 6/26 intervals, burns). Caution is
earned; cautious moths stop to time flicker peaks and refuse a
claim whose interval CV exceeds 0.35. The rooms differ only in
what a bystander may learn:

| Room | Fed | Burns | Teaching |
|------|-----|-------|----------|
| solo | 45 | 12 | own burns only |
| witness | 43 | 8 | a burn teaches witnesses (13 events, 6 first-cautions) |
| chain | 42 | 13 | a refusal also teaches (495 events, 18 first-cautions) |
| placed | 39 | 13 | a refusal also carries its direction (618 events, 20 first-cautions) |
| evidence | 43 | 12 | every check transmits its peak intervals (943 packets: 342 irregular, 601 steady; 18 first-cautions, 223 acquittals; checks 151) |

The principal's verdict, kept whole: *evidence survives third
hand — a witness that re-judges can learn (18) and can be
un-taught (223 acquittals) — and still protects by only one
burn (12 vs chain 13, witness 8), because teaching at every
check floods the room; volume, not fidelity, is the remaining
disease.*

## The Understudy's Machinery

No field, no grid, no positions, no gradient, no flight. Each
moth owns a **claim queue** (the stage machinery of
`understudy-moths`: waits 300–500 ticks, claims at equal lamp
odds, checks sample the lamp's next 5 intervals directly inside
the principal's 96-tick cap, tuition −35, feeding +25, aversion
150 — the script is the principal's). Social teaching is a
**message bus**: a burn posts quietly (delivery 0.05, matching
the principal's ~1 witness delivery per burn), a refusal posts
publicly (0.30, ~7 deliveries per refusal; in the placed room
it also names its lamp, exactly as in `understudy-placed`),
and in the evidence room every completed check posts its
**actual sampled intervals** as a packet (delivery 0.30,
~6.9 deliveries per check against the principal's ~6.2).

A packet carries no verdict, no lamp name, no direction. Each
recipient re-computes the interval CV itself, on the same 0.35
line: irregular earns caution; steady can **acquit** — a
recipient with no personal burn whose caution is hearsay
loses one. A queue is sequential where the cave was
simultaneous: messages are delivered before the next moth's
turn.

The one thing this stage changes about the *evidence itself*,
named before the run: in the cave, a checker's sample was read
off a shared wave field — both lamps at once, attenuated and
mixed — so a witness re-judged a *mixture*. On the bus the
sample is the lamp's own clock, exact and unmixed (decoy CV
0.625, honest CV 0, always). Re-judging here is therefore
re-judging perfect evidence. If the principal's thin margin
was the mixture's fault, this stage should widen it; if the
margin was the flood's, perfect evidence drowns the same.

## Tolerances and Predictions — Declared Before the First Run

Comparison bands (totals across three nights, this stage):

- **B1.** Solo burns in **6..=26** (this theater's queue
  precedent, `understudy-moths` / `understudy-placed`: 20)
  and solo survivors exactly **24**.
- **B2.** Witness burns **<** solo burns (principal 8 < 12).
- **B3.** Chain refusal events **> 5×** witness burn events
  (principal 495 vs 13), with witness events > 0.
- **B4.** Placed first-cautions **≥** chain first-cautions
  (principal 20 ≥ 18).

Predictions — the performance exists to test these:

- **P1. Both kinds travel.** Evidence packets delivered in
  both kinds: irregular **> 0** and steady **> 0**
  (principal 342 / 601).
- **P2. Evidence teaches first-hand.** Evidence-first
  learners **> 0** (principal 18).
- **P3. Evidence can un-teach.** Acquittals **> 0** — steady
  packets talk hearsay caution down on this stage too
  (principal 223).
- **P4 (the claim).** Evidence burns **<** this stage's chain
  burns. The principal held this by exactly one (12 < 13);
  with perfect, unmixed evidence the line should hold at
  least as well — if it cracks here, the flood, not the
  mixture, was always the disease.
- **P5. Acquittals prune checking.** Evidence checks **<**
  this stage's chain checks (principal 151 < 208).
- **P6. The flood is the flood.** Evidence events **>**
  chain events on this stage too (principal 943 > 495):
  teaching at every check out-publishes teaching at every
  refusal, whatever the medium.

These lines were written before this crate was built or run
and are not edited afterward. The headless ledger prints the
principal's numbers beside this performance's and marks each
line HELD or CRACKED.

## Falsification Condition

The principal's closing claim — *third-hand caution protects
once the lesson carries its evidence; what remains is volume*
— splits in two on this stage, and either half can fall:

- If **P4 holds with a wider margin than one burn**, the
  cave's thin margin was partly the mixture's: re-judged
  evidence protects better when the evidence was never mixed.
- If **P4 cracks** despite exact, unmixed evidence at every
  hand, volume is convicted on its own: a packet per check
  creates cautious checkers faster than their caution can
  protect them, on any medium, at any fidelity.

## What the Ledger Found

Five twin queue rooms, same seed, 24 moths, 3 nights × 1500
ticks. The first four rooms reproduced this theater's queue
precedent exactly (`understudy-placed`: solo fed 61, burns 20;
witness fed 50, burns 13, 15 events, 10 learners; chain fed
56, burns 5, 305 events, 17 learners; placed fed 58, burns 4,
360 events, 18 learners, 50 aversions spent — the control
script is unchanged when the fifth room is added).

| Room | Fed | Burns | Checks | Teaching |
|------|-----|-------|--------|----------|
| solo | 61 | 20 | 60 | — |
| witness | 50 | 13 | 92 | 15 burn events, 10 learners |
| chain | 56 | 5 | 99 | 305 refusal events, 17 learners |
| placed | 58 | 4 | 59 | 360 events, 18 learners, 50 aversions |
| evidence | 57 | **16** | 92 | 634 packets (277 irregular, 357 steady), 11 evidence-first learners, **156 acquittals** |

**Bands B1–B4 all HELD** (solo 20 in 6..=26, alive 24;
witness 13 < solo 20; chain 305 > 5× witness 15; placed
learners 18 ≥ chain 17). **P1 HELD** (both kinds travel:
277 irregular, 357 steady). **P2 HELD** (11 of 24 moths
first-cautious by re-judged evidence — all on night 1).
**P3 HELD, emphatically** (156 acquittals). **P5 HELD**
(evidence checks 92 < chain 99 — acquittals did prune
checking, as in the cave). **P6 HELD** (634 packets > 305
chain events — the flood is the flood). Honest refusals 0
in all five rooms; alive 24/24 everywhere; 11/11 tests.

**P4 CRACKED — and the crack is the discovery.** Evidence
burns **16**, against this stage's chain burns of **5** and
the placed room's 4. The principal held this line by one
burn in the cave (12 < 13); on a stage where the evidence is
perfect — the lamp's own clock, unmixed, re-judged exactly —
the evidence room burned more than three times the chain
room. The falsification condition resolves on its second
clause, with a mechanism the cave could not isolate:

- **The acquittal channel is the leak.** In the cave,
  un-teaching was safe bookkeeping: caution there is
  constantly re-earned off a mixture, and a moth talked out
  of hearsay caution still checks its way carefully. On this
  stage caution is the *only* protection an unburned moth
  has, and 156 acquittals kept returning moths to the
  uncautious state — where the next decoy claim is not a
  check but a burn (evidence burns by night: 8, 4, 4; the
  room never settles the way the chain room settles,
  5 → 0 → 0). Steady evidence is true — the honest lamp
  *is* steady — but a steady sample says nothing about the
  *other* lamp, and a witness that spends it as a general
  acquittal has un-learned a lesson the sample never
  contradicted.
- **Volume was never the whole disease either.** 634
  packets taught 11 first-cautions, all on night 1; after
  that the flood mostly *un*-taught (156 acquittals against
  a caution pool that irregular packets kept refilling).
  The chain room's verdict-only teaching, lossy as the
  Keeper called it, has one virtue evidence lacks here: a
  verdict can only add caution. Evidence, judged fairly,
  cuts both ways — and on an exact stage the second cut is
  the deeper one.

So the principal's verdict stands corrected in one place:
evidence does survive third hand, on any medium — what does
not survive is the assumption that surviving evidence is
protective evidence. The cave's one-burn margin was not the
mixture hiding a larger win; it was the mixture blunting
the acquittal's edge.

Named succession seed: an evidence room where a steady
packet can acquit only caution that *the same lamp's*
evidence earned — lamp-scoped acquittal, where the packet
must name what it acquits — does scoping the un-teaching
close the leak while keeping the 11 learners? Or: acquittal
with a floor (caution never below 1 by hearsay) — is a
one-deep residue enough to keep the check, and the moth?

## Usage

```bash
cargo run -p understudy-evidence -- --headless
cargo test -p understudy-evidence
```

*Give me the role. Keep your machinery — and let the witness
judge the evidence, never the verdict.*
