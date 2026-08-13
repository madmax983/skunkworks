**[Title: Eliminated Intermediate .collect::<Vec<_>>() and Excessive Cloning in bio-chain]**
**Learning:** `Vec::clone()` inside loops (like `for block in &self.pending_blocks.clone()`) allocates unnecessary memory. Referencing the collection directly, or iterating over indices, avoids these allocations. Furthermore, `ratatui`'s `List::new` takes an `IntoIterator`, meaning we can pass an iterator to it without having to call `.collect::<Vec<_>>()` and allocate an intermediate `Vec`.
**Action:** Avoid `.clone()` when iterating if a direct reference or index lookup suffices. When constructing `ratatui::widgets::List` (or similar UI components taking `IntoIterator`), skip the intermediate `.collect()` step unless the collection must be owned or reused.

**[Title: Eliminated Intermediate .collect::<Vec<_>>() and Excessive Cloning in bio-chain]**
**Learning:** `Vec::clone()` inside loops (like `for block in &self.pending_blocks.clone()`) allocates unnecessary memory. Referencing the collection directly, or iterating over indices, avoids these allocations. Furthermore, `ratatui`'s `List::new` takes an `IntoIterator`, meaning we can pass an iterator to it without having to call `.collect::<Vec<_>>()` and allocate an intermediate `Vec`.
**Action:** Avoid `.clone()` when iterating if a direct reference or index lookup suffices. When constructing `ratatui::widgets::List` (or similar UI components taking `IntoIterator`), skip the intermediate `.collect()` step unless the collection must be owned or reused.

**[Title: Eliminated Intermediate .clone() Allocations in Euclidean Algorithm]**
**Learning:** In numeric loops using big integers (like `BigUint` in `num-bigint`), creating a `.clone()` to act as a temporary swap variable leads to severe O(N) heap allocations, where N is the number of loop iterations. Leveraging `std::mem::swap(&mut a, &mut b)` in tandem with assignment operators (like `%=`) enables zero-allocation, in-place math.
**Action:** When implementing mathematical loops (like `gcd`) with heap-allocated types, prefer in-place mutation and `std::mem::swap` over temporary cloned variables.
