**[Handle Enter Extraction]**
**Learning:** Large `match` statements evaluating enums with deep, complex branches inside an event handler loop (like `handle_enter_key`) often become unreadable "God objects". Extracting the body of each match arm into dedicated, clearly named helper functions drastically flattens the pyramid of doom, separates concerns, and significantly enhances readability without altering logic.
**Action:** Scan event loops and TUI handler patterns for massive `match` blocks evaluating state enums (`ViewMode`, `InputMode`, etc.), and proactively break them down into domain-specific function calls.

**[State Cleanup Pattern]**
**Learning:** When consolidating repetitive state cleanup boilerplate from massive `match` blocks in TUI handlers, passing mutable boolean flags (`&mut bool`) into every helper function is unidiomatic and prone to behavioral regressions (like dropping error state handling).
**Action:** Use an explicit `enum` (e.g., `PostEnterAction { Cleanup, KeepState, KeepBufferOnly }`). Have helpers return this enum, evaluate it in the parent match block, and perform the cleanup once at the end. This flattens the structure and strictly preserves zero-behavior change.

**[Enum String Matching Refactor]**
**Learning:** Large `match` statements that map an enum to a string in multiple places can become very unwieldy and hard to maintain, especially when the enum is large (e.g. `ViewMode` with dozens of variants and `cfg` flags).
**Action:** Move the `match` statement into an `impl Enum { pub fn as_str(&self) -> &'static str }` method. This centralizes the logic, makes the `match` block reusable, and significantly cleans up the call sites (replacing hundreds of lines with a single method call).
