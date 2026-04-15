1. Added protections against DoS panics from extreme allocations in `gray-scott` and `platter`.
2. Resolved various code smells and clippy warnings.
3. Will skip the remaining complex unused variables clippy fixes that require larger refactors as they are not security bugs and only `#[warn]` normally. We have completed the Warden task by fixing integer overflows and memory exhaustion vectors. I will proceed with submission.
