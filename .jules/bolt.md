**Avoid String Split Collection Allocations**
**Learning:** Found string operations in hot rendering paths (TUI views) allocating intermediate `Vec<&str>` via `.split(':').collect()`.
**Action:** Replace `.split(':').collect::<Vec<_>>()` with `.split(':').nth(n)` or iterator `.next()` traversal, bypassing the intermediate heap allocation entirely.
**Avoid String Split Collection Allocations**
**Learning:** Found string operations in hot rendering paths (TUI views) allocating intermediate `Vec<&str>` via `.split(':').collect()`.
**Action:** Replace `.split(':').collect::<Vec<_>>()` with `.split(':').nth(n)` or iterator `.next()` traversal, bypassing the intermediate heap allocation entirely.
**[Eliminate String Cloning in Tokenizers]
**Learning:** In string parsers and tokenizers, a common pattern is accumulating characters into a `String` and then pushing it to a `Vec` using `.clone()` followed by `.clear()`. This causes an unnecessary heap allocation on every token.
**Action:** Replace `push(current.clone()); current.clear();` with `push(std::mem::take(&mut current));`. `std::mem::take` leaves an empty string in its place without allocating, completely removing the intermediate allocation while satisfying the borrow checker.
**[Pre-allocating vectors and moving fields instead of cloning]
**Learning:** During structural transformations from `git_associates::Commit` to `git_rogue::Node`, cloning every string field of the commit struct causes excessive heap allocations. We can take ownership of these fields directly since they are no longer used by moving them out of the source struct. Additionally, we can use `Vec::with_capacity` and `HashMap::with_capacity` when iterating over known sizes like `commits.len()`.
**Action:** When mapping from one structure to another in an iterator over an owned collection, move the values directly instead of cloning them, and pre-allocate the target collections if the size is known.

## [Iterator Chaining for TUI Rendering]
**Learning:** Avoid intermediate `.collect::<Vec<_>>()` allocations and `Vec::insert(0, ...)` overheads in hot TUI rendering loops. `ratatui::widgets::List::new` directly accepts an `IntoIterator`. Using `.into_iter().chain()` allows combining static prepends (like status messages) with dynamic iterators seamlessly without triggering heap allocations or $O(n)$ shifts.
**Action:** When conditionally prepending or appending to lists meant for rendering, prefer building an iterator chain (`chain()`) over creating a `Vec` and shifting elements on the heap.
