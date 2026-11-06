# Scarecrow Crowd 🌾

**Host:** `experiments/sono-flock` (The Cartographer of Lost Things 🗺️), as instrumented by `experiments/scarecrow-replay` — boids flying the parent crates' machinery: `crates/flocking` (`compute_force`, the host's parameters verbatim), `crates/locus` (`Vec2`), and `crates/resonance-audio` (`PhysicsGrid`, 80×40 FDTD wave field). Boids pluck the field (coin 0.01, strength 0.5); a boid standing in a cell louder than 0.2 is jittered. The host's constants, streams (three SplitMix64, one per purpose), and update order are copied faithfully and unchanged; the only variable in this planting is the flock's size. Tests pin this crate's 50-boid ledger to the host's published one.

**Principle:** Journal and replay — Harvest's rule that work which is written down can be stopped, stored, and replayed, and the replay is the same work. Learned from Harvest's durable execution: the journal is the run; everything else is derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-05. The comparison and predictions below were declared before the first run and were not edited after it. Verdicts are at the bottom, as found.

## The fragile point

`scarecrow-replay` journaled the coupled flock at the host's own density — 50 boids — and the full journal replayed exact. But its P3 cracked: the *naive* replay, which reads the journal as if a tick were an instant and applies each tick's strikes at the end of the tick, was also exact. Across 566 strikes, not one was heard within its own tick. The channel is real in the machinery — `PhysicsGrid::pluck` adds to the field's present level at once (`u[idx] += strength`), and boids read the field interleaved in flock order, each after its own move — so a strike can, in principle, be heard by a later boid in the same tick, and only the journal's within-tick order (its boid index) preserves that hearing. At 50 boids in 3,200 cells, no later boid ever stood in a struck cell in the strike's tick with the strike carrying its reading over the 0.2 line. The replay README named the condition and the next survey in one sentence: the boid index is insurance this flock never claimed; *a denser flock is the next survey.*

This planting is that survey. A seed that only sprouts in an empty field has not been tested. Put more birds in the field.

## The survey

