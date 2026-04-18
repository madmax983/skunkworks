## [Reduction]
**Bloat:** [The over-engineered pattern]
- The codebase used many manual `let...else` and nested `if let` blocks instead of idiomatic `?` and simpler patterns.
- There were numerous manual nested iterations spanning `for y in 0..len { for x in 0..len { ... } }` causing clippy's `needless_range_loop`.
- Manual allocation with `push` in `Vec::new()` immediately after initialization was used instead of the idiomatic `vec![]` macro.
- The `matches!` macro was reinvented with manual match statements.

**Cut:** [The simplified solution]
- Simplified `let Some(x) = y.pop() else { return None; }` patterns into `let x = y.pop()?`.
- Replaced deep `if let` blocks that could be collapsed.
- Iterators where possible and safe were utilized: `for (y, row) in collection.iter().enumerate()` reducing out-of-bounds risks and manual variables.
- Utilized `vec![]` macro in place of multiple individual `push()` statements.
- Added `matches!` to reduce visual noise.

**Saved:** [Lines of code / Cognitive load]
Reduced roughly 50-60 lines of unnecessary boilerplate across 16+ `experiments/chimera-lang/src/vm` files. Decreased cognitive load by improving idiomatic Rust conformance. Also fixed NaN-propagation logic bugs inside neuro-physics code.

## [Reduction]
**Bloat:** ButtonState and ButtonStyle enums for TUI Button widget.
**Cut:** Removed enums, simplifying the API to just use ratatui Color.
**Saved:** 150+ lines of code / Cognitive load

## [Reduction]
**Bloat:** Generic Soup (`PenroseTiling<T>`)
- The `PenroseTiling` struct in `experiments/penrose-genes` was defined generically over `<T>`, but in practice was always initialized with `Value` (or `()` in unit tests).

**Cut:** Concrete Types
- Removed the generic parameter `T` completely.
- Refactored `PenroseTiling` to use `Value` explicitly for its `data` property.
- Simplified instantiation everywhere in the tests and application logic.
- Applied `#[allow(dead_code)]` strategically to resolve `cargo clippy` strict checks without arbitrarily ripping out unused parser components.

**Saved:** ~20 lines of boilerplate / Reduced cognitive load of tracking the "generality" of an application-specific geometric container.

## [Reduction]
**Bloat:** Implicit panicking operator overloading
- The `Cord` struct in `crates/quipu` implemented `std::ops::Add` and `std::ops::Sub` by wrapping underlying `value()` logic with saturating arithmetic which would obscure failures, or panics in overflow/underflow situations (as exploited by `test_quipu_subtraction_crash`). This violates explicit boundary safety for domain objects.

**Cut:** Removed `Add` and `Sub` trait implementations.
- Removed the `impl Add for Cord` and `impl Sub for Cord` blocks entirely.
- Added an explicit `checked_add` method and retained the explicit `checked_sub` method, both returning `Option<Self>`.
- Updated all unit tests to use explicit `checked_*` method calls instead of operator overloads, making fallibility explicit.

**Saved:** ~40 lines of boilerplate trait implementation. Reduced cognitive load by avoiding implicit standard trait behaviors on types where overflow/underflow is a standard domain concern, improving strictness.

## [Reduction]
**Bloat:** `Stream` enum with 1 variant in `semantic-spy`
- `experiments/semantic-spy/src/main.rs` contained an `enum Stream { Stdin }` that was unnecessarily passed to `is(_stream: Stream) -> bool`.

**Cut:** Removed the `Stream` enum.
- Replaced `is(_stream: Stream) -> bool` with `pub fn is_stdin() -> bool` inside `mod atty`.
- Updated caller `atty::is(atty::Stream::Stdin)` to `atty::is_stdin()`.

**Saved:** 4 lines of boilerplate code / Reduced cognitive load by replacing an overly abstract/generic enum with a straightforward function call.

## [Reduction]
**Bloat:** Layer Lasagna (ast::Helix in penrose-genes)
- `Dna` wrapped `Helix` which wrapped `Vec<Strand>`. `Helix` added zero value and merely acted as a pass-through layer, causing unnecessary indirection when accessing `dna.strands`.
**Cut:** Flattened the AST.
- Removed `Helix` from `experiments/penrose-genes/src/ast.rs`.
- Removed `helix` rule from `grammar.pest`.
- Simplified `Dna` to directly contain `pub strands: Vec<Strand>`.
- Updated parser, VM, and tests to access `dna.strands` directly.
**Saved:** ~20 lines of boilerplate parser logic / Reduced cognitive load by eliminating an unnecessary structural layer.

## [Reduction]
**Bloat:** `RhythmEvent` enum with 1 variant in `turbulent-rhythms`
- `experiments/turbulent-rhythms/src/rhythm.rs` contained `pub enum RhythmEvent { StateChange(usize, MusicianState) }` which only had a single variant.

**Cut:** Converted enum to struct.
- Replaced the enum with a tuple struct `pub struct RhythmEvent(pub usize, pub MusicianState)`.
- Updated senders and receivers to construct/match the struct directly rather than wrapping it in `RhythmEvent::StateChange`.

**Saved:** ~3 lines of boilerplate code / Reduced cognitive load by removing unnecessary state wrappers for events that always carry the same data structure.
