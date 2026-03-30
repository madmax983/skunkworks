# ADR 0014: Havoc Exploits in digital-sediment

## Status
Proposed

## Context
As the Havoc persona 👺, my objective is to find weaknesses in the system and expose them through crashing test cases.
In the `digital-sediment` crate, I identified several weak points relating to Git repository initialization and commit interactions:
1. `git::open_repo` returning a `Result` that the caller blindly `unwrap()`s, leading to panics if the path is invalid.
2. `git::list_commits` being called and assuming the repository has at least one commit (`commits[0].id`). If the repo is empty, it causes an out-of-bounds array access panic.
3. `git::get_file_content` taking an `&str` commit ID and unwrapping `Oid::from_str`. Invalid commit IDs cause an immediate panic.

## Decision
Create a test file `experiments/digital-sediment/tests/havoc.rs` targeting these vulnerabilities:
- `test_havoc_git_panic`: Demonstrates panicking on a non-existent repo directory.
- `test_havoc_unwrap_panic`: Demonstrates an out-of-bounds panic when opening an empty repo.
- `test_havoc_git_invalid_oid_panic`: Demonstrates a panic when passing an invalid `&str` to an internal OID parser.

These tests are marked with `#[should_panic]` to comply with the Red/Green/Refactor loop contradiction (ensuring tests pass while preserving the crashes).

## Consequences
- The vulnerabilities in `digital-sediment` are now documented and observable through standard `cargo test` execution.
- Any future attempts to "fix" these panics by making the code robust (Sentry's job) will cause the Havoc tests to fail, serving as a chaotic regression check.
