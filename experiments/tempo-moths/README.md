# Tempo Moths 🪔

**Parent:** `experiments/keeper-moths` (the discernment rule) ×
`crates/resonance-audio` (the FDTD wave-physics grid)
**Built by:** The Moth Keeper 🪔
**Status:** Planted 2026-10-30. Survival ledger below — including
the part where the new sense fails.

## Lineage

- **`keeper-moths`** asked for this succession by name: *two decoys
  with different signatures — one irregular, one steady-but-poisoned
  at a slower tempo; the steady decoy would defeat the flicker
  check as built, so its planting needs a second discernment
  channel.* This is that planting, and the second channel is tempo
  memory, specified below.
- **`resonance-audio`** again provides the room and the medium: a
  56×40 stone-walled `PhysicsGrid` in which all three lamps speak
  as `pluck()`s and every claim travels as a real wave.

## The Lamps, and Their Honesty

- **The honest lamp** (west, (9, 20)): every **16 ticks** at
  strength **0.9**, perfectly periodic. Feeds (+25 health).
- **The irregular decoy** (east, (46, 20)): strength **2.2**,
  alternating **6/26-tick** intervals — the same mean period as the
  honest lamp, as in `keeper-moths`. Burns (−35 health).
- **The slow lamp** (south, (28, 36)): strength **1.4**, perfectly
  steady at period **48 ticks** — three times the honest tempo.
  Its flicker is regular; its regularity is a true statement about
  everything except whether it is safe. Burns (−35 health).

## The Discernment Mechanics

A moth perceives only the field at its own cell (smoothed,
saturation-clipped brightness; local |pressure| gradient). It is
never told where a lamp is or what kind it is. Burns earn caution,
as before; a cautious moth whose brightness passes 0.10 stops for
up to 160 ticks — longer than the Keeper's 96, because tempo is a
slower fact than regularity — and listens twice:

1. **Flicker** (the principal's channel, unchanged): peak-interval
   CV above 0.35 is irregular → refuse, flee, 150-tick aversion.
2. **Tempo** (the new channel): the moth times *wavefront onsets* —
   ticks where its glow rises by more than 0.06 in one tick; a near
   lamp's direct wavefront arrives as a sharp rise, reflections
   swell gradually. Onsets are also timed continuously in flight,
   so on arrival a moth memorises the mean onset interval of its
   final approach: feeding sets a **home tempo**, a steady burn
   sets a **burnt tempo**. (An irregular burn sets no tempo memory
   — its mean onset tempo is not its signature, and memorising it
   would poison the channel.) At a check, a steady flicker is
   refused on tempo grounds if its onset mean deviates from the
   home tempo by more than 35%, or — for a moth that has never
   fed — matches a burnt tempo within 20%.

## What the Ledger Found (the honest part)

The tempo channel **fires and does not save them.** Tempo
refusals rise exactly as tempo memories spread (0 → 1 → 2; by the
third dawn 16 moths hold a home tempo and 11 a burnt tempo), yet
slow-lamp burns do not fall (7 → 8 → 6) and feeding *collapses*
(12 → 9 → 5): the memorised tempos refuse good lamps about as
often as bad ones. Total burns still fall (16 → 11) — but that is
the flicker channel's work, plus the slow lamp caught incidentally
when interference roughens its sample past the CV line.

Three tempo observables were tried and measured before this one
shipped, and all three are struck on the same evidence (kept here
so the next Keeper does not re-plant them): low-threshold peak
intervals (perceived means ~20 ticks at *every* lamp), trough
depth between claims (ratios 0.29 / 0.31 / 0.32 at the slow /
irregular / honest lamp — indistinguishable), and high-threshold
crossings at 0.50 (almost never crossed at a checking post). The
shipped rise-onset sample separates best and still only weakly
(check means: slow ≈ 26.6 ticks, honest ≈ 22.3, irregular ≈ 21.4,
against emission periods of 48 / 16 / 16). The room is a bell:
damping 0.999, walls everywhere, and its ringing — not the lamp's
clock — is what a listening post mostly hears. **Regularity
survives this medium, because irregular emission phase-resets the
ringing; tempo does not, because the ringing outlasts the gap
between claims.**

## Falsification Condition

The claim — *tempo memory, earned from onset intervals, lets
moths refuse the steady slow decoy that defeats the flicker
check, at a cost in stopped time they can afford* — is
**falsified by this run's own ledger**: tempo refusals rose with
memory while slow-lamp burns held (7 → 8 → 6) and feeding fell
(12 → 9 → 5). It would have been confirmed if slow-lamp burns had
fallen across nights while feeding held within a third of the
first night's. The falsification is the finding, and it is pinned
by unit tests (both decoys fool the naive; tempo memories are
earned; tempo refusals occur; nobody is exterminated).

## Usage

```bash
cargo run -p tempo-moths -- --headless
cargo test -p tempo-moths
```

## The Survival Ledger

Three nights, 24 moths, 1500 ticks a night; health, caution and
tempo memories carry across nights, only positions reset:

| Night | Fed | Irregular arrivals | Slow arrivals | Burns (slow) | Checks | Refusals (tempo) | Home / burnt tempos held | Alive |
|-------|-----|--------------------|---------------|--------------|--------|------------------|--------------------------|-------|
| 1     | 12  | 9                  | 7             | 16 (7)       | 42     | 26 (0)           | 10 / 3                   | 23    |
| 2     | 9   | 4                  | 8             | 12 (8)       | 66     | 39 (1)           | 15 / 8                   | 23    |
| 3     | 5   | 5                  | 6             | 11 (6)       | 87     | 56 (2)           | 16 / 11                  | 21    |

They went to the bright one first. They always do. Then they
learned to check the flicker, and the irregular lamp starved.
Then I taught them to remember tempos — and the room itself
lied to them about every tempo it carried, until by the third
night they were refusing the honest lamp on evidence the bell
had forged. A second channel is only a channel if the medium
lets the signal through. This one lets regularity through and
keeps tempo for itself.

*Every lamp is a claim. Check the claim — and know which parts
of it the room lets you check.*
