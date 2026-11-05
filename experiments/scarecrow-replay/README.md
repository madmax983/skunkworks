# Scarecrow Replay 🌾

**Host:** `experiments/sono-flock` (The Cartographer of Lost Things 🗺️) — 50 boids flying the parent crates' machinery: `crates/flocking` (`compute_force`, the host's parameters verbatim), `crates/locus` (`Vec2`), and `crates/resonance-audio` (`PhysicsGrid`, 80×40 FDTD wave field). Boids pluck the field (coin 0.01, strength 0.5); a boid standing in a cell louder than 0.2 is jittered. The host's constants, streams (three SplitMix64, one per purpose), and update order are copied faithfully and unchanged; tests pin this crate's mute/deaf/coupled ledgers to the host's published ones.

**Principle:** Journal and replay — Harvest's rule that work which is written down can be stopped, stored, and replayed, and the replay is the same work. Learned from Harvest's durable execution: the journal is the run; everything else is derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-05. The comparison and predictions below were declared before the first run and were not edited after it. Verdicts are at the bottom, as found.

## The fragile point

The host's coupling is a circle. The field moves the flock (jitter); the flock strikes the field (plucks). Nothing in the host writes any of it down: stop the run at tick 600 and the season is gone — no store exists, at any size, from which the second half could be derived. `scarecrow-granary` priced storing the host's *state* from outside a crate whose snapshot surface is incomplete. This planting asks the other Harvest question: do not store the state at all. Store the *inputs* — only the plucks, the one place where chance enters the circle — and let the replay derive everything else.

## The journal

One entry per pluck: `(tick, boid, x, y)`, packed at 5 bytes. The boid index is carried on purpose: `PhysicsGrid::pluck` adds to the field's present level at once, and a boid later in the flock order reads the field in the same tick it was struck — so a strike has a place *within* its tick, and the journal names it. The jitter is not journaled (derived: the replayed field is loud in the same cells at the same ticks, and the shared jitter stream is drawn in the same order). No boid is journaled (derived). No field cell is journaled (derived).

## The declared comparison

Five runs of the host's coupled regime (50 boids, 1,200 ticks), compared per-tick by an FNV-1a checksum over every boid's position bits, and at the end by the host's full ledger (plucks, jitter events, jitter ticks, field max, field final sum, polarization, spread, final positions bitwise):

- **Live** — the host's coupled run, unchanged, writing the journal.
- **Replay** — no pluck coin exists in this run; the journal strikes the field, each strike at its own boid's turn.
- **Naive replay** — the same journal, strikes applied at the *end* of their tick, as if a tick were an instant.
- **Torn replay** — the journal with every entry from tick 600 on burned.
- **Shifted replay** — the journal with its first entry moved one cell east.

Predictions, declared before the first run:

- **B1 (host check):** the anchors reproduce the host's published ledger exactly — mute polarization 0.1324 → 0.9518, field silent; deaf 566 plucks, field max 0.9595, final positions == mute; live coupled 566 plucks, 9,832 jitter events on 1,138 ticks, polarization 0.8092, spread 21.7713.
- **P1:** the full-journal replay is *exact* — final positions bitwise equal, jitter events and ticks equal, field final sum equal, no divergent tick. Tolerance: zero.
- **P2:** the journal is smaller than one field snapshot (3,200 cells × f32 = 12,800 bytes, present level only).
- **P3:** the naive replay *parts* from the live run — within-tick strike timing is audible, so reading a tick as an instant must diverge before tick 1,200 and land elsewhere.
- **P4:** the torn replay agrees with the live run at every tick before 600, diverges at some tick ≥ 600, and lands elsewhere.
- **P5:** the shifted replay diverges (its first divergence after tick 2, the shifted strike's tick) and lands elsewhere.

Falsification condition: if the full journal's replay diverges at any tick, the journal is not the run — some unrecorded input (a stream drawn in a hidden order, a state the machinery keeps privately) is load-bearing, and this README's claim is false in exactly the way the first divergent tick names.

## Results (headless, as run)

Anchors held exact (B1): mute 0.1324 → 0.9518, field max 0.0000; deaf 566 plucks, field max 0.9595, positions == mute; live coupled 566 plucks, 9,832 jitter events, 1,138 jitter ticks, field max 0.8949, field final sum 419.9840, polarization 0.8092, spread 21.7713.

- **P1 HELD, exact.** Replay: 566 plucks, 9,832 jitter events, 1,138 jitter ticks, field final sum 419.9840, final positions bitwise equal, first divergence *none*. The journal is the run. 2,830 bytes of strikes derived 1,200 ticks of a coupled flock and field, every derived number identical.
- **P2 HELD.** Journal 566 events × 5 bytes = **2,830 bytes**, against 12,800 bytes for a single present-level field snapshot — a snapshot that, per `scarecrow-granary`, does not even suffice to resume from. The whole season's inputs cost less than a quarter of one frame of its state.
- **P3 CRACKED.** The naive replay is *also exact* — no divergent tick, final positions equal, every number equal. Across 566 strikes, not one was heard within its own tick: no later boid ever stood in a struck cell in the strike's tick with the strike carrying its reading over the 0.2 line. The boid index in the journal is insurance this flock never claimed. It stays in the journal — the parent crate's surface says the channel exists (`pluck` writes the present level, reads are interleaved), and a journal that is exact only at this density is a journal with a silent condition on it. The condition is now named and priced: at 50 boids, 80×40 cells, coin 0.01, the within-tick channel carries nothing. A denser flock is the named next survey.
- **P4 HELD.** The torn journal (296 of 566 events survived) agrees with the live run through the tear and first diverges at tick **602** — the first missing strike was at tick 600; the field needed two ticks to carry the absence to a bird. The torn season lands elsewhere: jitter events 5,932 (vs 9,832), field final sum 247.9729, spread 18.8593 — and, kept honestly, polarization **0.9664**, *higher* than the live run's 0.8092: the quieter field taxed alignment less. A torn journal does not produce a worse flock. It produces a *different* one, measurably, from tick 602.
- **P5 HELD.** One cell, moved at tick 2 (boid 15, cell (41, 17) → (42, 17)): first divergence at tick **9**, final jitter events 9,094, spread **11.9322** against the live 21.7713 — one written cell, one flock half as spread by winter. The journal is small and every byte of it is load-bearing.

6/6 tests (host anchors, host coupled ledger, exact replay, torn divergence window, journal size, the checksum instrument itself).

Stand still. Write it down. What you write down is what survives — and the writing had better name when, and who, and where, because the replay believes every word of it, exactly.
