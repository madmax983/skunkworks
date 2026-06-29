# 113. Enforce Module Boundaries in Syncopated Threads

* **Status:** Accepted
* **Context:**
  In the `syncopated-threads` experiment, internal modules (`audio`, `model`, `threads`, `tui`) were previously exposed using `pub mod`. This leaked internal implementation details and structure to consumers, violating the principle of encapsulation and creating tight coupling between the experiment's internal organization and its external usage.

* **Decision:**
  We enforced the Facade pattern for the `syncopated-threads` crate. The leaky `pub mod` declarations in `lib.rs` were demoted to `pub(crate) mod`, hiding the module structure from external consumers. We then explicitly re-exported the necessary public types using `pub use`.

* **Consequences:**
  - **Positive:** Improved encapsulation and reduced coupling. Consumers now interact with a clean, unified API surface without needing to know the internal directory or module structure. The internal structure can now be refactored safely without breaking external API compatibility.
  - **Negative:** Requires developers to manually maintain `pub use` statements in the root `lib.rs` when adding new public types, adding a small amount of boilerplate overhead.
