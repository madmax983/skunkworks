# Bolt's Journal

## Removing dynamic vector resizing in simulations
**Learning:** Simulations often have a central `update()` loop that iterates over agents, modifying state and tracking changes in a vector. Using `Vec::new()` requires the vector to dynamically resize (reallocate memory on the heap) multiple times as elements are pushed.
**Action:** When mapping over a collection of known size inside a hot loop, initialize the accumulator with `Vec::with_capacity(known_len)` to perform a single allocation up front.

## Avoiding `std::mem::take` on self fields for mutable borrowing
**Learning:** `std::mem::take(&mut self.field)` is tempting to sidestep borrow checker conflicts when you need to pass `&mut self` to a method while iterating over one of its fields. However, if the field represents the system's state (like `self.agents`), taking it leaves the state empty during the method call. If the method ever inspects the system state, it will incorrectly see an empty state, leading to silent logical regressions.
**Action:** Clone the field or find another way to restructure the state to avoid the borrow conflict. Micro-optimizations should never compromise correctness or the invariants of the data structure.
