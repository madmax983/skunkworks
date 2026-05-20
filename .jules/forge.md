**[Route View and Handler Refactoring]**
**Learning:** When refactoring massive `if let` chains into clean `match` statements inside event routers (like `route_view`), it is critical to preserve early `return` behaviors if the function contains logic further down that shouldn't apply to all branches. In this case, extracting the boolean `bypasses_glitch` from the match statement cleanly maps early returns without code duplication or missing conditions.
**Action:** Always verify if a function contains code execution *after* a large `if`/`match` block before refactoring to prevent unintended side effects from bleeding into newly unified logic paths.


**[Flattening Simulation Event Handlers]**
**Learning:** Simulation event handlers (like processing bids or asks in market-sim) often develop a "Pyramid of Doom" combining boundary conditions (`if y > 0`) with entity collision checks (`match target_cell`). This increases cognitive load and indentation depth.
**Action:** Invert boundary checks into guard clauses that return early (e.g., `if y == 0 { return None; }`). Flatten trailing catch-all match arms (`Particle::Wall` and `_`) into a single default arm (`_ =>`) if they share identical fallback behavior like lateral movement.

**[Extracting Audio Process Commands]**
**Learning:** When extracting helper methods that act on instance state from within a large "God Function" (like the command event processor loop in `AudioModel::process`), watch out for the docstrings on the original function. If they describe the *entire* process (including steps now handled by the new helper), they should not be copied verbatim to the private helper.
**Action:** Always rewrite or remove doc comments for newly extracted private helpers to accurately reflect only the specific sub-task they perform, ensuring documentation remains accurate and concise.
**[Flattening Nested Match Logic]**
**Learning:** When flattening deeply nested loops containing match statements (like in `extract_hunks`), extracting the innermost match logic into a named helper function returning an `Option<T>` is extremely effective. It enables replacing the loops with a flat `.filter_map().collect()` iterator pipeline.
**Action:** Always extract inner `match` expressions into small helper functions when they cause "Pyramids of Doom" inside multiple `for` loops.

**[Missing Default Implementations]**
**Learning:** Idiomatic Rust requires implementing the `Default` trait for structs that provide a parameterless `pub fn new() -> Self` constructor.
**Action:** When auditing files, always check if a struct has `pub fn new() -> Self` but lacks an `impl Default`. Apply `#[derive(Default)]` or implement manually to resolve the discrepancy.
**[Flattening Nested Match Loops with Iterators]**
**Learning:** To flatten 'Pyramids of Doom' caused by deeply nested loops containing `match` statements, extracting the innermost match logic into a helper function returning `Option<T>` allows replacing the outer loops with a flat `.filter_map().collect()` iterator pipeline. This drastically improves code readability without altering behavior.
**Action:** Always extract inner `match` expressions into small helper functions when they cause "Pyramids of Doom" inside multiple `for` loops, then rewrite the iteration using `filter_map`.
**[Replacing nested for loops and match with iterator mappings]**
**Learning:** For loops that just push items to a vector inside a  expression arm like `SExpr::List` create unnecessary mutability and boilerplate.
**Action:** Replace  loops inside list matching arms with an idiomatic iterator pipeline  to map expressions recursively and collect them into a Result vector.

**[Replacing nested loops and conditionals with iterator mappings]**
**Learning:** For loops that simply push items to a vector inside a `match` expression arm like `SExpr::List` create unnecessary mutability and boilerplate. Flattening them with early returns via guard clauses reduces indentation.
**Action:** Replace `for` loops inside list matching arms with an idiomatic iterator pipeline `.iter().map(|item| ...).collect()` to map expressions recursively and collect them into a `Result<Vec<_>>`.
