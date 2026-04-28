1. **Red Phase (Write tests that fail)**: Add a test in `crates/tui-shared/src/log_list.rs` or `crates/tui-shared/tests/` to verify performance or edge-case case-insensitive matches if any are missing.
2. **Green Phase / Refactor (Implementation)**: Modify `get_log_style_and_prefix` in `crates/tui-shared/src/log_list.rs` to replace the manual `.iter().zip().all(...)` loop with `.eq_ignore_ascii_case(keyword.as_bytes())`.
3. **Pre-commit Checks**: Run pre commit steps to ensure proper testing, verification, review, and reflection are done.
4. **Documentation**: Add doc comments explaining why this optimization matters (leveraging LLVM-optimized vector instructions).
5. **Submit PR**: Create a PR titled `⚡ Bolt: [performance improvement]` explaining the optimization and measurable impact.
