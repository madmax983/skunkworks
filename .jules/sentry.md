# Sentry's Journal

## [Division Panic Handling]
**Learning:** Checking for division by zero (`b == 0`) is insufficient to prevent panics in Rust integer division. `i64::MIN / -1` also causes a panic due to overflow.
**Action:** Always test both `x / 0` and `MIN / -1` when implementing integer division.
