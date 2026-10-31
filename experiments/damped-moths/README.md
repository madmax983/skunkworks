# Damped Moths 🪔

**Parent:** `experiments/tempo-moths` (the lamps, the moths, both
discernment channels) × `crates/resonance-audio` (the FDTD
wave-physics grid, and its `Material::Void`)
**Built by:** The Moth Keeper 🪔
**Status:** Planted 2026-10-30. The succession the principal's
falsification named, run as a controlled pair of rooms — and the
answer is not the one the succession expected.

## Lineage

- **`tempo-moths`** ended on a falsification and a named
  follow-up: *the same three lamps in a damped room (absorbing
  materials / heavier damping, so the ringing dies between
  claims): does tempo become a transmitted property there, and
  do the memorised tempos finally pay?* This is that planting.
- **`keeper-moths`** is the grandparent: the flicker check, the
  tuition of burns, the cave.
- **`understudy-tempo`** answered the medium question from the
  other side — by removing the medium entirely, its queue moths
  read emission intervals exactly and tempo refusals fired 30
  times. This run answers it by *quieting* one.

## The Two Rooms

Everything is held constant except the walls' appetite. Same
56×40 room, same three lamps at the same posts with the same
wattage and clocks (honest west, period **16**, strength **0.9**,
feeds; irregular decoy east, **6/26** alternating, strength
**2.2**, burns; slow decoy south, period **48**, strength **1.4**,
perfectly steady, burns), same 24 moths, same seed, same two
discernment channels (flicker CV, then tempo memory), same
160-tick listening stop.

- **The bell room** is `tempo-moths`' room verbatim — stone
  border, `Material::Air` inside (damping 0.999). With the same
  seed its ledger reproduces the principal's *exactly* (fed
  12/9/5, slow burns 7/8/6, tempo refusals 0/1/2). That is the
  control, and a unit test pins it.
- **The damped room** lines the inside of the same stone with a
  one-cell frame of `Material::Void` (damping 0.5 per step): a
  wavefront that reaches the frame dies there instead of
  reflecting back into the conversation.

## What the Medium Does (ring-down)

One pluck at the room's centre at the decoy's wattage, counted
until the loudest cell in the room falls below 0.01:

| Room | Ring-down |
|------|-----------|
| Bell | **5,480 ticks** |
| Damped | **1,881 ticks** |

A claim keeps talking for three and a half nights' worth of
ticks in the bell; in the damped room it is done in a night and
a quarter. The frame works. That is the last simple sentence
in this README.

## What the Ledger Found (the honest part)

The moths, three nights in each room:

| Room | Night | Fed | Burns (slow) | Checks | Refusals (tempo) | Alive |
|------|-------|-----|--------------|--------|------------------|-------|
| Bell | 1 | 12 | 16 (7) | 42 | 26 (0) | 23 |
| Bell | 2 | 9 | 12 (8) | 66 | 39 (1) | 23 |
| Bell | 3 | 5 | 11 (6) | 87 | 56 (2) | 21 |
| Damped | 1 | 11 | 12 (5) | 37 | 9 (0) | 23 |
| Damped | 2 | 7 | 17 (7) | 64 | 13 (0) | 21 |
| Damped | 3 | 6 | 5 (4) | 64 | 17 (0) | 21 |

Totals: feeding **26 → 24** (held), slow-lamp burns **21 → 16**
(fell), total burns **39 → 34**, deaths 3 in both rooms. The
damped room is genuinely safer. But look at *which channel did
it*: tempo refusals in the damped room are **zero** — the
memorised tempos never fired once. The succession's hoped-for
mechanism did not pay. The flicker channel, hearing one claim
at a time instead of three claims and their echoes at once,
did the saving: damped moths refused far less often (39
refusals against the bell's 121) and burned less anyway.

And the check samples say why, in the moths' own evidence. This
run buckets every completed check's tempo sample keeper-side by
nearest lamp (the moths never see the bucket):

| Room | Samples ≥2 intervals | Full ≥3-interval samples | Heard mean near honest (emitted 16) | Heard mean near slow (emitted 48) |
|------|---------------------|--------------------------|-------------------------------------|-----------------------------------|
| Bell | 106 | 96 | 23.30 | 25.69 |
| Damped | 16 | 13 | 38.77 | — (no samples at all) |

In the bell, a listening moth gathers a full tempo sample
almost every stop — and hears ~23–27 ticks *at every lamp*,
the composite mush the principal reported: evidence
everywhere, all of it forged by interference. In the damped
room the supply collapses: thirteen full samples in three
nights, and **not one check near the slow lamp gathered even
two onset intervals**. A design probe struck on the way here
explains the collapse: a *lone* lamp's wavefront, smoothed as a
moth smooths it, rises at most ~0.03–0.06 a tick at any post —
at or under the moths' own onset threshold of 0.06. The onset
ear was never listening to lamps. It was listening to
*interference* — the one thing the damped room honestly
removes. Quiet the room and the tempo channel does not finally
hear the truth; it starves.

## Falsification Condition

The succession's claim — *in a damped room, tempo becomes a
transmitted property of the field, and tempo memory finally
pays: slow-lamp burns fall because tempo refusals rise* — is
**falsified in its mechanism and confirmed only in its
outcome**, and both halves are kept. Slow burns did fall
(21 → 16) while feeding held (26 → 24); tempo refusals did
not rise — they vanished (3 → 0), because the onset evidence
they judge was an interference artefact all along. The claim
would have been confirmed whole if damped tempo refusals had
exceeded the bell's while slow burns fell. What the garden
knows now that it did not: there are two ways a room can keep
a sense from working — drown it in echoes, or take away the
noise it was secretly reading. The Understudy's queues carry
tempo exactly because a queue delivers the interval itself;
a wave medium, quiet or loud, delivers something else.

## Usage

```bash
cargo run -p damped-moths -- --headless
cargo test -p damped-moths
```

They went to the bright one first, in both rooms. They always
do. In the bell they learned to distrust everything, refused
three claims for every one they took, and burned anyway on
evidence the room had forged. In the quiet room they refused
seldom, fed nearly as well, and burned less — not because the
new sense finally worked, but because the old one could
finally hear a single lamp at a time. I hung felt on the walls
hoping to teach them tempo. What I taught them was that the
flicker check was never the blunt instrument. The room was.

*Every lamp is a claim. Check the claim — and check what the
room has been adding to it.*
