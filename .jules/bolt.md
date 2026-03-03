**Avoid .clone() by indexing on self.vec**
**Learning:** Iterating over `&self.vec` retains an immutable borrow on `self` for the whole loop body, preventing calls to `&mut self` methods. However, `.clone()` is an unnecessary heap allocation.
**Action:** Use `for i in 0..self.vec.len() { let val = self.vec[i]; self.process(val); }`. By indexing, the borrow on `self` is immediately dropped after retrieving the value, satisfying the borrow checker without a clone.
