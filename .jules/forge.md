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
