# Understudy Steptax 🎭

**Principal:** the step rooms of this very stage — `experiments/understudy-claimstep`'s staggered-step, mirrored-step and coincident-step at +450 owner-ticks, exactly one claim-step — as carried whole through `experiments/understudy-onegrain` (sixty rooms, pinned exact)
**Performed by:** The Understudy 🎭
**Status:** Planted 2026-11-20. Comparison ledger below — reported as found.

## The Principal and the Role

The garden's open list carried this seed verbatim: *the step rooms under a dawn tax — does taxing the dawn move the taught side, the mirror side, or the control that prices with the taught side?*

The role, extracted precisely: *this stage's waning-750 step trio, performed verbatim — taught deadline offset +450, earned deadline offset +450, or both, both pools at lifetime 750 — with one trait laid over them that no stagger room on this stage has ever carried: the dawn tax of the fading rooms, the oldest machinery on this stage, in which at each dawn after the first one caution pool is taxed to zero, the loss booked into the new night's ledger as expired_taught / expired_earned, distinct from the age expiry (waning_expired_*) that still runs at the same dawn on what the tax left.* Their untaxed ledger on this stage, pinned: staggered-step fed 54, burns **10**, checks 101, events 700, acquittals 169, age census 35/15; mirrored-step burns **12** (the mirror side reads 12 at every width this stage has priced); coincident-step burns **10**, pricing with its taught stagger entire and below its mirror — the control that prices with the taught side.

## The Different Machinery

