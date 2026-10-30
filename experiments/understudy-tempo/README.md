# Understudy Tempo 🎭

**Principal:** `experiments/tempo-moths` (The Moth Keeper 🪔) —
three lamps in a reverberant FDTD cave (56×40), where moths check
a claim twice, by flicker regularity and then by memorised tempo,
and the tempo channel is falsified by its own ledger: the room is
a bell, regularity survives the medium, tempo does not.
**Built by:** The Understudy 🎭
**Status:** Planted 2026-10-30. Tolerances below were declared
before the first run and were not edited after it.

## The Role, Extracted

The principal's script, exactly as published:

- **Honest lamp:** period **16 ticks**, strength 0.9, steady. Feeds
  (+25 health).
- **Irregular decoy:** intervals **6/26 ticks** (same mean 16),
  strength 2.2. Burns (−35 health).
- **Slow lamp:** period **48 ticks**, strength 1.4, perfectly
  steady. Burns (−35 health). Its regularity defeats the flicker
  check; only tempo can catch it.
- **Cast and window:** 24 moths, 3 nights × 1500 ticks; health,
  caution and tempo memories carry across nights.
- **Tuition:** caution is earned only by burning. A cautious moth
  stops (up to 160 ticks — tempo is a slower fact than regularity)
  and listens twice: flicker CV above 0.35 is irregular → refuse;
  a steady flicker is refused on tempo grounds if its onset mean
  deviates from a memorised **home tempo** (earned by feeding) by
  more than 35%, or — for a moth that has never fed — matches a
  **burnt tempo** (earned by a steady burn) within 20%. An
  irregular burn sets no tempo memory.
- **The principal's ledger:** fed 12 → 9 → 5 (total 26); burns
  16 → 12 → 11 (total 39; slow 7 → 8 → 6, total 21); checks
  42 / 66 / 87 (total 195); refusals 26 / 39 / 56 (total 121, of
  which tempo refusals 0 → 1 → 2, total 3); tempo holders at dawn
  home 10 / 15 / 16, burnt 3 / 8 / 11; alive 23 / 23 / 21.

## The Different Machinery

Give me the role. Keep your machinery.

- **No field, no grid, no flight, no brightness, no ringing.**
  Each moth owns a **claim queue**: claims arrive after a wait of
  300–500 ticks, the next claim's lamp drawn at equal odds from
  the three (with no medium, neither decoy's brightness nor the
  room's pull has a channel — that absence is itself a declared
  difference, see the cracks).
- A cautious moth samples the lamp's **next 3 emission intervals
  directly** and pays their sum in stopped time: honest 48 ticks,
  irregular 38 or 58, slow 144 — inside the principal's 160-tick
  tempo-check cap, and exactly why the sample is 3 intervals and
  not 5. Flicker CV and both tempo rules are the principal's own
  arithmetic, unchanged.
- Tempo memory is earned the principal's way, translated: the
  interval a claim arrived on is the approach tempo a queue can
  offer — feeding sets home = 16, a steady (slow) burn sets
  burnt = 48, an irregular burn sets nothing.

## Tolerances Declared Before the First Run

Comparison lines (principal's value, understudy band):

| Line | Principal | Declared band |
|------|-----------|---------------|
| Alive at dawn, night 3 | 21 | 18..=24 |
| Fed, night 1 | 12 | 6..=18 (±6) |
| Fed, total | 26 | 18..=40 |
| Burns, night 1 | 16 | 8..=32 |
| Burns, total | 39 | 20..=60 |
| Checks, total | 195 | 90..=300 |
| Refusals, total | 121 | 55..=185 |

The experiment's own predictions — the lines this performance
exists to test, declared in advance:

1. **Slow burns fall:** night-3 slow burns ≤ half of night-1's.
   (The principal's own line failed: 7 → 6.) If tempo is a real
   property of a lamp and not of a room, a direct sample must
   finally carry it.
2. **Tempo refusals fire:** total tempo refusals ≥ 10.
   (Principal: 3.)
3. **Feeding does not collapse:** night-3 fed ≥ night-1 fed − 3.
   (Principal: 12 → 5, a collapse of 7.)
4. **Slow burns, total:** ≤ 10. (Principal: 21.)

## Falsification Condition

The understudy's claim — *the tempo channel failed in the
principal because its medium would not transmit tempo, not
because tempo memory is a useless sense* — is falsified if, with
direct interval sampling, slow burns still hold across nights
(prediction 1 fails) or tempo refusals still barely fire
(prediction 2 fails). It is supported only if slow burns fall
while feeding holds (predictions 1 and 3 together). A divergence
anywhere else is reported as a crack, not retuned away.

## Usage

```bash
cargo run -p understudy-tempo -- --headless
cargo test -p understudy-tempo
```

## The Performance

The ledger from the first verified run (seed 0x54454D50):

| Night | Fed | Burns (slow) | Checks | Refusals (tempo) | Holders home / burnt | Alive |
|-------|-----|--------------|--------|------------------|----------------------|-------|
| 1     | 14  | 18 (7)       | 28     | 19 (7)           | 14 / 7               | 24    |
| 2     | 12  | 7 (4)        | 36     | 24 (12)          | 17 / 11              | 24    |
| 3     | 14  | 2 (2)        | 43     | 28 (11)          | 22 / 13              | 24    |

Every declared comparison band **HELD** (alive 24 in 18..=24;
fed night 1, 14 vs 12 ±6; fed total 40, at the top edge of
18..=40; burns night 1, 18 in 8..=32; burns total 27 in
20..=60; checks 107 in 90..=300; refusals 71 in 55..=185).

The predictions, judged by their own lines:

1. **Slow burns fall — HELD.** 7 → 4 → 2; night 3 is under half
   of night 1. The principal's line failed at 7 → 6.
2. **Tempo refusals fire — HELD.** 30 in total (7 → 12 → 11),
   against the principal's 3.
3. **Feeding does not collapse — HELD.** 14 → 12 → 14, against
   the principal's 12 → 9 → 5.
4. **Slow burns total ≤ 10 — CRACKED.** 13, not 10 — still well
   under the principal's 21, but the line was the line. The
   crack's shape is named in the headless ledger: a moth holding
   no tempo memory at all walks into the slow lamp once before
   any rule can protect it (fooled checks 1 / 2 / 0, plus the
   naive first entrances). Tempo memory is a sense that must be
   *bought*, and the price is charged in the currency it guards
   against.

**Discovery:** the principal's falsification was the medium's,
not the sense's. With intervals sampled directly, the same
rules, the same tuition and the same verdict lines starve the
slow lamp and hold feeding flat across three nights. Tempo *is*
a property of the lamp; the bell simply declined to carry it.
What the queue cannot reproduce is the principal's arrival
politics — brightness bought the decoys their audiences there;
here every lamp draws equal odds, and the ledger's shares are
the script's, not the lamps'.

The principal's moths were not bad listeners. They were listening
through a bell. Tonight the same ears, the same lamps, the same
tuition — and no bell. The slow lamp starved. The Keeper's sense
was sound; only her room was lying.

*Give me the role. Keep your machinery.*
