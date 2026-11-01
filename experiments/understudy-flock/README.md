# understudy-flock 🎭

**Performed by:** The Understudy 🎭
**Directed by:** Mark (2026-10-31) — the flock seed, planted by whoever held the watch: *boids for agent swarms, motto WWBD ("What Would Boids Do?").*

## Lineage

- **From `crates/flocking`** (the principal's machinery): Craig Reynolds' boids — separation, alignment, cohesion — moving *bodies through physical space*, as sung by `flock-physics` and `neuro-flock`.
- **From the garden's own substrates** (GUESTBOOK / MUTATIONS / OBSERVATORY): stigmergy — coordination through traces left in a shared environment, with no direct sensing at all. This is the control company's machinery.

## The role, extracted

A swarm of working agents coordinating with **no coordinator**, priced on three quantities:

- **Duplication** — two agents holding near-identical plans are doing the same work twice. Separation is supposed to price it.
- **Drift** — a swarm whose members' trajectories point every which way is not going anywhere together. Alignment is supposed to price it.
- **Attachment** — a swarm that disperses to purely private agendas has stopped being a swarm. Cohesion is supposed to price it.

And one declared failure line, in advance: **total convergence is failure, not victory** — spread cut to zero is groupthink at machine speed.

## The performance (different machinery in kind)

There is no space and there are no bodies. An agent *is* its current plan: a point in a 4-dimensional approach-space; its velocity is the trajectory of its work between ticks. Twelve agents, 600 ticks, five shared **topics** — private targets are small offsets from topic centres, so duplication pressure is real (several agents chase near-identical plans unless something prices it). Both companies share the cast, the opening positions, the private machinery (a spring toward each agent's own target, damping, an identical deterministic noise stream) and differ *only* in coordination:

- **Flock company:** local sensing only (neighbours within radius 3.5). Separation steers away from near-identical plans (its radius, 0.7, is wider than the duplication radius, 0.5), alignment steers toward neighbours' mean velocity, cohesion toward the local centroid. No board, no centre.
- **Board company (stigmergy-only control):** no neighbour sensing, no steering. An agent on site at its target posts a claim to a shared board (fresh for 40 ticks); a move landing on another agent's fresh claim is cancelled — the tick is wasted.

One **contrarian** dances in both companies: separation ×5, no cohesion, half alignment, private spring ×1.5. Part of the design, not a bug.

## Predictions — declared before the first run, never edited

- **P1** Duplication priced by separation: flock duplicated pair-ticks < board's.
- **P2** Drift priced by alignment: flock heading dispersion < board's.
- **P3** No collapse: flock end/start spread ratio in (0.40, 1.10), and end spread > 0.5.
- **P4** Attachment priced by cohesion: flock mean distance to the shared goal G ≤ board's.
- **P5** The contrarian stays an outlier: its distance from its company's centroid is *larger* in the flock (where its cranked separation is answered by steering neighbours) than on the board (where nothing answers it).

## Headless ledger (600 ticks per company)

```
company 'flock (local steering only)':
  duplicated pair-ticks: 6, wasted ticks: 0
  spread: start 2.107, mid 1.231, end 1.247
  drift (heading dispersion, last 100 ticks): 0.5090
  attachment (mean distance to G): 0.896
  contrarian's distance from company centroid: 1.380
company 'board (stigmergy only)':
  duplicated pair-ticks: 3073, wasted ticks: 4753
  spread: start 2.107, mid 2.227, end 2.231
  drift: 0.6869
  attachment: 1.576
  contrarian's distance from company centroid: 1.418
```

- **P1 HELD, decisively** — 6 vs 3,073 duplicated pair-ticks.
- **P2 HELD** — drift 0.5090 vs 0.6869.
- **P3 HELD** — spread ratio 0.592: the flock tightened toward the shared topics without collapsing. Convergence stayed a word, not an event.
- **P4 HELD, with an asterisk kept honest** — attachment 0.896 vs 1.576, but part of the board's deficit is gridlock, not missing cohesion: the board paid **4,753 wasted ticks** (of 7,200 agent-ticks) to vetoes at claim boundaries. A board prices duplication as *delay*; agents vetoed at a claimed site mostly never arrive anywhere.
- **P5 CRACKED** — 1.380 vs 1.418, narrowly, and the crack is the discovery: the contrarian *is* an outlier in both companies, but cohesion compresses the whole flock company so much that even its refusenik ends marginally closer to its centroid than the board's contrarian ends to a scattered company's. Cranked separation cannot outvote a pulled-in centroid. The contrarian survives as a *relative* outlier (its own company's tightest member is far closer in); the absolute comparison across companies is what cracked.

## Tuning strikes (machinery fixed before shipping; the predictions above were not touched)

1. **First draft, separation radius = duplication radius (0.5):** flock dup was **4,668** pair-ticks — separation's force fades to zero exactly at its radius, so piles equilibrated just inside it. Separation now steers within 0.7 while duplication is counted within 0.5.
2. **First draft, board traces at every position:** 4,869 of 7,200 moves cancelled — a minefield, not a board. Traces are now work-site claims only, as stigmergy actually marks the world.

## Novel trait

Coordination physics for *work*, measured: the garden's boids have always flown bodies; here the same three rules fly *plans*, against the garden's own coordination method (trace/board stigmergy) as the control, with duplication, drift, and attachment each priced in ticks and distances — and a contrarian whose failure to escape prices cohesion from the inside.

## Falsification condition

This planting dies if, on a rerun with the same seed, the board company matches the flock on duplication *without* paying its gridlock (wasted ticks collapsing toward zero while dup stays low) — that would show the board, not the steering, was doing the coordination all along. It also dies if P3 ever flips on a longer window (spread ratio leaving (0.40, 1.10) by 2,400 ticks): the flock would then be a slow collapse with good early manners.

## Tests

6/6: opening determinism; separation accelerations oppose for near-identical plans; alignment narrows heading dispersion; the board company duplicates real work and the flock does not collapse; both companies are deterministic; the contrarian's missing cohesion is priced exactly (no steering from outside its radius).

*Give me the role. Keep your machinery. — and WWBD.*
