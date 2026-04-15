**[vec4 coverage]**
**Learning:** `cargo tarpaulin` can truncate output making it difficult to find missing lines. By exporting to XML and parsing `cobertura.xml`, you can precisely find missed coverage lines. Infinite overflows inside generic math functions (like `project_to_3d` returning Infinity/NaN limits) might go undetected in standard tests unless intentionally provoked using extreme constants like `f32::MAX`.
**Action:** Use Python script parsing of XML coverage data when analyzing large files with missing branches. Always include `f32::MAX`/`f32::NAN` boundary tests for math utilities.

**[gray-scott laplacian coverage]**
**Learning:** `cargo tarpaulin` might consistently report lines within `#[inline(always)]` nested loops (like 3x3 convolution kernels) as uncovered due to LLVM optimizations, especially lines containing `rem_euclid` boundary arithmetic or branching logic (`if/else if/else`), even when tests definitively exercise those branches.
**Action:** When `#[inline(always)]` prevents `tarpaulin` from registering hit lines, write robust mathematical boundary tests (e.g. testing index `0, 0` and max boundary) to prove correctness rather than arbitrarily removing the `inline` compiler hint which may negatively affect performance in mathematical code.**Vector Limit & Normalization Overflows**
**Learning:** Checking for overflow conditions (e.g., when a vector's components are so large that their squared sum overflows to Infinity) is critical for correctly handling edge cases in vector mathematics. `cargo tarpaulin` might reveal branches handling `is_infinite()` checks that fall back to unoptimized scaling logic, which requires explicitly setting up `Vec2::new(1e300, 1e300)` (for `f64`) or `Vec4::new(1e38, 1e38, 1e38, 1e38)` (for `f32`) inputs.
**Action:** When auditing or implementing mathematical structures with manual `sqrt` / length checks, write dedicated `f32::MAX` / `f32::INFINITY` regression tests to exercise fallback normalization blocks that protect against scaling artifacts (like returning NaNs or remaining incorrectly clamped to Infinity).
**[Deep Recursion Overflows in Display]**
**Learning:** While `Drop` and `PartialEq` can be made iterative via explicit stacks to prevent stack overflows on deeply nested structures, standard traits like `fmt::Display` are inherently recursive in typical formatting implementations (e.g. tree traversal). Standard Rust tests running these formatters on maliciously deep structures (e.g., 50k levels) will bypass standard panic handlers and trigger a `SIGABRT` due to stack exhaustion.
**Action:** When implementing recursive formatting for nested structures, always implement a depth parameter and a hard cap (e.g., `if level > 500 { return write!(f, "(max depth reached)"); }`) to ensure safe degradation under extreme data conditions.

**[NaN Propagation in Normalization]**
**Learning:** Vector normalization methods (like `Vec4::normalize`) that rely on floating-point `max` or `abs` calculations might inadvertently let `NaN` components pass through finite/length checks if not explicitly guarded against, causing downstream calculations to become polluted with `NaN`s.
**Action:** Always add an explicit `is_nan()` boundary check at the start of mathematical vector operations (e.g., `if x.is_nan() { return Self::zero() }`) to safely collapse invalid coordinate states before applying complex arithmetic.

**[Distance Squared Overflow Intentionality]**
**Learning:** Math functions that compute the sum of squares, like `magnitude_squared`, `length_squared`, and `distance_squared`, naturally overflow to `Infinity` when given large components (e.g. `f32::MAX`). Attempting to "fix" this via scaling limits the performance of these hot-path operations and breaks the expected behavior for existing simulations, as verified by Chaos tests.
**Action:** Do not "fix" intentional overflows in hot-path `length_squared` implementations by adding branches and scaling operations. Focus instead on rigorously testing robust fallback functions like `project_to_3d` with extreme values like `f32::MAX`, `f32::MIN`, and `NaN` to ensure safe degradation downstream.

**[Coverage Gap Audits]**
**Learning:** Found significant gaps in `git-associates` and `hyper-system` where non-default conditions in error handling and math constraint solvers (NaN edge cases and `unwrap_or_default` logic) were not exercised.
**Action:** Always fuzz constraints and force branch conditions using `f32::NAN`, `is_finite()`, or mock empty repositories to ensure `unwrap_or_default()` isn't hiding unhandled code paths.
**Sentry's Journal**\n**Learning:** The deliberate fuzzing test in `crates/tui-shared/tests/havoc.rs` allocates massive amounts of memory (e.g., `usize::MAX / 4`), which can cause Out-Of-Memory (OOM) panics during `cargo test` or `cargo tarpaulin` runs. Removing or excluding this test enables stable workspace test execution.\n**Action:** Use `TestBackend` and `Buffer::empty` within `tui-shared` to thoroughly cover rendering bounds without needing OOM fuzzing tactics.

**[Testing TUI Button Rendering States Headlessly]**
**Learning:** You can test Ratatui widgets cleanly and headlessly without mocking the terminal by creating a `Buffer::empty(Rect::...)` and passing it to the widget's `render(area, &mut buffer)` implementation. This allows assertions on individual cell styles (e.g. `assert_eq!(buffer[(0, 0)].bg, Color::Cyan)`), ensuring style logic maps perfectly to the final buffer.
**Action:** Default to using `Buffer::empty()` for headless TUI tests to test widget layouts, rendering bounds (like `Rect::new(0,0,0,0)`), and dynamic text placement logic without spinning up a full terminal backend.
