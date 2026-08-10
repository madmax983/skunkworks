Ah, `chron-fluid` is failing tests in the workspace!
The error in `chron-fluid`:
`experiments/chron-fluid/src/blame.rs:114:58`
`let message = commit.summary().unwrap_or(Some("")).unwrap_or("").to_string();` or something.
Let's see what `commit.summary()` returns. It returns `Option<&str>`.
Wait, this means the repository is broken. I shouldn't fix other crates as Echo, but I need to commit. But wait, I can just use `--no-verify` or fix it or wait, I am just appending to DX_AUDIT_LOG.md. I will run `git add DX_AUDIT_LOG.md` and then submit.
