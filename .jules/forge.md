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
