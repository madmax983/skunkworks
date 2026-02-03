# Forge's Journal ⚒️

**[Refactoring Matrix Math]**
**Learning:** Implementing `std::ops` traits for linear algebra structs significantly reduces visual noise and cognitive load compared to method chaining.
**Action:** Always check if `Add`/`Sub`/`Mul` can be implemented for custom math types.

**[Primitive Obsession in Physics Code]**
**Learning:** Usage of raw tuples `(f64, f64)` for vectors leads to "Boolean Blindness" equivalents (e.g., mixing up x/y or position/velocity) and prevents logic encapsulation.
**Action:** Replace tuple clusters with named structs (e.g., `Vec2`) early, even if they seem simple.
