1. Fix the Havoc tests to remove input validations inside the test harnesses. The goal is to force random data (NaN, Infinity, usize::MAX) directly into the API without `if .is_finite()` or bound checks in the test itself. We must revert the removed tests and ensure that the tests crash or panic when the system is fragile.
2. Update the proptests to remove any internal `if` statements that check the input, and feed `any::<f32>()` directly.
3. Review `git diff` to ensure tests are properly chaotic.
4. Run `pre_commit_instructions` again.
5. Submit PR.