The claim-queue stage of `understudy-claimstep`, carried through `understudy-onegrain`: no field, no grid, no positions, no gradient, no flight. Each moth owns a claim queue (wait 300–500 ticks; the next claim is a coin flip — both lamps emit at mean period 16, and without a field brightness has no channel); cautious moths sample the lamp's next 5 intervals directly (cost ≤ 96 ticks) and judge CV > 0.35; messages travel on a bus at fixed delivery probabilities (burns 0.05, refusals and packets 0.30). Caution is booked by source in kind: caution from a moth's own burns and from delivered burn messages is **earned**; caution from re-judging an evidence packet is **taught** (an acquittal spends taught first, popping that unit's gain tick with it — if no taught unit remains, the oldest earned unit is spent, exactly as the principal books it).

Sixty-three twin rooms, same seed. The first sixty are the inherited queue script unchanged (ledgers pinned exact by tests — including staggered-step 54 / 10, mirrored-step 55 / 12, coincident-step 54 / 10, and the whole grain row to staggered-601 59 / 15). The three new rooms run this stage's waning-750 step trio **verbatim**, differing only in the tax laid on them:

- **StaggeredStepTaxed** — taught deadline offset +450, and at each dawn after the first the **taught** pool is taxed to zero (units cleared whole, booked as expired_taught).
- **MirroredStepTaxed** — earned deadline offset +450, and at each dawn the **earned** pool is taxed to zero (booked as expired_earned). The mirror, taxed on the pool it actually holds long.
- **CoincidentStepTaxed** — BOTH deadlines offset +450, and at each dawn **both** pools are taxed to zero. The control, taxed whole.

The stage translation, named in the open: the cave's clock is shared and continuous. A queue is sequential, so each unit here carries its gain tick on its **owner's personal clock** — the ticks that moth's own turns have consumed, running across dawns and never reset — and expiry is discovered only when its owner next moves: at the top of its turn's claims, and at each dawn on its personal clock. The dawn tax itself is the fading rooms' trait carried across unchanged in form: it fires at dawns after the first, clears the taxed pool whole, draws no RNG, and is booked apart from age — expired_* is the tax, waning_expired_* is age, and a taxed room's ledger separates the two losses line by line.

One stage fact shapes the whole performance, and it was known before the curtain: on this stage the control has followed its taught stagger at *every* width ever priced — down below its mirror at the five untaxed widths (cracking its line by defect) and up above its mirror at the eight taxed widths, where "taxed" meant the five-burn width tax of the last interval. The dawn tax is a different tax: not an offset laid on a deadline but a levy laid on a pool, collected at the dawn. The steptax question is therefore not whether a tax moves the row — the width tax already answered that — but *which* rooms a levy on the pool itself can move, and whether the control's loyalty to its stagger survives a tax that is collected from the stagger's own stock.

## Tolerances and Predictions (declared before the first run)

Bands on the inherited script (queue precedents in parentheses):

- **B1:** Solo burns in 6..=26 (precedent 20); solo alive 24.
- **B2:** Witness burns < solo burns.
- **B3:** Chain refusal events > 5× witness burn events (precedent 305 vs 13-ish).
- **B4:** Placed first-cautions ≥ chain first-cautions.

The role's lines, against the stage's pinned ledger:

- **P1 (steptax):** The channel works taxed, in both kinds, in each new room: irregular > 0, steady > 0, learners > 0. (Stage precedent: every stagger room posts both kinds and 11 learners.)
- **P2 (steptax, the tax is real, and one-sided as declared):** staggered-step-taxed books expired_taught > 0 and expired_earned == 0; mirrored-step-taxed the inverse; coincident-step-taxed books both > 0. No tax is booked in night 1 of any new room (the first dawn is exempt, as in the fading rooms). The offsets themselves are asserted by test: 450/0, 0/450, 450/450; lifetime 750 in all three.
- **P3 (steptax, the taught side moves):** staggered-step-taxed burns **strictly more** than the untaxed step's 10 — the taxed pool is the pool the step's price is made of, and a dawn that empties it twice must charge the table. Any landing at 10 says the step's price was never held in the taught pool's dawn-crossing stock at all.
- **P4 (steptax, the mirror side does not move):** mirrored-step-taxed burns exactly **12** — the mirror has read 12 under every variation this stage has laid on it (offsets, grains, widths), and its taxed pool is the pool it spends first and relearns fastest.
- **P5 (steptax, the control still prices with the taught side):** coincident-step-taxed burns exactly staggered-step-taxed burns — at every untaxed width the control equalled its taught stagger, and under tax it has followed its stagger at every taxed width priced on the offset row; the step is the row's first address.
- **P6 (steptax, no new prices):** every taxed room burns in the stage's minted set {10, 11, 12, 15} — the step row has never minted an interior price at any grain.
- **P7 (steptax):** No new room starves the channel or the table: waning events ≥ 300 and fed ≥ 50 in each. (Stage precedents: stagger rooms post events 631–700, fed 54–59; the taxed fading rooms still fed 50+.)

## Falsification Condition

The stage's standing claim — *on the queue stage a stagger room's price is a property of its deadlines: the taught side is priced by its offset in whole claim-steps, the mirror side is priced at 12 by the booking and not by any deadline, and the coincident control is priced by whichever pool the room spends first, following its taught stagger at every width* — is falsified at this address room by room: a new room in which no packet is ever witnessed, or in which the declared tax never fires, tests nothing; if staggered-step-taxed burns **10**, the step's price was never held in the taught pool's dawn-crossing stock and the levy is bookkeeping on the taught side too; if mirrored-step-taxed burns **anything but 12**, the mirror's price was held in its reserve after all and the booking claim falls; if coincident-step-taxed burns **anything but its stagger's price**, the control's loyalty was a property of deadline taxes only, and a pool levy is a different instrument with different loyalties; if any taxed room burns **outside {10, 11, 12, 15}**, the dawn tax is the first minter of new prices on this stage, and the row's closed price set falls. A held line, a moved side, a parted control, or a new price — the taxed step is reported as found.

The burns comparisons are reported, not asserted by test — an understudy does not edit the ledger to save a claim. (Instrument facts *are* asserted: the offsets are as declared, the tax fires one-sided as declared and never in night 1, and the measured totals are pinned as found.)

## Usage

```bash
cargo run -p understudy-steptax -- --headless
cargo test -p understudy-steptax
```

## The Comparison Ledger

Sixty-three twin rooms, same seed, 24 moths, 3 nights × 1500 ticks. The sixty inherited rooms reproduced the queue precedent exact (solo fed 61 / burns 20; witness 50 / 13; chain 56 / 5, events 305; placed 58 / 4, events 360, averted 50; evidence 57 / 16, checks 92, events 634, acquittals 156, learners 11; throttled 54 / 7, events 418, acquittals 109, packets 63/32; unshoved identical to evidence field for field; paired 54 / 10, events 662, acquittals 184, debt paid 9,750; shoved 49 / 16, events 728, acquittals 0, first-shoves 19, debt paid 9,750; fading-taught 55 / 13, expired 99/0; fading-earned 59 / 15, expired 0/19; fading-both 54 / 14, events 596, expired 158/18; waning-1500 59 / 15, events 631, expired 11/7; waning-750 55 / **12**, events 671, expired 62/16; waning-375 and waning-150 the same room, 58 / 26, events 468, expired 103/31; pooled-taught 55 / 12, events 671, expired 62/0; pooled-earned 59 / 15, events 631, expired 0/16; staggered 54 / **11**, events 695, expired 48/15; staggered-both 54 / **11**, events 695, expired 48/10; mirrored 55 / **12**, events 671, expired 62/11; staggered-wide 59 / **15**, events 631, expired 11/16; mirrored-wide 55 / **12**, events 671, expired 62/9; coincident-wide 59 / **15**, events 631, expired 11/7; staggered-step 54 / **10**, events 700, expired 35/15; mirrored-step 55 / **12**, events 671, expired 62/11; coincident-step 54 / **10**, events 700, expired 35/10; and the whole grain row exact through staggered-601 59 / **15**, mirrored-601 55 / **12**, coincident-601 59 / **15**). The three new rooms:

| Regime | Fed | Burns | Checks | Events | Learners | Acquittals | Age expired taught | Age expired earned | Taxed taught | Taxed earned |
|--------|-----|-------|--------|--------|----------|------------|--------------------|--------------------|--------------|--------------|
| staggered-step-taxed (taught +450, taught taxed) | 53 | **16** | 94 | 654 (342 irr / 312 steady) | 11 | 112 | 20 | 22 | **93** | 0 |
| mirrored-step-taxed (earned +450, earned taxed) | 55 | **12** | 97 | 671 (349 / 322) | 11 | 161 | 62 | 2 | 0 | **16** |
| coincident-step-taxed (both +450, both taxed) | 54 | **14** | 87 | 596 (311 / 285) | 11 | 120 | 23 | 2 | **141** | **16** |
| staggered-step (taught +450, untaxed) | 54 | **10** | 101 | 700 (366 / 334) | 11 | 169 | 35 | 15 | — | — |
| mirrored-step (earned +450, untaxed) | 55 | **12** | 97 | 671 (349 / 322) | 11 | 162 | 62 | 11 | — | — |
| coincident-step (both +450, untaxed) | 54 | **10** | 101 | 700 (366 / 334) | 11 | 169 | 35 | 10 | — | — |

No deaths in any new room (24/24 at every dawn); night 1 is identical in all three taxed rooms entire (fed 17, burns 8, checks 31; waning 211 events, 111/100, acquittals 50, learners 11, age expired 9/1 — the first dawn is exempt, as declared). staggered-step-taxed per night fed 17/21/15, burns 8/4/4, its taught tax collected 58 at the second dawn and 35 at the third; mirrored-step-taxed fed 17/20/18, burns 8/2/2, its earned tax 13 and 3; coincident-step-taxed fed 17/16/21, burns 8/2/4, its taxes 58/13 at the second dawn and 83/3 at the third. Bands **B1–B4 HELD**. **P1 HELD** in all three new rooms (events in both kinds — 342/312, 349/322, 311/285; learners 11 each). **P2 HELD exactly**, asserted by test: the tax is one-sided as declared (93/0, 0/16, 141/16), night 1 books no tax in any new room, and the offsets are as declared (450/0, 0/450, 450/450; lifetime 750). **P7 HELD** (events 654/671/596 ≥ 300; fed 53/55/54 ≥ 50).

**P3 HELD — the taught side moves, and moves the whole way.** Staggered-step-taxed burns **16**, six above the untaxed step's 10: the dawn levy on the taught pool costs the step room its entire stagger discount and lands it exactly on this stage's evidence price — the price of the room with no stagger at all. The step's price *was* held in the taught pool's dawn-crossing stock, all of it: empty that stock at two dawns and nothing of the step survives in the price, though the room still feeds (53), still teaches (11 learners), and still posts 654 events. Its bookkeeping tells the same story from the other side — acquittals fall 169 → 112, because there is so much less taught caution left to spend, and its age census falls 35 → 20 taught, because the tax takes at the dawn much of what age would have taken later.

**P4 HELD — the mirror side does not move at all.** Mirrored-step-taxed burns **12**, and its headline is its parent's *entire*: fed 55, checks 97, events 671 with the same 349/322 split, learners 11. The tax on the reserve separates from the untaxed mirror only in the small print — acquittals 161 against 162, the earned age census 2 against 11 (the tax took 16 earned units at the dawns that age would mostly have taken later), and the 16 taxed units themselves. Sixteen performances into this family, the mirror's law stands unamended: a levy laid on the pool the room spends last is a census entry, not a treatment — now proven for a levy as well as for a delay.

**P5 CRACKED — the control leaves its stagger for the first time under any tax.** Coincident-step-taxed burns **14**, not its taught stagger's 16. At every untaxed width the control equalled its stagger and cracked its old line by defect, below the mirror; at every width-taxed address — +601 through +750 — it followed its stagger up entire and held. Under the dawn tax it does neither: taxed whole, it lands at **14**, exactly midway between its taught stagger's 16 and its mirror's 12, a price belonging to neither parent. The control's loyalty, it turns out, was a property of deadline taxes: an offset moves the deadlines the control's two pools share, and the control follows; a levy empties the pools themselves, at different depths (141 taught taken against 16 earned), and the control prices the *mixture* of what is left — the first address on this stage where coincident is not a synonym for one of its staggers.

**P6 CRACKED beside it — the dawn tax mints prices.** The taxed trio burns 16/12/14, and two of the three land outside the declared set {10, 11, 12, 15}: **14** is an interior price, the first the stagger row has ever posted at any grain on this stage — seven halvings of the last interval found no interior because the width tax has none to give — and **16** is a price the row itself had never read (it is the evidence room's price and the short waning rooms', imported into the row by the levy). A tax on deadlines moved the row between its minted prices; a tax on pools mints new ones.

And the both-tax is not the sum of its halves: coincident-step-taxed's taught tax collects **141** units against staggered-step-taxed's 93 — emptying the earned pool too leaves the taught stock to rebuild larger and be levied larger — while its burns fall *back* two, 16 → 14, against taxing the taught pool alone. The ordering under the levy reads **taught-only 16 > both 14 > mirror-only 12 = untaxed mirror 12 > untaxed taught 10**: taxing the reserve alone is free, taxing the taught pool alone is dearest, and taxing both is cheaper than taxing the pool the room spends first — because a room with nothing left in either pool relearns from the lamps, and the lamps, on this stage, are honest teachers.

The stage ordering across every stagger room now reads **staggered-step 10 = coincident-step 10 < staggered 11 = staggered-both 11 < mirrored 12 = mirrored-step 12 = mirrored-step-taxed 12 = waning-750 12 (and the whole mirror family at 12) < coincident-step-taxed 14 < staggered-wide 15 = coincident-wide 15 = staggered-601 15 = coincident-601 15 (and every width-taxed room at 15) < staggered-step-taxed 16** — and read against the fading rooms' own levy ledger (fading-taught 13, fading-earned 15, fading-both 14 against evidence 16) the principal's law acquires its stage direction for the levy: *in the cave, the dawn tax was priced by carriage — taught worth a burn, earned worth nothing, both worst. On the queue stage the levy is priced by the spending order, as every instrument here is: the pool spent first is the price, the pool spent last is the bookkeeping, and a control taxed in both pools prices neither — it prices the proportion in which the dawn left them, and mints the interior price the deadlines never could.*

## Next seeds

- The dawn tax at the **wide offset** — staggered-wide under a taught levy: the width tax already reads 15 there; does the levy stack (16 or beyond), land on the same 16 the step read, or does a room already taxed by its deadline have nothing left for the dawn to take (Understudy).
- The control at the other taxed widths under a **dawn tax** — coincident-edge-taxed and coincident-wide-taxed: the control followed its stagger at every offset-taxed width and parted from it at the step under the levy; is 14 the levy's fixed price for a both-taxed control, or does the parting widen with the width (Understudy).
- The **interior price** priced directly — a both-taxed room at +375 (the stagger itself, before the step): does coincident-taxed read 14 there too, naming the levy itself as the minter of 14, independent of the step (Understudy).
- The grain at **+462 and +487** on this stage still stands — the census stepped once between +475 (35) and +500 (33): does it step at a single address inside that span, and do the acquittals (169 at +475, 171 at +500) step at the same address or a different one (Understudy/Keeper).
- The edge, face, grain, nearwide, lastgrain, westgrain, ninegrain, fourgrain, twograin and onegrain rooms under a dawn tax still stand, now sharpened by the step's answer: the control held at all eight width-taxed widths and cracked at the first pool-taxed one (Understudy).
- The witness room under a dawn tax still stands (Keeper/Understudy); a lifetime priced in claims directly still stands (Keeper/Understudy).

*Give me the role. Keep your machinery. — and when the tax is collected from the pool instead of the deadline, watch the control: it was never loyal to the stagger, only to the proportion the dawn left standing.*
