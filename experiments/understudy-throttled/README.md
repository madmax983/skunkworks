# Understudy Throttled 🎭

**Principal:** `experiments/throttled-moths` (The Moth Keeper 🪔)
**Performed by:** The Understudy 🎭
**Status:** Planted 2026-11-03. The principal's sixth room —
evidence teaching rationed to one packet per checker per night —
performed on claim queues, where a check's evidence is the
lamp's own clock, exact.

## The Principal's Role, Extracted

`throttled-moths` ran six twin FDTD rooms (24 moths, 3 nights ×
1500 ticks, same seed; honest lamp period 16 at 0.9, feeds;
decoy 2.2 on alternating 6/26 intervals, burns). Caution is
earned; cautious moths stop to time flicker peaks and refuse a
claim whose interval CV exceeds 0.35. The first five rooms
differ only in what a bystander may learn; the sixth rations it:

| Room | Fed | Burns | Teaching |
|------|-----|-------|----------|
| solo | 45 | 12 | own burns only |
| witness | 43 | 8 | a burn teaches witnesses (13 events, 6 first-cautions) |
| chain | 42 | 13 | a refusal also teaches (495 events, 18 first-cautions) |
| placed | 39 | 13 | a refusal also carries its direction (618 events, 20 first-cautions) |
| evidence | 43 | 12 | every check transmits its peak intervals (943 packets: 342 irregular, 601 steady; 18 first-cautions, 223 acquittals; checks 151) |
| throttled | 44 | 12 | only a checker's FIRST check each night transmits (55 sent, 105 suppressed, 534 events: 234 irregular, 300 steady; 18 first-cautions, 122 acquittals; checks 169) |

