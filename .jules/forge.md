# Forge's Journal

## Removing Panics in Stack Pops
**Learning:** Even when stack bounds are checked via `stack.len() >= N`, calling `.unwrap()` is dangerous because it assumes internal logic or bounds checking won't change. However, when writing guard clauses (e.g. `let Some(val) = stack.pop() else { return None; }`), if the enclosing function returns an `Option`, using the `?` operator (e.g., `let val = stack.pop()?;`) is significantly cleaner and idiomatic.
**Action:** When popping from a stack in a function returning `Option`, use the `?` try operator. For functions returning `()` or `bool`, use `let Some(...) = stack.pop() else { return ...; };`. Rely on `cargo clippy --fix` to automate `?` suggestions.
