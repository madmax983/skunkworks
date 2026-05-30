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
**[Flattening Match Pyramids into if/else if Guard Clauses]**
**Learning:** Many compiler functions have unnecessary nesting where a  arm for an identifier opens a secondary  arm to perform specific actions or catch-all default behaviors. This nesting obfuscates the control flow.
**Action:** Flatten nested match arms when the inner conditions are just strings. Replace the inner match with a flat  chain, effectively removing a layer of indentation while maintaining exactly the same logic.

**[Flattening Match Pyramids into if/else if Guard Clauses]**
**Learning:** Many compiler functions have unnecessary nesting where a `match inner.as_rule()` arm for an identifier opens a secondary `match op.as_str()` arm to perform specific actions or catch-all default behaviors. This nesting obfuscates the control flow.
**Action:** Flatten nested match arms when the inner conditions are just strings. Replace the inner match with a flat `if/else if/else` chain, effectively removing a layer of indentation while maintaining exactly the same logic.
**[Flattening the "Pyramid of Doom" in nested if lets]**
**Learning:** Deeply nested `if let Ok(x) = y` statements obfuscate logic, pushing the happy path deep to the right while making the code harder to read. Replacing them with guard clauses (`let Ok(x) = y else { return; }`) drastically flattens the execution flow.
**Action:** When auditing files, search for multiple nested `if let` blocks or loops wrapped inside `if let`. Extract the conditionals to the top using `let ... else` guard clauses to achieve a linear sequence of execution.

**[Topology Normalization Pyramid of Doom]
**Learning:** Found deeply nested if/else blocks (Pyramid of Doom) and reversed guard clause logic in `Topology::Mobius`, `Topology::CylinderH`, and `Topology::CylinderV` normalization logic.
**Action:** Use early returns to flip guard clauses and flatten the structure, improving readability and adhering to Forge's principles.
**Flattening Deep Nesting in Core Loops**
**Learning:** Replaced deeply nested `if let` and bounds checking inside hot loops (`market-sim`, `git-associates`, `resonance-audio`) with `let else { continue; }` and early `return`/`continue`. This dramatically reduces cognitive load and rightward drift, enforcing a flat architecture without sacrificing performance.
**Action:** Proactively apply guard clauses in iterators or block bounds checks before delving into inner loop logic.
