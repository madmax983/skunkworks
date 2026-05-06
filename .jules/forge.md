**[Route View and Handler Refactoring]**
**Learning:** When refactoring massive `if let` chains into clean `match` statements inside event routers (like `route_view`), it is critical to preserve early `return` behaviors if the function contains logic further down that shouldn't apply to all branches. In this case, extracting the boolean `bypasses_glitch` from the match statement cleanly maps early returns without code duplication or missing conditions.
**Action:** Always verify if a function contains code execution *after* a large `if`/`match` block before refactoring to prevent unintended side effects from bleeding into newly unified logic paths.


**[Flattening Simulation Event Handlers]**
**Learning:** Simulation event handlers (like processing bids or asks in market-sim) often develop a "Pyramid of Doom" combining boundary conditions (`if y > 0`) with entity collision checks (`match target_cell`). This increases cognitive load and indentation depth.
**Action:** Invert boundary checks into guard clauses that return early (e.g., `if y == 0 { return None; }`). Flatten trailing catch-all match arms (`Particle::Wall` and `_`) into a single default arm (`_ =>`) if they share identical fallback behavior like lateral movement.

**[Extracting Audio Process Commands]**
**Learning:** When extracting helper methods that act on instance state from within a large "God Function" (like the command event processor loop in `AudioModel::process`), watch out for the docstrings on the original function. If they describe the *entire* process (including steps now handled by the new helper), they should not be copied verbatim to the private helper.
**Action:** Always rewrite or remove doc comments for newly extracted private helpers to accurately reflect only the specific sub-task they perform, ensuring documentation remains accurate and concise.