Three crowds, one field (80×40, 1,200 ticks, every host constant unchanged): **50** boids (the host's own crowd, the anchor), **200**, and **400**. Each crowd runs three ways:

- **Live** — the host's coupled run, unchanged, writing the journal.
- **Replay** — no pluck coin exists in this run; the journal strikes the field, each strike at its own boid's turn.
- **Naive** — the same journal, strikes applied at the *end* of their tick.

And on the live run, a counter stands on the channel itself. At each boid's jitter read, the run compares the value heard against the value that would have been heard without this tick's strikes so far (kept exactly: a per-tick per-cell tally, and a strike adds exactly +0.5, with no field step intervening within a tick):

- **contact** — the boid's cell was struck earlier this same tick.
- **decisive** — the boid hears the field *because* of the same-tick strikes: |heard| > 0.2 while |unstruck| ≤ 0.2. A decisive contact is a hearing the naive replay cannot reproduce at that turn: its jitter stream draws differently from that tick on.
- **masked** — the strikes *silenced* a hearing: |heard| ≤ 0.2 while |unstruck| > 0.2. A +0.5 strike laid over a negative wave can pull a loud cell back under the line; the channel can take hearings away as well as give them.

The journal widens in a crowd: past 255 birds a boid index no longer fits in one byte, so an entry here is (tick u16, boid u16, x u8, y u8) — **6 bytes**, one more than the sparse field's 5. Density taxes the journal's name field first.

## The declared comparison

Predictions, declared before the first run:

- **B1 (host check):** the 50-boid crowd reproduces the host's published ledger exactly — 566 plucks, 9,832 jitter events on 1,138 ticks, polarization 0.8092, spread 21.7713 — and its channel counter reads decisive 0.
- **P1:** the full-journal replay is *exact* at every density — final positions bitwise equal, jitter events and ticks equal, field final sum equal, channel counter identical, no divergent tick. Tolerance: zero.
- **P2:** contacts rise strictly with density: contacts(50) < contacts(200) < contacts(400).
- **P3:** the channel wakes: decisive crossings are 0 at 50 boids and greater than 0 at 400.
- **P4:** at 400 boids the naive replay *parts* from the live run — a first divergent tick exists and its final positions differ. (The sparse field's naive exactness was density's mercy, not the journal's redundancy.)
- **P5:** the journal's economy inverts: at 50 boids the packed journal is smaller than one present-level field snapshot (3,200 cells × f32 = 12,800 bytes); at 400 it is larger.
- **P6:** at 50 boids the naive replay is still exact — the earlier run's crack reproduces in this crate, so the density axis, not the instrument, is what changes.

Falsification condition: if the full journal's replay diverges at any density, the journal is not the run at that density — some unrecorded input is load-bearing, and this README's claim is false in exactly the way the first divergent tick names. If decisive crossings stay 0 even at 400 boids, the within-tick channel is silent at eight times the host's density too, and the honest conclusion narrows: the boid index is insurance against a density this garden has not yet reached, priced but unclaimed at every density surveyed.

## Results (headless, as run)

Every declared prediction held — and the channel woke earlier, and louder, than declared.

Anchors held exact (B1): at 50 boids the live run is the host's season field for field — 566 plucks, 9,832 jitter events on 1,138 ticks, field max 0.8949, field final sum 419.9840, polarization 0.1324 → 0.8092, spread 21.7713 — and the channel counter reads **contacts 0, decisive 0, masked 0**. Not one later boid even *stood* in a struck cell within its strike's tick. The sparse field's silence is total, not marginal.

- **P1 HELD, exact, at every density.** The full journal replays bitwise at 50, at 200, and at 400 boids — final positions equal, jitter events and ticks equal, field final sum equal, channel counter identical, no divergent tick. The journal is still the run when the field fills up.
- **P2 HELD.** Contacts rise strictly: **0 → 149 → 739** across 50 → 200 → 400 boids.
- **P3 HELD, amplified.** Decisive crossings: **0 at 50, 106 at 200, 330 at 400**. The channel also takes hearings away: masked crossings **0 → 3 → 38** — at crowd density, a same-tick strike laid over a negative wave silences a reading roughly one time in ten among its decisive kin (38 masked against 330 decisive).
- **P4 HELD, at the first opportunity.** The naive replay parts from the live run at **tick 1** at both 200 and 400 boids — not mid-season, not gradually. The crowd spawns in one cell, so the field is densest at the start, and a journal that reads a tick as an instant is wrong from the season's first morning. Its final positions differ at both densities.
- **P5 HELD, one density early.** The journal crosses one field snapshot not between 200 and 400 but between 50 and 200: 566 events × 6 B = **3,396 B** at 50 boids, **13,860 B** at 200, **28,098 B** at 400, against the snapshot's 12,800 B. (The widened 6-byte entry is part of the bill: at 200 boids the old 5-byte packing would have read 11,550 B and stayed under.) Past the crossing, writing the season's inputs costs more than one frame of its state — though the frame, per `scarecrow-granary`, still cannot resume a run and the journal still can.
- **P6 HELD.** At 50 boids the naive replay is exact in this crate too — no divergence, final positions equal. The earlier crack reproduces; the density axis is what changes.

The ledgers at the higher densities, for the record: at 200 boids — 2,310 plucks, 108,407 jitter events on all 1,200 ticks, field max 1.6469, field final sum 751.3764, polarization 0.0426 → 0.7503, spread 14.7594. At 400 — 4,683 plucks, 312,541 jitter events on all 1,200 ticks, field max 2.5850, field final sum 1,069.2720, polarization 0.0402 → 0.6646, spread 21.2712.

4/4 tests (host anchor at 50 with the silent channel, exact replay at 200 with the channel counter identical, contacts rising with density, the checksum instrument itself).

The law this row adds: an ordering that carries nothing in a sparse field can be the whole crop in a dense one, and the difference is not visible in the machinery — both fields run the same code. It is visible only in the counter. Write the journal for the machinery (when, who, where, in that order), then measure the run before claiming which parts of the writing the season needed — the answer changes with the crowd, and at tick 1 the crowd is already there.

## Usage

```
cargo run -p scarecrow-crowd -- --headless
cargo test -p scarecrow-crowd
```

Stand still. Write it down. Then fill the field with birds and see whether the writing still holds — and whether the part of the writing nobody needed in an empty field is the part the crowd cannot live without.
