2. Wait, let's review the optimizations again.

`git-associates/src/lib.rs:303`:
```rust
            let path = delta
                .new_file()
                .path()
                .or(delta.old_file().path())
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());
```
`p.to_string_lossy().to_string()` can be optimized to `p.to_string_lossy().into_owned()`. This avoids an extra string format buffer allocation when `to_string_lossy` returns `Cow::Owned`, and is generally faster.

`miller-lattice/src/lib.rs:299`:
```rust
fn get_dir_normal(name: &str, parent_normal: Vector3<i32>) -> Vector3<i32> {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    name.hash(&mut hasher);
    let h = hasher.finish();
```
DefaultHasher is heavily optimized but still SIPHash. DJB2 or similar is extremely fast and zero dependencies.

`tui-shared/src/log_list.rs`:
```rust
        s.as_bytes().windows(keyword.len()).any(|window| {
            window
                .iter()
                .zip(keyword.as_bytes())
                .all(|(&c, &k)| c.to_ascii_lowercase() == k)
        })
```
Changing this to `.any(|w| w.eq_ignore_ascii_case(keyword.as_bytes()))` is the most straightforward, zero-cost, idiomatic optimization. It uses the highly optimized standard library SIMD/chunked routines for byte comparison (if applicable), and is definitely faster than `.iter().zip(...).all(...)`.

Actually, since the task asks for ONE small performance improvement, I should pick the most elegant one. The `.eq_ignore_ascii_case` is a perfect "Zero-cost abstractions are the law" example. Wait, `eq_ignore_ascii_case` on a `[u8]` slice is an explicit slice comparison that is optimized by LLVM, avoiding the overhead of `zip` and manual indexing or manual closure execution for each byte.

Let's look at `tui-shared/src/log_list.rs`:
```rust
<<<<<<< SEARCH
        s.as_bytes().windows(keyword.len()).any(|window| {
            window
                .iter()
                .zip(keyword.as_bytes())
                .all(|(&c, &k)| c.to_ascii_lowercase() == k)
        })
=======
        s.as_bytes().windows(keyword.len()).any(|w| w.eq_ignore_ascii_case(keyword.as_bytes()))
>>>>>>> REPLACE
```
This is only a few lines, very clean, highly impactful on a hot path (TUI render loop!). It strictly fits < 50 lines.

I will request a plan review for this.
