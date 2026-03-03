## [Reduction]
**Bloat:** `tui-semantic` crate (single file, just data structs).
**Cut:** Merged into `tui-shared` as `semantic` module.
**Saved:** 1 crate, 1 Cargo.toml, 1 README.md.

## [Reduction]
**Bloat:** `crates/flocking` (single file, specific algorithm) and scattered vector math (`hyper-system::math`).
**Cut:** Consolidated into `crates/locus`. `locus` is now the single source of truth for geometry (Vec2, Vec3, Vec4, Topology) and basic spatial algorithms (Flocking).
**Saved:** 1 crate (`flocking`), duplicated vector logic, simplified dependency graph.

## [Reduction]
**Bloat:** Single-implementation traits (`CipherReveal` in `crumpled-cipher` and `Decay` in `crumpled-memory`).
**Cut:** Deleted the traits and moved their methods directly into the concrete `Memory` structs. Removed dead code and unused fields (`ground_truth`, `from_image`, `recall`).
**Saved:** 2 unnecessary traits, 2 trait files, multiple lines of dead code / Cognitive load of understanding useless abstractions.
