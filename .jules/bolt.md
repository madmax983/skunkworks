**Avoid .clone() by indexing on self.vec**
**Learning:** Iterating over `&self.vec` retains an immutable borrow on `self` for the whole loop body, preventing calls to `&mut self` methods. However, `.clone()` is an unnecessary heap allocation.
**Action:** Use `for i in 0..self.vec.len() { let val = self.vec[i]; self.process(val); }`. By indexing, the borrow on `self` is immediately dropped after retrieving the value, satisfying the borrow checker without a clone.
## 2026-03-04 - Removed intermediate .collect() allocation
**Learning:** Iterating directly over an `into_iter` can avoid intermediate `Vec` allocations (e.g. `collect::<Vec<_>>()`) and unnecessary cloning when elements are needed by value.
**Action:** Use `.len()` on the original collection to calculate expected bounds, then consume the iterator directly without allocating an intermediate `Vec`.

**[Removing intermediate Vec allocations on loops]
**Learning:** Iterating directly over `processes.into_iter().take(20)` rather than calling `.collect::<Vec<_>>()` beforehand removes unnecessary heap allocations, resulting in zero-cost abstraction for taking sub-sections of collections in loops.
**Action:** Use `.len().min(n)` to pre-calculate spacing when `.len()` of the collection isn't available from `into_iter().take(n)`, preventing the need to intermediate allocations just to compute lengths.

## Iterators over Vectors for canvas Shapes
**Learning:** `tui` canvas widgets using `Shape` (like `Points`) can accept generic iterators rather than `&[(f64, f64)]`. This avoids creating unnecessary intermediate heap allocations (`.collect::<Vec<_>>()`) every render tick when mapping coordinates (like `y` to `HEIGHT - y`).
**Action:** Use `struct Points<I> { coords: I, color: Color }` where `I: Iterator<Item = (f64, f64)> + Clone` instead of requiring a slice, and use `.coords.clone()` inside `draw` implementation.

**[Avoid doc comments on local let statements]**
**Learning:** Adding a `///` doc comment to a local `let` binding or expression will trigger Clippy's `unused_doc_comments` lint because rustdoc doesn't generate documentation for statements.
**Action:** Use standard `//` comments instead of `///` when documenting local, inline performance optimizations to avoid Clippy errors.

**[Removing heap allocation using fixed-size arrays]**
**Learning:** Using `vec![...]` to define a local collection of predefined items inside a frequently called function creates a new heap allocation on every invocation. When the collection is immutable and of fixed size, this allocation is an unnecessary performance penalty.
**Action:** Use an array literal `[...]` instead of `vec![...]` to allocate the collection directly on the stack, providing a zero-cost abstraction.

**[ratatui::widgets::List iterator compatibility]**
**Learning:** `ratatui::widgets::List::new` takes `IntoIterator<Item = ListItem>`, meaning it is often unnecessary to `.collect::<Vec<_>>()` iterators into a `Vec` before passing them to the UI widget per frame. This saves an intermediate heap allocation on every single frame rendering step.
**Action:** Always pass mapped iterators directly to UI constructors like `List::new()` rather than `.collect::<Vec<_>>()`-ing them unnecessarily, especially in hot paths like `Terminal::draw`.
**[Eliminating Vec<char> during string iterations]**
**Learning:** [Using peekable iterators eliminates unnecessary Vec allocations while preserving logical correctness.]
**Action:** [Use peekable iterators instead of chars().collect() when traversing strings.]

**[Double Buffered Diffusion]**
**Learning:** In simulation or cellular automata loops that process grid state over time, cloning the entire `Vec` representing the grid every frame is extremely costly (`O(n)` heap allocations).
**Action:** Use a double buffer approach. Add a `next_state` vector of the same size to the main struct, read from `self.state`, write to `self.next_state`, and use `std::mem::swap(&mut self.state, &mut self.next_state)` at the end of the step.

**[Removing intermediate Vec<char> allocation in TUI drawing loops]**
**Learning:** Calling `.chars().collect::<Vec<char>>()` inside a TUI `draw` frame loop results in unnecessary heap allocations on every single tick for every line rendered. You can directly consume the iterator to map or construct UI spans without allocating an intermediate vector.
**Action:** Use `line_content.chars()` directly. If character padding is required to reach a certain width, combine the iterator with `.next().unwrap_or(' ')` inside a bounded loop (`for x in 0..sim.width`).
