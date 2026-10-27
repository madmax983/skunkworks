# Miller Quipu 🦇🧶

**Parents:** `miller-lattice` × `quipu`
**Crossed by:** The Night Listener 🦇 (Splice lineage recorded in MUTATIONS.md)
**Status:** Planted 2026-10-27. Headless ledger below.

## Lineage

- **`miller-lattice`** grows a filesystem tree into a 3D crystal:
  atoms for files and directories, bonds for parent–child structure.
- **`quipu`** is the Inca knotted-cord ledger: each `Cord` encodes a
  number in base-10 knots, and cords combine with checked arithmetic.

Neither parent listens. This cross makes the crystal *answer*.

## Concept

Every atom in the crystal ties a quipu cord recording what its
subtree holds — a directory's cord counts ten per file beneath it, a
file's cord counts its name. That is the census, tied in knots.

Then the Listener refuses to read it. Direct inspection is cheating.
Instead, one pulse is struck at the root and travels outward along
the crystal's bonds, exactly one hop per tick. Each atom the pulse
reaches reflects an echo back along the bond it arrived on, carrying
its cord's value. Returning echoes also move one bond per tick. The
listener at the root knows only what returns, and when it returns.

## Novel Trait

**A census taken by ear.** The quipu's accounting and the echo's
accounting are computed by two independent paths — the cords folded
with quipu's own `checked_add`, and the values physically carried
home by bond-hopping carriers — and they must agree. Arrival time is
information the cords alone do not contain: the per-tick return
fingerprint encodes the *shape* of the tree (depth, branching), while
the cord total encodes only its contents. Remove `quipu` and there
is nothing to carry home; remove `miller-lattice` and there is no
topology for the echo to travel, and no fingerprint.

## Falsification Condition

The experiment's claim — *an echo census recovers a knotted ledger
and fingerprints tree shape* — is falsified if any of these hold:

- the echo total differs from the quipu total folded with `checked_add`;
- a return is heard before its atom could physically be reached and
  answered (a depth-`d` atom heard before tick `2d`);
- two structurally different trees produce the same return fingerprint.

All three are pinned by unit tests.

## Usage

```bash
# Headless (handled before anything else; prints the ledger)
cargo run -p miller-quipu -- --headless
cargo run -p miller-quipu -- --headless <path-to-tree>

# Tests
cargo test -p miller-quipu
```

With no path, it strikes a small deterministic grove built for the
occasion (root with `alpha/`, `beta/deep/`, `gamma/`, and two root
files: 15 atoms, 14 bonds).

## The Ledger

Headless runs report atoms, bonds, lattice span, cords tied, the
quipu total, pulses sent, atoms reached, returns heard, the echo
total, the peak return tick, and the per-tick echo fingerprint —
the numbers that say whether anything actually came back.

*Strike it gently. Listen honestly.*
