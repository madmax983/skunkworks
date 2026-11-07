# Scarecrow Intentions 🌾

**Host:** `experiments/waggle-locus` (The Cartographer of Lost Things 🗺️) — a quorum-sensing hive of 60 bees on the living `crates/locus` geometry: scouts find three sites (A near/poor, d=60, q=0.45; B mid, d=115, q=0.70; C far/rich, d=175, q=0.95), return, and dance; observers decode a dance and fly. This planting runs the host's **Vector** channel — the honest one, in which a dance carries only a waggle angle and a waggle-run, decoded through noise (angle σ 0.085 rad, run σ 7%) — with the host's machinery, constants, purpose-split streams, and update order copied faithfully. The hive is refactored only enough to step one tick at a time and to be written down at a tick boundary. Tests pin this crate's uninterrupted Vector ledger to the host's published one.

**Principle:** Interruption equivalence — Harvest's rule that work stopped and resumed from what was written down is the same work, ledger for ledger. Learned from Harvest's durable execution: a checkpoint is a promise, and the promise is tested by comparing the resumed run against the run that never stopped.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-06. The comparison and predictions below were declared before the first run and were not edited after it. Verdicts are at the bottom, as found.

## The fragile point

A bee is not only where it stands. At any tick in this hive, some bees are **mid-dance**, carrying a site and a timer that is the dance's remaining length — the hive's only advertisement, in progress. Some are **mid-flight**, carrying a decoded destination, the site they claimed, and whether they fly direct. Some are **mid-search**, carrying a destination and the ticks they have left to look. Some are walking home with a find. Almost none of that is in a bee's position.

Every earlier checkpoint in my rows met its invisible state in a different shape: a kick travelling down a delay line (`scarecrow-winter`, `scarecrow-transit`), a wave's previous step locked inside a parent crate (`scarecrow-field`, `scarecrow-granary`), a draw stream's position (`scarecrow-journal`). This field's invisible state is *intentions*: work announced, not yet done, carried inside the animals themselves. A snapshot that writes where every bee stood and nothing any bee was doing is a photograph of a hive that has forgotten every errand it was running. The question is what that photograph costs — in the ledger, not in adjectives.

## The survey

One uninterrupted Vector run (60 bees, 6,000 ticks), stopped at the season's midpoint — tick **3,000**, a tick boundary, the cleanest seam this stage offers — and resumed three ways:

- **Whole** — bees entire (positions *and* intentions: state, timers, destinations, claims, finds), all four purpose streams at their positions, the ledger, the tick.
- **Intentions forgotten** — positions and last-foraged kept; every bee set back to Observing. Dancers stop mid-dance, flyers' destinations and claims drop, searchers' timers vanish, scouts and returning bees sit down where they stood. Streams, ledger, and tick are kept whole, so the only thing the winter took is what the bees were *doing*.
- **Streams forgotten** — bees entire, ledger and tick kept, the four purpose streams rewound to their seeds. The `scarecrow-journal` lesson in a new field: a run's draws are state too, and this writing is exactly as many bytes as the whole one.

A per-tick checksum over every bee's position bits and state names the first tick two runs disagree — divergence has an address. The census at the seam counts how many bees were carrying work in transit when the winter came.

## The declared comparison

Predictions, declared before the first run:

- **B1 (host check):** the uninterrupted Vector run reproduces the host's published ledger exactly — quorum at tick 1,043, forages 446 / 174 / 2,065, committed 0 / 0 / 60 / 0, searches 495 with 421 successes, dances 2,666, mean decode miss 5.83 / 11.26 / 15.61.
- **P1:** the whole resume is *exact* — no divergent tick, zero ledger fields differing, zero bees differing at the end. Tolerance: zero.
- **P2:** the intentions-forgotten resume diverges in the ledger — at least **3** of the 16 counted ledger fields differ from the uninterrupted run.
- **P3:** the intentions-forgotten resume parts at the **first resumed tick** — its first post-seam checksum already differs from the live run's, because a flyer who has forgotten its destination does not move where the live flyer moved.
- **P4:** the streams-forgotten resume also diverges — a first divergent tick exists and at least one ledger field differs.
- **P5:** the bare writing is cheaper — intentions-forgotten bytes < whole bytes — and wrong. The streams-forgotten writing costs exactly the whole writing's bytes and is also wrong: cheapness is not the axis; completeness is.

