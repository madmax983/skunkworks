**[Flattening ViewMode Checks]**
**Learning:** `tui/app/handlers/normal/chars.rs` had deep nesting with `if let ViewMode::... = app_state.view_mode { ... } else if let ...`.
**Action:** Replaced `if let ViewMode::... = ...` chains with `match app_state.view_mode { ... }` blocks where multiple branches existed. Replaced isolated `if let ViewMode::... = ...` with `if matches!(app_state.view_mode, ViewMode::...)` to reduce nesting and improve clarity.

**[Flattening VM Pyramids of Doom]**
**Learning:** Stack manipulation in VM operations often involved deep nesting `if vm.stack.len() >= N { let val = ...; if let Value::Int(...) = val { ... } }`.
**Action:** Refactored VM operations in `nova.rs`, `babel.rs`, and `elektra.rs` to use guard clauses: `let Some(Value::Int(c_val)) = vm.stack.pop() else { return None; };` to flatten the execution flow and improve readability.

**[Flattening ViewMode Checks]**
**Learning:** `tui/app/handlers/normal/chars.rs` had deep nesting with `if let ViewMode::... = app_state.view_mode { ... } else if let ...`.
**Action:** Replaced `if let ViewMode::... = ...` chains with `match app_state.view_mode { ... }` blocks where multiple branches existed. Replaced isolated `if let ViewMode::... = ...` with `if matches!(app_state.view_mode, ViewMode::...)` to reduce nesting and improve clarity.

**[Flattening VM Pyramids of Doom]**
**Learning:** Stack manipulation in VM operations often involved deep nesting `if vm.stack.len() >= N { let val = ...; if let Value::Int(...) = val { ... } }`.
**Action:** Refactored VM operations in `nova.rs`, `babel.rs`, and `elektra.rs` to use guard clauses: `let Some(Value::Int(c_val)) = vm.stack.pop() else { return None; };` to flatten the execution flow and improve readability.
**[Flattening ViewMode Checks in Handlers]**\n**Learning:** In TUI handler functions that only apply to a specific `ViewMode` and do nothing otherwise, using `if matches!(app_state.view_mode, ViewMode::X) { ... }` creates unnecessary rightward drift and nesting.\n**Action:** Replaced these blocks with early return guard clauses: `let ViewMode::X = app_state.view_mode else { return Ok(false); };`, flattening the function body and adhering strictly to Forge's guard clause preference.

**[Extracting God Functions in Display Impls]**
**Learning:** `std::fmt::Display` implementations, especially for structures like `Snapshot` that serialize multiple collections (e.g., `metrics` and `entities`), can easily grow into "God Functions" (100+ lines).
**Action:** Extract the formatting logic for individual collections into private helper methods on the struct (e.g., `fmt_metrics`, `fmt_entities`), reducing nesting and cognitive load in the main `fmt` method.

**[Flattening Pyramids of Doom in Match Arms]**
**Learning:** When matching on multiple patterns where the success case relies on nested `if let Some(...) = ...` (a Pyramid of Doom), utilizing guard clauses `let Some(...) = ... else { return; }` allows us to extract values cleanly and dramatically reduce nesting, as seen in `apply_sink_rune`.
**Action:** Prefer `let ... else { return; }` in `match` arms over deep nesting to keep code flat and readable.

**[Replacing if-else Chains with Match]**
**Learning:** Massive `if current_type == X else if current_type == Y` chains (like the one in `process_agents` which spanned nearly 150 lines) are hard to read and easily miss logic. They should be simplified using standard `match` syntax.
**Action:** Refactor long `if-else if` chains evaluating equality on the same variable to idiomatic Rust `match` expressions.
