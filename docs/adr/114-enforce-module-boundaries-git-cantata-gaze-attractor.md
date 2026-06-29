# 114. Enforce Module Boundaries in Git Cantata and Gaze Attractor

* **Status:** Accepted
* **Context:**
  In the `git-cantata` and `gaze-attractor` experiments, internal module structures were exposed due to leaky `pub mod` declarations (in previous states or before Atlas's refactor). This leaks internal implementation details to consumers, violating the principle of encapsulation and creating tight coupling between an experiment's internal file layout and its external interface.

* **Decision:**
  We applied the Facade pattern to enforce module boundaries in `git-cantata` and `gaze-attractor`. The internal modules (e.g., `audio`, `git`, `vis` in `git-cantata`) were demoted to `pub(crate) mod` to hide their structure, and the necessary public types were explicitly re-exported in the root `lib.rs` (or equivalent interface) using `pub use`.

* **Consequences:**
  - **Positive:** Improved encapsulation. External consumers now interact with a clean, unified API surface without having to understand or depend on the internal directory structure. This allows safe refactoring of the internal structure without breaking external API compatibility.
  - **Negative:** Adds a small maintenance overhead for developers, who must now explicitly update `pub use` statements when exposing new public types.
