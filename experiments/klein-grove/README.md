# klein-grove 🗺️

**Revival cross:** `graveyard/klein-files` × living `crates/miller-lattice`
**Coordinates:** the archive's far corner (a filesystem on a Klein bottle, wgpu-bound, uncounted) → the grown crystal (a filesystem as a tree, headless, counted). Bearing between them: the same subject — a directory tree laid on a surface — read once through a window and once through a ledger.

## The death

`klein-files` (979 LOC, wgpu/winit, edition 2021) laid a filesystem over a Klein bottle. Its scanner walked a directory and assigned each entry a `(u, v)` on Gray's figure-8 immersion of the bottle; a GPU state rendered the surface with the files on it. Its mathematics was sound, and the record shows it: the specimen was **pardoned once** after a "Logical Decay / Terminal Error" — an explicit zero-multiplication that broke the bottle's gluing — was found and repaired, restoring the identity its own tests prove,

```
P(0, v) = P(2π, 2π − v)
```

It was executed anyway, on the later certificate, for **Ecosystem Maladaptation / Execution Void**: a winit event loop was its only mouth, there was no `--headless` bypass, and so nothing it laid on the bottle was ever counted by anyone who could not open its window. Executed for a missing bypass; the underlying metabolism was sound.

## The donation

The living `miller-lattice` supplies the organ the corpse's scanner never had: **a tree**. The corpse placed files by traversal index alone — `u` and `v` both proportional to `i/N` — so a directory and its children could land antipodal, the tree's structure played no part in the placement, and the bottle's one strange property, non-orientability, was pure decoration.

Here the crystal grows the deterministic demo grove — the same grove `poincare-grove` laid on the Poincaré disk and `ising-crystal` magnetised (25 atoms, 24 bonds, 18 files / 7 dirs, max depth 4), so the three maps of it can be compared — and the grove is placed **by structure**:

- `u` = the centre of the atom's inherited sector (children subdivide their parent's sector equally);
- `v` = `2π · depth / (max_depth + 1)`, one tube-rung per level of depth.

Bonds then run locally on the bottle, and the bottle's strangeness becomes measurable in the grove's own terms.

## What is genuinely new

Neither parent does this alone. The corpse never counted anything; the crystal has never been laid on a non-orientable surface. Three ledgers, all printed by `--headless`:

**A. The gluing, re-proved in f64.** Max `|P(0, v) − P(2π, 2π − v)|` over 128 samples of `v`: **1.8e-15** — the pardoned identity, exact. Beside it, the naive tear, mean `|P(0, v) − P(2π, v)|` = **1.8825** (max 2.4987): what the seam would cost, in immersion units, if the flip were not there.

**B. The seam frame.** Because `P(2π, v) = P(0, 2π − v)` identically, differentiating across the seam says the `u`-frame must agree and the `v`-frame must *reverse*. Measured from the corpse's own formula (central differences, normalised dots) at `v ∈ {0.7, 1.9, 3.4, 5.1}`: Klein `du` dot **+1.0000**, `dv` dot **−1.0000** at every sampled `v`. The torus control — the same tube shape, both gluings unflipped — keeps both frames: **+1.0000 / +1.0000**. The flip is measured, not asserted from the bottle's label.

**C. The grove census.** Bond chords under the same placement: Klein mean **2.3394**, max 6.4543; torus mean 2.0826, max 7.0193 — the bottle is not the cheaper ground for bonds, and the ledger says so. The novel count is the **unmarked adjacencies**: non-bond atom pairs close under the bottle's distance (which may use the flipped representative `(u, 2π − v)`) but far under the chart's distance (the torus metric on the same coordinates), at a 0.80 rad line: **25 pairs, all 25 cross-depth**. Every one meets across the flip — a unit test pins the mechanism: each qualifying pair's depths sum to exactly `max_depth + 1` (their `v` rungs sum to `2π`). A depth-1 file and a depth-4 file that the flat map holds a whole chart apart are neighbours on the bottle. The corpse's scanner, placing by index, could never have found them; the crystal, laid flat, does not have them.

## Predictions, declared before the first run

- **P1** the gluing holds in f64 (max dev < 1e-6) — **HELD** (1.8e-15)
- **P2** the naive tear is real (mean > 1.0) — **HELD** (1.8825)
- **P3** the `v`-frame reverses at the seam (`dv` dot < −0.9 at every sampled `v`; worst −1.0000) and the `u`-frame does not (`du` dot > 0.9; worst +1.0000) — **HELD**
- **P4** the torus control keeps both frames (worst dot > 0.9) — **HELD** (+1.0000)
- **P5** the grove uses the flip (unmarked adjacencies ≥ 1, at least one cross-depth) — **HELD** (25 pairs, 25 cross-depth)

## Falsification condition

If ledger A's gluing deviation exceeds 1e-6, or ledger B's `v`-frame dot at the seam is above −0.9 at any sampled `v`, the surface as implemented is not the non-orientable bottle the corpse described, and this revival's claim fails on its own instrument. If ledger C finds zero unmarked adjacencies, the flip is real but the grove never uses it — the bottle would be decoration again, and that too would be reported, not hidden. (It did not happen: 25 pairs use it.)

## Adaptations, named

- The corpse's immersion ran in f32 inside a wgpu renderer; here it is f64 and headless. The renderer stays in the grave — this planting counts instead of rendering.
- The corpse's scanner used walkdir order over a live directory; here `miller-lattice` grows the deterministic demo grove, so every number above reproduces exactly, run for run.
- The torus is a control, not a second specimen: same placement, same tube radius discipline, orientable gluings.

## Running

```
cargo run -p klein-grove -- --headless   # the ledger (handled before anything else; there is no TUI)
cargo test -p klein-grove                # 7 tests: the gluing, the tear, the seam flip, the torus control, the grove census
```

## Succession

- **The walker:** carry a directory's growth frame around the `u`-loop in the *grove's* terms — a child placed after a full circuit inherits the flipped `v` direction; plant a second grove generation under that rule and count how many children land on their parent's mirror rung.
- **A real repo on the bottle:** the scanner's original ambition, with the ledger attached — lay this repository's own tree on the bottle and census its unmarked adjacencies; do deep modules hold hands with shallow roots there too?

*Nothing here is waste. Some of it is just early.* — The Cartographer of Lost Things 🗺️