The principal's verdict, kept whole: *the ration performed
exactly as aimed on every line but the claim — packets, events,
and acquittals all fell, all 18 learners kept, feeding best of
any social room — and burns did not move (12, the evidence
room's count). Across a stricter per-run probe (133 events)
burns moved by one: volume was not the disease; the first
telling is the teaching.*

## The Understudy's Machinery

No field, no grid, no positions, no gradient, no flight. Each
moth owns a **claim queue** (the stage machinery of
`understudy-moths` / `understudy-evidence`: waits 300–500 ticks,
claims at equal lamp odds, checks sample the lamp's next 5
intervals directly inside the principal's 96-tick cap, tuition
−35, feeding +25, aversion 150 — the script is the principal's).
Social teaching is a **message bus**: a burn posts quietly
(delivery 0.05), a refusal posts publicly (0.30; in the placed
room it also names its lamp), and in the evidence room every
completed check posts its **actual sampled intervals** as a
packet (delivery 0.30). In the throttled room a checker posts
only its first completed check each night; every later check is
counted as suppressed and judged for itself alone.

A packet carries no verdict, no lamp name, no direction. Each
recipient re-computes the interval CV itself, on the same 0.35
line: irregular earns caution; steady can **acquit** — a
recipient with no personal burn whose caution is hearsay loses
one. A queue is sequential where the cave was simultaneous:
messages are delivered before the next moth's turn.

The one thing this stage changes about the *evidence itself*,
named before the run: in the cave, a checker's sample was read
off a shared wave field — both lamps at once, attenuated and
mixed — so a witness re-judged a *mixture*, and the principal's
own diagnosis of the throttled room was that the mixture stands
every learner in both lamps' light at once. On the bus the
sample is the lamp's own clock, exact and unmixed (decoy CV
0.625, honest CV 0, always). This stage has already performed
the evidence room once (`understudy-evidence`): there the
acquittal channel was convicted as the leak — evidence burns
16 against chain 5, with 156 acquittals returning unburned
moths to the uncautious state. Rationing thins packets *and*
acquittals at once. If the ration now pays on an exact stage
where it did not pay in the cave, the cave's crack was the
mixture blunting a real effect; if burns hold at this stage's
evidence count too, the first telling is convicted on both
mediums, at both fidelities.

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

- **P1. Both kinds travel.** Throttled packets delivered in
  both kinds: irregular **> 0** and steady **> 0**
  (principal 234 / 300).
- **P2. The first telling still teaches.** Throttled-first
  learners **> 0** (principal 18; this stage's evidence
  room taught 11).
- **P3. Rationing thins the un-teaching.** Throttled
  acquittals **> 0** and **<** this stage's evidence-room
  acquittals (principal 122 < 223; this stage's evidence
  room acquitted 156).
- **P4 (the claim).** Throttled burns **<** this stage's
  evidence burns. The principal CRACKED this line in the
  cave (12 vs 12, not below); on this stage the evidence
  room burned 16, driven by the acquittal leak the ration
  directly thins — if the line cracks here too, at exact
  fidelity, the first telling alone is the whole disease.
- **P5. The ration is real.** Packets sent **≤ 72** (one per
  checker per night, 24 × 3), suppressed **> 0**, and
  sent + suppressed **=** throttled checks exactly
  (principal 55 sent, 105 suppressed, of 169 checks — the
  principal's own book does not close this sum; this stage
  declares that it must).
- **P6. The flood thins.** Throttled events **<** this
  stage's evidence events (principal 534 < 943; this
  stage's evidence room delivered 634).

These lines were written before this crate was built or run
and are not edited afterward. The headless ledger prints the
principal's numbers beside this performance's and marks each
line HELD or CRACKED.

## Falsification Condition

The principal's closing claim — *volume is not the disease;
the first lesson alone recruits the whole checker crowd* —
splits in two on this stage, and either half can fall:

- If **P4 holds** here, the cave's flat burns were at least
  partly the mixture's: rationing perfect evidence, whose
  acquittals are the convicted leak, pays where rationing
  mixed evidence did not.
- If **P4 cracks** with throttled burns at or above this
  stage's evidence burns (16), the first telling is convicted
  on its own: one exact packet per checker per night still
  recruits enough checkers — and acquits enough of them — to
  burn the room at the flood's rate.

## What the Ledger Found

Six twin queue rooms, same seed, 24 moths, 3 nights × 1500
ticks. The first five rooms reproduced this theater's queue
precedent exactly (`understudy-evidence`: solo fed 61, burns
20; witness fed 50, burns 13, 15 events, 10 learners; chain
fed 56, burns 5, 305 events, 17 learners; placed fed 58,
burns 4, 360 events, 18 learners, 50 aversions; evidence fed
57, burns 16, checks 92, 634 packets — 277 irregular, 357
steady — 11 evidence-first learners, 156 acquittals — the
control script is unchanged when the sixth room is added).

| Room | Fed | Burns | Checks | Teaching |
|------|-----|-------|--------|----------|
| solo | 61 | 20 | 60 | — |
| witness | 50 | 13 | 92 | 15 burn events, 10 learners |
| chain | 56 | 5 | 99 | 305 refusal events, 17 learners |
| placed | 58 | 4 | 59 | 360 events, 18 learners, 50 aversions |
| evidence | 57 | **16** | 92 | 634 packets (277 / 357), 11 learners, 156 acquittals |
| throttled | 54 | **7** | 95 | 418 events (220 irregular, 198 steady), 14 throttled-first learners, 109 acquittals, 63 packets sent, 32 suppressed |

**Bands B1–B4 all HELD** (solo 20 in 6..=26, alive 24;
witness 13 < solo 20; chain 305 > 5× witness 15; placed
learners 18 ≥ chain 17). **P1 HELD** (both kinds travel:
220 irregular, 198 steady). **P2 HELD, with interest**
(14 throttled-first learners — *more* than the evidence
room's 11, and every one on night 1: the first telling did
not merely keep the class, on this stage it seated a larger
one, because the flood's own acquittals were already
un-seating learners while it taught). **P3 HELD** (109
acquittals, thinned from 156 but nowhere near gone).
**P5 HELD** (63 sent ≤ 72, 32 suppressed, and
63 + 32 = 95 checks exactly — this stage's book closes the
sum the principal's left open). **P6 HELD** (418 events <
634). Honest refusals 0 in all six rooms; alive 24/24
everywhere; 13/13 tests.

**P4 HELD — and the hold is the discovery.** Throttled burns
**7** (6 / 0 / 1 by night), against this stage's evidence
burns of **16** and within sight of the chain room's 5. The
principal cracked this exact line in the cave (12 vs 12, the
ration moving nothing), and its verdict was that volume is
not the disease. On an exact stage the ration pays in full:
cutting the packet stream by a third cut burns by more than
half. The falsification condition resolves on its first
clause, with the mechanism this theater convicted last time:

- **The cave's flat line was the mixture protecting the
  leak.** In the cave an acquittal was cheap bookkeeping —
  caution there is constantly re-earned off a field where
  both lamps speak at once, so a moth talked out of hearsay
  caution still checks its way carefully, and rationing the
  packets changed nothing the burns could feel. On this
  stage caution is the *only* protection an unburned moth
  has, the acquittal is the burn's open door (evidence
  16 = the acquittal leak, convicted in `understudy-evidence`),
  and the ration closes that door partway: 109 acquittals
  instead of 156, and burns 7 instead of 16.
- **The first telling is still the teaching.** All 14
  throttled-first cautions arrived on night 1, from the
  first packets alone; nights 2–3 taught nobody new, exactly
  as the principal found (18 learners kept on 55 packets).
  What the later packets mostly carried on this stage was
  not teaching but *un*-teaching — and that is the part the
  ration starves.

So the principal's verdict stands corrected in one place,
from the other medium: volume was never the disease *in the
cave*, because the cave's mixture blunted both cuts of the
evidence. Where the evidence is exact, the second cut is the
deeper one — and rationing, which thins the second cut along
with the flood, is a real medicine there. The disease was
never how much was said. It was what the saying could undo,
and how much the room let an undoing cost.

Named succession seed: lamp-scoped acquittal on this stage —
a steady packet acquits only caution the *same lamp's*
evidence earned (the seed `understudy-evidence` named, still
open) — does scoping close the leak the ration only thins,
and do burns then reach the chain room's 5 without losing
the 14 learners? Or: acquittal with a floor (hearsay caution
never below 1) in the throttled room — ration plus residue,
the two partial medicines at once.

## Usage

```bash
cargo run -p understudy-throttled -- --headless
cargo test -p understudy-throttled
```

*Give me the role. Keep your machinery — and ration the
lesson as you like; I will count what the first telling does.*
