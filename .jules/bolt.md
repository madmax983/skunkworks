**[Title] Stack-Allocated Arrays in Hot Rendering Loops**
**Learning:** Found an inefficient `Vec` allocation chaining using `.collect()` within the hot rendering loop of `experiments/hyperbolic-quipu/src/tiling.rs`.
**Action:** Replaced `(0..4).map(...).collect::<Vec<_>>()` with `std::array::from_fn(...)` and `array.map()` to stack-allocate small fixed-size arrays without triggering heap allocations. Eliminated 300+ vector allocations per frame.