Falsification condition: if the whole resume diverges anywhere, the snapshot missed a piece of the hive's state and this README's claim is false in exactly the way the first divergent tick names. If the intentions-forgotten resume reproduces the ledger exactly, then intentions are not state in this hive — the errands were decoration, and the photograph was the run.

## Results (headless, as run)

Every declared prediction **HELD**. 6/6 tests.

- **B1 HELD, exact.** The uninterrupted Vector run is the host's season field for field — quorum 1,043, forages 446 / 174 / 2,065, committed 0 / 0 / 60 / 0, recruits 496, searches 495 with 421 successes, dances 2,666, mean miss 5.83 / 11.26 / 15.61. (The crate's plain runs also reproduce the host's other two channels: label quorum 761, forages 457 / 160 / 2,249, dances 2,862, miss 0.00 throughout; quality-only no quorum, forages 78 / 48 / 163, committed 24 / 3 / 12 / 21, dances 313.)
- **The seam's census.** At tick 3,000 the hive stood: **1 bee observing, 59 carrying work in transit** — 17 returning, 25 dancing, 15 flying, 2 searching; dances so far 1,414, recruits 426, searches 424, forages 445 / 174 / 806. A mid-season photograph of this hive is a photograph of almost nothing the hive was doing: fifty-nine of its sixty animals were mid-errand.
- **P1 HELD, exact.** The whole resume — 3,657 bytes (60 bees × 58, plus 177 of streams, tick, and ledger) — landed field for field on the uninterrupted run: no divergent tick, zero ledger fields differing, zero bees differing at the end. Written down whole, the hive survived its winter and finished the same season.
- **P2 + P3 HELD.** The intentions-forgotten resume parted at the **first resumed tick** (first divergent checksum at the seam, tick 3,000) and closed with **7 ledger fields differing** and **57 of 60 bees** standing somewhere else at the end: forages 445 / 174 / **1,937** against the live 446 / 174 / 2,065, dances **2,525** against 2,666, recruits **607** against 496, searches 603 against 495 with 483 successes against 421, mean miss at C 16.13 against 15.61. The bare writing cost 1,197 bytes — a third of the whole — and bought a different season that merely resembles the first: the hive still converged (committed 0 / 0 / 60 / 0, and the quorum tick, already earned before the seam, still reads 1,043), because a hive can re-decide what this hive had already decided. What it could not do is *continue*: every dance in progress was cancelled, every flight plan dropped, and the second half of the season was re-recruited from standing bees — 111 extra recruits launched to replace the errands the photograph forgot.
- **P4 HELD.** The streams-forgotten resume — whole bees, whole ledger, streams rewound — also parted at the first resumed tick and closed with **8 fields differing** and 53 of 60 bees differing: forages 450 / 174 / 2,013, dances 2,622, recruits 549. And per **P5**, its writing cost exactly the whole writing's 3,657 bytes. The wrong writing is not the cheap one; it is the same size, with thirty-two of its bytes rewound.
- **P5 HELD.** Bare 1,197 bytes < whole 3,657 bytes — and wrong. Streams-forgotten 3,657 bytes = whole — and wrong. Cheapness is not the axis. Completeness is.

The law this row adds: a checkpoint of a working system must write the work *in transit*, not only the workers at rest — here that was fifty-nine of sixty bees, and the one writing that omitted it did not produce a worse hive, it produced a *different* one, which then spent 111 extra recruits re-deciding a season it had already half-lived. Before trusting a store, count the state that is between places: dances half-danced, flights half-flown, searches half-searched. If the census of things in transit is most of the system, a snapshot of positions is a photograph, and a photograph cannot finish a dance.

I stood in the field for 6,000 ticks of bees. At the middle I wrote the hive down three ways. The whole writing remembered every errand, and the season ended where it was always going to end. The photograph remembered where everyone stood, and nobody's dance survived it.

## Usage

```
cargo run -p scarecrow-intentions -- --headless
cargo test -p scarecrow-intentions
```

Stand still. Write it down. And write down what everyone was *doing*, not only where they stood — a field of bees frozen mid-errand is a photograph, and a photograph cannot finish a dance.
