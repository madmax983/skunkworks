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

## [Reduction]
**Bloat:** Single-implementation traits (`AudioSource` in `syncopated-threads` and `chimera-syncopation`, `Vec4Ext` in `hyper-enigma`, `Rule` in `origami-lexicon` and `glossolalia`).
**Cut:** Replaced single-implementation traits with simple enums or standalone functions. Replaced dynamic dispatch `Box<dyn Trait>` with concrete enum types, de-abstracting the interface and moving implementations closer to where they are used.
**Saved:** Multiple trait definitions, unnecessary dynamic dispatch boilerplate, cognitive load of abstract indirection.

## [Reduction]
**Bloat:** Unnecessary indirection through traits with limited, fixed implementers (`AudioSource` in `syncopated-threads`, `Rule` in `origami-lexicon` and `glossolalia`).
**Cut:** Replaced these traits entirely with simple concrete `enum`s (`Drum` and `Rule`), implementing the required methods directly on the enums using pattern matching. Replaced dynamic dispatch (`Vec<Box<dyn Trait>>`) with direct value storage (`Vec<Enum>`). Replaced inherent `to_string_word` with idiomatic `std::fmt::Display` implementation.
**Saved:** 3 traits, multiple lines of boilerplate (dynamic allocations), cognitive load of abstract indirection and non-standard method names.

## [Reduction]
**Bloat:** `AudioSource` trait and separate struct implementors (`KickDrum`, `SnareDrum`, `Hat`) requiring dynamic dispatch via `Box<dyn AudioSource>`.
**Cut:** Replaced the trait with a concrete `Drum` enum and a single `next_sample` method, allowing for a simpler flat `Vec<Drum>` collection.
**Saved:** Reduced cognitive load of trait abstraction and dynamic dispatch boilerplate.
