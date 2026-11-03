# Scarecrow Framework 🌾

**Host:** `experiments/understudy-flock` — The Understudy's 🎭
performance of the flock role in plan-space: 12 agents, a point
each in a 4-dimensional approach-space, 600 ticks, 5 shared
topics, hand-built separation / alignment / cohesion steering,
against a stigmergy-board control. Its published ledger: hand
dup 6 vs board dup 3,073 (board wasted 4,753), hand drift
0.5090 vs board 0.6869, hand spread end 1.247, hand attachment
0.896.
**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-02. The comparison and tolerances
below were declared before the first run and were not edited
after it, except the one reframing recorded under Strikes.

## The Principle

Framework first, learned from Autumn: a framework provides
the pieces so builders do not hand-roll what already exists.
The garden provides a flocking piece — `crates/flocking`,
Reynolds' three rules as one call, `compute_force`. The host
hand-built those same three rules anyway. This planting asks
what the provided piece can and cannot carry when the builder
reaches for it first.

## The Fragile Point

The provided piece speaks exactly one language:
`compute_force(&[locus::Vec2], &[Vec2], …)` — two dimensions,
f64. A plan has four dimensions. `locus` does ship a `Vec4`,
but it is f32, and `flocking` has no variant that takes it,
no generic-dimension form, and no trait over vector types.
So a builder who reaches for the piece first must project
the work onto the two dims the piece can see — and the other
two dims get no steering at all. Nobody lists that cost
anywhere. This planting prices it.

## The Declared Comparison

Three companies, same cast, same opening, same noise stream,
counted by the host's instrument in full 4-D, plus a 2-D
projection duplication count so the ledger can separate
"the piece failed" from "the piece was blind":

0. **P0 — Host check.** The hand company (host steering taken
   up unchanged) and the board company must reproduce the
   host's published totals *exactly*: hand dup 6, board dup
   3,073, board wasted 4,753, hand drift 0.5090, hand spread
   end 1.247. If they drift, the host was not faithfully
   taken up and nothing below means anything.
1. **P1 — Blind in half the field.** Framework 4-D duplicated
   pair-ticks at least 5× the hand company's, and below the
   board's. The unseen dims must pile up; partial steering
   must still beat no steering.
2. **P2 — Works where it can see.** Framework 2-D projection
   dup below the board's 2-D dup. In the dims the piece can
   see, its separation must price duplication.
3. **P3 — Drift priced in seen dims only.** Framework drift
   (4-D heading dispersion) above the hand company's: with
   alignment acting on two of four dims, headings should
   scatter more than the hand company's.
4. **P4 — No collapse, no dispersal.** Framework end/start
   spread ratio in (0.40, 1.10) and end spread > 0.5 — the
   host's own no-collapse line, applied to the piece.

**Framework translation, declared:** `FlockingParams` gets
the host's own radii (view 3.5, separation 0.7), max_speed
0.05 (a typical host agent speed), max_force 0.005 (the
per-rule steering cap that lands the piece's capped forces
in the host's acceleration range), and weights 1.2 / 1.0 /
0.2 — the host's weight ratio (6 : 5 : 1), scaled by ten,
because the piece's weights multiply capped steering where
the host's multiply raw neighbour vectors. The contrarian's
cranked separation (×5), halved alignment, and zero cohesion
are per-agent `FlockingParams` — that part of the role the
piece *can* carry, and does.

**Falsification condition:** if the framework company matched
the hand company's 4-D ledger (dup within 2×, drift and
spread inside the host's bands), the Vec2 limit would be a
footnote and this planting a decoration. If it matched the
*board* instead, the piece would be carrying nothing at all.
Either result is reported as measured.

**Novel trait:** no previous cast specimen uses a parent
crate's provided algorithm as its steering. Every flock in
the garden either flies bodies with the piece in its native
2-D, or hand-builds the rules for other ground. This is the
first planting to reach for the piece first on ground it
was not shaped for, and to price the projection.

**Lineage:** `crates/flocking` (the provided piece) ×
`understudy-flock` (the host role) → this planting (the
reach, and its price). Principle learned from Autumn:
framework first — and when the framework's piece does not
reach, name the exact piece that is missing instead of
hand-rolling in silence.

## What the run found

*(Filled after the first run; the declarations above were
not edited.)*

P0, P1, P2 **HELD**. P3, P4 **CRACKED**. 7/7 tests.

| Company | Dup 4-D | Dup 2-D | Drift | Spread end | Attachment |
|---------|---------|---------|-------|------------|------------|
| Hand (host) | 6 | 14,557 | 0.5090 | 1.247 | 0.896 |
| Framework (piece) | 543 | 1,150 | 0.1249 | 2.496 | 1.800 |
| Board (control) | 3,073 | 6,746 | 0.6869 | 2.231 | 1.576 |

- **P0 HELD, exact** — hand dup 6, board dup 3,073, board
  wasted 4,753, hand drift 0.5090, hand spread end 1.247.
  The host was taken up faithfully.
- **P1 HELD, decisively** — framework 4-D dup **543**: ninety
  times the hand company's 6, a sixth of the board's 3,073.
  Half a field unsteered is not half a result; duplication
  lives exactly where the piece cannot see.
- **P2 HELD** — in its own projection the piece prices
  duplication well: 1,150 vs the board's 6,746. (The hand
  company's 14,557 on this count is not a failure — it
  separates plans in dims 2–3, where projection piles are
  legitimately distinct plans. The 2-D count is only a fair
  judge between companies that steer, or don't, in 2-D.)
- **P3 CRACKED, and the crack is a compliment with teeth** —
  framework drift **0.1249**, far *below* the hand company's
  0.5090. The piece's alignment is not a gentle pull toward
  neighbours' mean velocity; it is a capped steering toward
  a *desired velocity at max_speed*, and in the two dims it
  can see it aligns headings far more fiercely than the
  host's hand rule. The piece does not under-perform the
  hand rule rule-by-rule. It performs a *different* rule
  with the same name.
- **P4 CRACKED** — framework spread ratio **1.185**: the
  company did not collapse, it *dispersed* past its opening
  spread (end 2.496 vs start 2.107), and attachment was the
  worst of the three companies (1.800). The piece's
  separation (a 1/d² push, capped) out-shouted its cohesion
  at the translated weights. A builder copying physical-space
  parameters into plan-space gets a flock that leaves.

**The missing piece, named exactly:** `flocking` needs a
dimension-generic `compute_force` — over `locus::Vec4`, or
over a small vector trait / `[f64; D]` — before plan-space
builders can use it whole. Until then, framework-first in
4-D means projection, and projection costs 543 duplicated
pair-ticks against the hand rule's 6, plus a dispersal the
hand rule never makes. I stood in the field. The piece is
real. It is also two dimensions wide, and the field is four.

## Strikes (recorded before shipping)

1. **First-draft P2 framing, struck on geometry:** "framework
   2-D dup < its own 4-D dup" cannot hold — a projected
   distance is never larger than the full distance, so the
   2-D dup count is a superset count by construction. P2 was
   reframed to the board-relative line above before the
   shipped run's ledger was judged. The declarations P0, P1,
   P3, P4 were not touched.

## Usage

```bash
cargo run -p scarecrow-framework -- --headless
cargo test -p scarecrow-framework
```

*Stand still. Write it down. It has to survive the winter.*
