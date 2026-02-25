# Forge's Journal ⚒️

**[Refactoring Matrix Math]**
**Learning:** Implementing `std::ops` traits for linear algebra structs significantly reduces visual noise and cognitive load compared to method chaining.
**Action:** Always check if `Add`/`Sub`/`Mul` can be implemented for custom math types.

**[Primitive Obsession in Physics Code]**
**Learning:** Usage of raw tuples `(f64, f64)` for vectors leads to "Boolean Blindness" equivalents (e.g., mixing up x/y or position/velocity) and prevents logic encapsulation.
**Action:** Replace tuple clusters with named structs (e.g., `Vec2`) early, even if they seem simple.

**[Duplicated Control Flow in Error Handling]**
**Learning:** Logic for updating application state was duplicated in both success and error branches of a `match` statement, leading to drift risk.
**Action:** Extract the calculation logic (e.g., `measure_latency`) to return a unified value (e.g., `Duration`) so state updates happen in a single, linear path.

**[Logic Duplication in Grid Iteration]**
**Learning:** Logic for calculating cell properties (e.g. conductivity) was duplicated inside multiple nested loops, making it hard to change the property definition globally.
**Action:** Extract property calculation into a pure helper function (`get_conductivity`) before iterating.

**[Feature Flag Blindness]
**Learning:** Logic that lives behind feature flags (e.g., `#[cfg(feature = "audio")]`) can rot silently if refactors are only checked with default features.
**Action:** Always verify refactors by running `cargo check` with all feature combinations or check the CI configuration to ensure coverage.

**[Large Match Arms]**
**Learning:** Extracting complex logic from `match` arms into private helper functions (e.g., `solve_actuator`) significantly improves the readability of the main loop and reduces cognitive load.
**Action:** When a `match` arm exceeds 5-10 lines or contains control flow (like `if/else` or `panic!`), extract it into a named helper function.

**[God Function Refactor: Force Accumulation]**
**Learning:** Extracting complex accumulation logic (like force sums) from tight loops into a helper struct (`ForceAccumulator`) significantly improves readability and reduces "God Function" smell.
**Action:** When a loop body handles multiple state variables (e.g., `separation`, `alignment`, `cohesion`), extract them into a dedicated struct with an `accumulate` method.
