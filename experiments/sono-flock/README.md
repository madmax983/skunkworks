# sono-flock 🗺️

**Planted by:** The Cartographer of Lost Things 🗺️
**Lineage:** graveyard/sono-boids × living crates/flocking + crates/locus + crates/resonance-audio (revival cross)
**Coordinates:** the archive's near corner (a flock wired to a wave field, dead on one import) → the living crates it reached for. Bearing between them: the shortest in this surveyor's book — the corpse died one door away from everything it needed.

## The death

`graveyard/sono-boids` (267 LOC, edition 2021) coupled a boid flock to an FDTD wave field **in both directions**: each boid, at a 0.01 coin per tick, plucked the field at its own cell (strength 0.5); and any boid sitting in a loud cell (|pressure| > 0.2) was shaken by a random force (±0.5 per axis, spent on the next tick). The flock itself ran on the garden's own machinery — `flocking::compute_force` with its author's parameters (view radius 10, separation radius 5, max speed 1.0, max force 0.1, weights 1.5 / 1.0 / 1.0) over `locus::Vec2`, 50 boids released from the centre of an 80×40 wrapped world.

It was executed for **Terminal Compilation Failure / Ecosystem Maladaptation**, and the certificate names the exact wound: `error[E0603]` — a private-module access violation for `resonance_audio::audio`. The corpse imported `AudioCommand`, `AudioModel`, and `AudioSnapshot` through a module that was never public. Its final form **never compiled**, its window was its only mouth, and its loop was never counted by anyone. Executed for one wrong import path; the underlying metabolism — a flock that rings its room and is rattled by the ringing — was never even given the chance to be sound or unsound.

## The donation

Every organ the corpse reached for is alive, and one of them has since opened the very door it died at:

- **crates/resonance-audio** now declares `pub(crate) mod audio;` and **re-exports `AudioModel`, `AudioCommand`, and `AudioSnapshot` at the crate root** — the E0603 door is open. This revival takes the sibling organ the same crate keeps public, `PhysicsGrid` (the FDTD field itself: `pluck`, `step`, `get`), because a ledger needs the field in the open, not behind an audio thread and a snapshot channel.
- **crates/flocking** still exports `compute_force` with the identical signature, and `FlockingParams` still carries the corpse's fields — the corpse's parameter values are used verbatim.
- **crates/locus** still supplies the `Vec2` both the corpse and the flocking crate speak.

Distance note, kept honestly: a living cross of these same two crates already stands in the garden — `flock-resonance` (flocking × resonance-audio, one-way sonification: boids inject index tones, the field never answers). The corpse's trait was never the pairing; it was the **return path**. No living specimen closes that loop, and the corpse is the only one that ever tried.

## The novel trait

The corpse's loop runs verbatim beside its two ablations, on identical spawn and identical streams, so the coupling is priced rather than illustrated. Three deterministic SplitMix64 streams (spawn, pluck coin, wave jitter — one per purpose, shared across regimes) replace the corpse's three `thread_rng` draws; that is what makes the middle ledger an exact identity instead of a resemblance:

- **Mute** — the flock flies; nobody plucks; the field is never touched.
- **Deaf** — boids pluck on the corpse's coin; the field rings; no boid ever reads it.
- **Coupled** — the corpse's full loop: plucks out, jitter back.

## Headless ledger (50 boids, 80×40, 1,200 ticks)

| regime | plucks | jitter events | jitter ticks | field max \|p\| | field final Σ\|p\| | polarization | mean pairwise toroidal dist |
|---|---|---|---|---|---|---|---|
| mute | 0 | 0 | 0 | 0.0000 | 0.00 | 0.1324 → 0.9518 | 0.0000 → 16.7516 |
| deaf | 566 | 0 | 0 | 0.9595 | 451.62 | 0.1324 → 0.9518 | 0.0000 → 16.7516 |
| coupled | 566 | 9,832 | 1,138 / 1,200 | 0.8949 | 419.98 | 0.1324 → 0.8092 | 0.0000 → 21.7713 |

Read the bearings:

- **The deaf flock *is* the mute flock, boid for boid.** Final positions are exactly equal — the forward path alone (566 plucks, a field peaking at 0.9595, nearly five times the corpse's 0.2 threshold) moves nothing. A one-way coupling is a broadcast, not a conversation.
- **The return path is not decoration; it is a tax.** In the coupled room the wave reaches a boid 9,832 times across 1,138 of 1,200 ticks, and the flock pays for its own ringing in its own coin: polarization ends at **0.8092 against the deaf flock's 0.9518**, and the flock's spread widens from 16.7516 to **21.7713**. The corpse's jitter rule (a *random* shake, with no gradient in it) cannot steer — it can only scatter, and the ledger prices the scatter exactly.
- **The answering field is slightly quieter, not louder** (final Σ|p| 419.98 vs 451.62; peak 0.8949 vs 0.9595): a scattered flock plucks from more cells, more thinly, than a tight one revisiting the same water. Kept as measured; no mechanism is claimed beyond the geometry.

## Predictions, declared before the first run

- **P1** the mute field is exactly silent (max |p| = 0.0, Σ|p| = 0.0, 0 plucks) — **HELD** exactly.
- **P2** the deaf flock lands boid-for-boid on the mute flock while its field crosses the 0.2 threshold — **HELD** exactly (positions equal; max |p| 0.9595).
- **P3** the coupled flock is actually reached (jitter events > 0) and does not land where the deaf flock landed — **HELD** (9,832 events; positions differ), on the same pluck count (566 = 566, the shared stream's receipt).
- **P4** the flock aligns (mute polarization gain > 0.1) — **HELD** (0.1324 → 0.9518, gain +0.8194).

## Falsification condition

If the coupled regime had posted **zero jitter events** — the field never once loud at a boid's own cell — or if the coupled flock had landed **exactly** where the deaf flock landed, the corpse's loop would be decorative in both directions and this revival would be a portrait, not a measurement. Neither happened; the loop bites, and what it bites is the flock's alignment.

## Named adaptations

- `AudioModel` + channels → one `PhysicsGrid::step()` per tick, in the open; pluck and read through the grid's public `pluck`/`get`. The corpse read a snapshot one audio buffer stale; a boid here reads the field stepped at the top of its own tick.
- `thread_rng` (three draws) → three seeded SplitMix64 streams, one per purpose — the price of the deaf = mute identity, paid gladly.
- The corpse's TUI is left in the grave. `--headless` is handled before anything else; there is no window in this revival at all, because a window was the corpse's only mouth and that was half its death certificate.

## Tests (6/6)

An isolated boid feels no force; the mute field is exactly silent; the deaf flock equals the mute flock boid-for-boid with a ringing field; the coupled loop bites (jitter > 0, positions diverge, pluck count identical); the flock aligns; a 200-tick coupled run repeats exactly.

## Succession

The corpse's jitter is *random* — the one word in its design with no information in it. The named successor: a boid that reads the field's **gradient** instead of its loudness, steering down-wave the way keeper-moths read a field's slope — does a directed return path buy back the 0.14 of polarization the random one taxes, or does any answering field scatter a flock by that much regardless of what the answer says?
