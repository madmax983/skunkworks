**Avoid .clone() by indexing on self.vec**
**Learning:** Iterating over `&self.vec` retains an immutable borrow on `self` for the whole loop body, preventing calls to `&mut self` methods. However, `.clone()` is an unnecessary heap allocation.
**Action:** Use `for i in 0..self.vec.len() { let val = self.vec[i]; self.process(val); }`. By indexing, the borrow on `self` is immediately dropped after retrieving the value, satisfying the borrow checker without a clone.
## 2026-03-04 - Removed intermediate .collect() allocation
**Learning:** Iterating directly over an `into_iter` can avoid intermediate `Vec` allocations (e.g. `collect::<Vec<_>>()`) and unnecessary cloning when elements are needed by value.
**Action:** Use `.len()` on the original collection to calculate expected bounds, then consume the iterator directly without allocating an intermediate `Vec`.
