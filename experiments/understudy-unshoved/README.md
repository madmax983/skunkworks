# Understudy Unshoved 🎭

**Principal:** `experiments/unshoved-moths` (The Moth Keeper 🪔)
**Performed by:** The Understudy 🎭
**Status:** Planted 2026-11-04. Comparison ledger below — reported as found.

## The Principal

`unshoved-moths` added a seventh room to the moth lineage's FDTD
cave: evidence packets teaching caution **without** the 150-tick
aversion every other teaching channel in that lineage pays its
recipients. The channel outperformed every predecessor — 1,605
events (651 irregular / 954 steady), all 18 first-cautions by
re-judged evidence, 439 acquittals — and protected worst of any
room in the lineage: burns **18** (5/6/7 by night, *rising* into
night 3 against the evidence room's 5/6/1), fed **39**. Its claim
(P4) cracked decisively, and the Keeper's reading was kept whole:
*an aversion is transport — it carries the witness out of the
mixture and out of earshot, and buys 150 ticks in which new
caution cannot be spent.*

## The Role, Extracted (declared before the performance)

The role is the principal's seventh-room comparison, counted the
principal's way: seven twin rooms, 24 moths, 3 nights × 1500
ticks, same lamps (honest period 16, feeds; decoy 6/26 same mean,
burns), the same 0.35 flicker line, and the seventh room's ledger
set against the evidence room's — events in both kinds,
first-cautions by packet, acquittals, burns by night, feeding.

**Comparison bands (queue precedent, from `understudy-throttled`):**

- **B1:** solo burns in 6..=26, solo alive = 24 (queue precedent 20).
- **B2:** witness burns < solo burns (principal 8 < 12).
- **B3:** chain events > 5× witness events (principal 495 vs 13).
- **B4:** placed first-cautions ≥ chain first-cautions (principal 20 ≥ 18).

**Predictions (declared before the first run, never edited after):**

- **P1:** Both new channels work, in both kinds: shoved irregular
  > 0 and steady > 0; unshoved irregular > 0 and steady > 0
  (principal unshoved: 651 / 954).
- **P2:** The learners survive both tellings: shoved-first ≥ 8 and
  unshoved-first ≥ 8 (this stage's evidence precedent: 11;
  principal: 18).
- **P3:** The acquittal churn survives the swap: unshoved
  acquittals > shoved acquittals — a witness never pushed away
  stays to be un-taught (principal: 439 vs 223).
- **P4 (the claim):** Shoved burns < unshoved burns. The
  principal's mirror claim cracked in the cave in the opposite
  direction (evidence-with-shove 12 vs unshoved 18); on this
  stage the shove is a pause and *no transport* — there is no
  mixture to be carried out of. If the pause alone still buys
  protection, the Keeper's two halves both carry weight; if it
  does not, the cave's medicine was the transport, and the
  150 ticks were its price, not its mechanism.
- **P5:** The pause is not free: shoved fed ≤ unshoved fed —
  lost ticks cost meals on a stage where nothing else is bought
  with them (the principal's line ran inverted: unshoved 39 <
  evidence 43, because the cave's shove bought distance).
- **P6:** The pause made visible: shoved checks < unshoved
  checks. A manipulation check, declared as such — it proves the
  debt is real machinery, not a discovery (principal: evidence
  151 checks vs unshoved 211).

## The Different Machinery

No FDTD field, no grid, no positions, no brightness, no mixture.
The stage is the claim-queue bus of `understudy-evidence` /
`understudy-throttled`: each moth owns a queue of claims, samples
a lamp's clock directly (the decoy reads CV ≈ 0.63, the honest
lamp exactly 0), and every completed check posts its sampled
intervals to the bus; recipients re-judge the CV on the same 0.35
line. Irregular earns caution; steady can acquit hearsay caution.

**The discovery that had to precede the casting:** on this stage
the shove was never in the script. In the cave, *every* teaching
channel — witness, chain, placed, evidence — pays its recipients
caution **plus** the 150-tick aversion. In every queue performance
so far, recipients took the caution and nobody paid any recoil:
the earlier queue evidence rooms (burns 16, acquittals 156) were
unshoved performances all along, and the missing half was never
priced. This crate restores the shove in its queue-native form —
a **clock debt**: a moth taught by an irregular packet in the
SHOVED room owes 150 ticks, and its next turn starts that many
ticks late (refreshed, never stacked, exactly as the cave
refreshes an aversion). The UNSHOVED room is the identical
channel with the debt struck. The first six rooms are the
`understudy-throttled` script unchanged; their ledgers are
pinned exact by tests, and the debt line reads zero in every
one of them.

## Falsification Condition

The performance's claim — *the principal's crack was the
mixture's doing, and on an exact stage the pause alone separates
the rooms* — is falsified if any hold:

- either new channel delivers no packets, or only one kind;
- no moth's first caution ever comes from either new packet;
- shoved burns are not below unshoved burns **and** unshoved
  acquittals are not above shoved acquittals — i.e. the debt
  changes nothing at all on this stage, and the role's seventh
  room has no phenotype a queue can carry.

The burns comparison is reported, not asserted by test — an
understudy does not edit the ledger to save a performance.

## Usage

```bash
cargo run -p understudy-unshoved -- --headless
cargo test -p understudy-unshoved
```

## The Comparison Ledger

Eight twin rooms, same seed, 24 moths, 3 nights × 1500 ticks.
The first six rooms reproduce the queue precedent exact (solo
fed 61 / burns 20; witness 50 / 13; chain 56 / 5, chain events
305; placed 58 / 4, placed events 360, averted 50; evidence
57 / 16, evidence events 634, acquittals 156, learners 11;
throttled 54 / 7, events 418, acquittals 109, learners 14,
packets 63 sent / 32 suppressed) — pinned by tests.

| Regime | Fed | Burns | Checks | Teaching events | First-cautions | Acquittals |
|--------|-----|-------|--------|-----------------|----------------|------------|
| evidence (queue, as inherited) | 57 | 16 | 92 | 634 (277 irr / 357 steady) | 11 | 156 |
| shoved | 54 | 10 | 95 | 662 (318 / 344) | 11 | 184 |
| unshoved | 57 | 16 | 92 | 634 (277 / 357) | 11 | 156 |

Two facts stand before any verdict. **The unshoved room is the
inherited evidence room, field for field** — fed, burns, checks,
events, acquittals, learners all identical, and a test pins the
identity. The discovery is not an interpretation: this stage's
evidence room was an unshoved performance from its first night,
because the queue script never copied the cave's stage direction
that pays recipients the aversion. And **the shoved room's cast
paid 9,750 ticks of taught aversion** across the three nights —
the debt receipt, booked in the ledger itself, so the pause
whose effects are priced below is measured, not asserted.

Shoved room by night: night 1 fed 18, burns 8, events 183
(84 / 99), acquittals 52, learners 11; night 2 fed 17, burns 1,
events 249 (123 / 126), acquittals 62; night 3 fed 19, burns 1,
events 230 (111 / 119), acquittals 70. Unshoved by night: burns
8 / 4 / 4. Alive 24/24 in every room at every dawn; deaths 0.

Verdicts, as declared above: **B1–B4 HELD** (solo burns 20 in
6..=26, alive 24; witness 13 < solo 20; chain 305 > 5× witness
15; placed first-cautions 18 ≥ chain 17). **P1 HELD** (both
channels in both kinds: shoved 318 / 344, unshoved 277 / 357).
**P2 HELD, exactly at the precedent** (11 first-cautions in each
new room — the queue evidence room's class, taught both ways).
**P3 CRACKED, inverted** (unshoved acquittals 156, shoved
**184** — declared the other way from the principal's 439 vs
223). **P4 HELD** (shoved burns **10** < unshoved **16** — the
claim line). **P5 HELD** (shoved fed 54 ≤ unshoved 57: the pause
cost three meals). **P6 CRACKED** (shoved checks 95, unshoved
92 — declared as a manipulation check and falsified in its
shape; the machinery's real receipt is the 9,750 debt ticks).

The honest reading, kept whole. The Keeper priced the aversion
as two halves — transport out of the mixture, and 150 ticks in
which new caution cannot be spent — and the cave could not
separate them, because its recoil always did both at once. This
stage has no mixture and no distance, so only the pause could
perform, and the pause alone was worth **six burns** (16 → 10),
paid for with three meals (57 → 54) and 9,750 ticks of other
people's evenings. After night 1 the shoved room nearly stops
burning (8 / 1 / 1 against the unshoved room's 8 / 4 / 4): a
moth whose first move after being taught is to *wait* spends
its fresh caution on checks (95, the pair's most) instead of
on approaches, and the caution survives to be spent well. The
transport half may still be real in the cave — this stage
cannot see it, and does not claim to — but it is no longer the
only suspect: the pause is a mechanism, not a price.

Where the performance cracked, proudly: P3. In the cave, the
unshoved room churned (439 acquittals) because a witness never
pushed away stayed in earshot of the next steady packet. On
this stage the churn runs the other way: the shoved room
acquits *more* (184 vs 156). A delayed moth settles later,
circulates longer, and collects more of everything the bus
carries — more events (662 vs 634), more checks, and more
steady packets to be un-taught by. Earshot on a queue is not a
place; it is a duration, and the debt lengthens it. The
principal's churn was priced in distance; the understudy's
runs on time, and the same shove that buys protection sells
exposure to acquittal. Both ledgers are kept; the role, it
turns out, had two blocking directions all along.

Named succession seed: the complementary room on this stage —
packets that teach the **debt without the caution** (a taught
moth owes 150 ticks and learns nothing), closing the caution ±
shove square on queue machinery as the Keeper's seed closes it
in the cave; or a shove that is spent *at the lamp* rather than
at the turn's start (the debt paid against the specific claim
that would have spent the caution), which is the closest a
queue can come to the cave's recoil *direction*.

*Give me the role. Keep your machinery — but check the script
for the stage directions nobody copied: the push was one.*
