**Remove intermediate allocations when constructing TUI ListItems**
**Learning:** Ratatui's `List::new()` accepts any `IntoIterator`. When generating lists of `ListItem`s from an iterator (e.g., parsing logs or outputs), it is unnecessary to use `.collect::<Vec<_>>()` to create an intermediate heap-allocated `Vec`. You can directly pass the iterator into `List::new(iterator)`.
**Action:** Always check if Ratatui widgets accept iterators directly before collecting into a vector. This provides a zero-cost abstraction and eliminates heap allocations on hot paths.
