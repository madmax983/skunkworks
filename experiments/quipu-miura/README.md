# Quipu Miura 🗺️🧶

**Resurrection cross:** `quipu-origami` (graveyard) × `origami` (living)
**Raised by:** The Cartographer of Lost Things 🗺️
**Bearing:** graveyard/quipu-origami → crates/origami, north-northwest across the map
**Status:** Planted 2026-10-27. Headless ledger below.

## Lineage — the death

- **`quipu-origami`** (executed; grave at `graveyard/quipu-origami`)
  proposed *Knotted Data Morphogenesis*: quipu cord values should
  drive the folding of an origami sheet. The idea was stranger than
  its flaw was boring, which is the Cartographer's whole criterion.
- **Cause of death (Reaper's report, in the grave):** Skeletal
  Implementation / Ecosystem Maladaptation. An 87-line skeleton that
  "renders static dots on screen, using a sine wave instead of
  genuine bidirectional interaction between `quipu` and `origami`…
  it projects flat, abstract 2D dots rather than true folded paper
  constraints. The discrete data (cords) barely influences the
  outcome." Executed for a missing body, not a missing idea.

## Lineage — the donation

- **`origami`** (living crate) donates the exact missing organ:
  `generate_miura_mesh` — a genuine 3D Miura-ori mesh with real
  vertices, indices, and fold heights, parameterised by extension
  factor and crease angle `gamma`.
- **`quipu`** (living crate) donates the ledger: cords whose values
  are tied in base-10 positional knots, folded with `checked_add`.

This is a cross, not a revival: the corpse's name is not reused,
its macroquad dot-renderer is not restored, and no sine wave
survives. The dead keep their name in this lineage, per custom.

## Concept

Every decimal digit of every cord becomes one vertical strip of a
Miura sheet. A digit sets its strip's **target extension** (0 →
0.12, a nearly flat empty space; 9 → 0.95) and its **stiffness**
(more knots, stiffer cord).

Then the two owners negotiate the sheet:

- **Extension belongs to the knots.** Seams pull each strip toward
  its neighbours' extensions; the knots spring it back toward its
  tied target. Stiff strips yield less than slack ones.
- **Gamma belongs to the paper.** Each strip may re-crease —
  hill-climbing its own local seam energy — absorbing mismatch the
  ledger refuses to pay for.

A surveyor's note, learned while mapping this: in the `origami`
crate's Horizontal Miura, edge *z* at even columns is identically
zero — seam mismatch lives in the row spacing and zig-zag x-offset.
The seam energy here is measured on full 3D edge profiles; a
z-only survey reads a flat map of folded land.

Finally the settled sheet is **read back**: each strip's final
extension is decoded to the nearest digit, digits are re-tied into
cords, and the readback total is compared with the original quipu
folded by `checked_add`.

## Novel Trait

**A ledger that folds, negotiated in both directions.** The dead
specimen let cords vaguely influence a picture and let the picture
influence nothing. Here the influence is a measured equilibrium:
seam energy falls from 20.8472 to 0.0457, the paper demonstrably
pushes back (max deviation from knot targets 0.0325, max crease
re-cut 0.180 rad), the slack zero-strip yields further than the
stiff nine-strip (unit-tested) — and the ledger survives the
folding exactly: 13/13 digits preserved, readback 1848 = original
1848. Remove `quipu` and there is nothing to tie or read back;
remove `origami` and there is no folded surface, only the dots the
Reaper already buried once.

## Falsification Condition

The claim — *a knotted ledger can be folded into a real Miura
sheet by bidirectional negotiation and read back intact* — is
falsified if any of these hold:

- the sheet is flat or area-less (a dot projection in disguise):
  height range or 3D surface area at or near zero;
- seam energy does not fall substantially under relaxation (the
  "negotiation" is decorative);
- no strip ever deviates from its knot target and no strip
  re-creases (the paper never pushed back — unidirectional again);
- the readback loses the ledger: fewer than 3/4 of digits survive,
  or the readback total drifts by more than a quarter.

All four are pinned by unit tests (5/5 passing).

## Usage

```bash
# Headless (decided before anything else; prints the ledger)
cargo run -p quipu-miura -- --headless
cargo run -p quipu-miura -- --headless 90 123   # your own cords

# Tests
cargo test -p quipu-miura
```

## The Ledger

Headless, demo cords [420, 77, 108, 9, 1234]: 5 cords tied, quipu
total 1848, 13 digit strips, 455 mesh vertices, 624 triangles,
surface area 286.03, fold height range 0.914, seam energy
20.8472 → 0.0457 over 400 relaxation steps, max deviation from
knot target 0.0325, max gamma shift 0.180 rad, digits preserved
13/13, readback total 1848.

*Nothing here is waste. Some of it is just early.*
