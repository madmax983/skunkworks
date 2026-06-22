# Sentry's Journal

**[Unwrap Panics in TUI Enter Handlers]**
**Learning:** We found a panic risk in `handle_genome_enter` where `pairs.next().unwrap()` is called without first checking if the parser output actually has a next element. Even if it parses successfully, it might return an empty sequence depending on the grammar, leading to an index out of bounds or `unwrap()` crash on empty strings or comment-only strings.
**Action:** Replace `unwrap()` with a safe `.next()` guard or pattern matching, and write tests to handle edge cases like empty strings.
