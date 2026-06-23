# Sentry's Journal

**[Unwrap Panics in TUI Enter Handlers]**
**Learning:** We found a panic risk in `handle_genome_enter` where `pairs.next().unwrap()` is called without first checking if the parser output actually has a next element. Even if it parses successfully, it might return an empty sequence depending on the grammar, leading to an index out of bounds or `unwrap()` crash on empty strings or comment-only strings.
**Action:** Replace `unwrap()` with a safe `.next()` guard or pattern matching, and write tests to handle edge cases like empty strings.
**[Unwrap Panics in apply_glitch_fx]**
**Learning:** Found a panic risk in `apply_glitch_fx` where `buffer.cell_mut((x,y)).unwrap()` was called inside a grid traversal loop. If the calculated coordinates somehow fell out of bounds (which is possible if the underlying window resizes out of sync with the logic, or given bounds logic quirks in `ratatui`), it would panic and crash the TUI.
**Action:** Replace `unwrap()` with a safe `if let Some(cell) = buffer.cell_mut((x,y))` guard, and write tests to handle out of bounds or empty buffer edge cases without panicking.
