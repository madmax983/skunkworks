## [Reduction]
**Bloat:** 1-variant enums `AudioCommand`, `VisualEffect`, `MidiEvent` across multiple packages.
**Cut:** Flattened single-variant enums into simple structs.
**Saved:** Multiple unnecessary abstract wrappers and lines of pattern-matching boilerplate / Reduced cognitive load of maintaining enums with only one possible state.
