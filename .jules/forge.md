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
