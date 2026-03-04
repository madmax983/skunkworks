# 065. De-abstract Single-Implementation Traits

* **Status:** Proposed
* **Context:**
  The codebase contained multiple single-implementation traits that introduced unnecessary indirection, dynamic dispatch (`Box<dyn Trait>`), and cognitive overhead without providing any actual flexibility. This violated the YAGNI (You Aren't Gonna Need It) and KISS (Keep It Simple, Stupid) principles. Specific instances included:
  - `AudioSource` trait in `syncopated-threads` and `chimera-syncopation`.
  - `Vec4Ext` trait in `hyper-enigma`.
  - `Rule` trait in `origami-lexicon` and `glossolalia`.

* **Decision:**
  We decided to de-abstract these interfaces into concrete types or standalone functions.
  - The `AudioSource` trait was replaced with a concrete `enum Drum` containing variants `Kick`, `Snare`, and `Hat`.
  - The `Rule` trait for phonology was converted to an `enum Rule`.
  - The `Vec4Ext` trait was removed entirely in favor of standalone rotation functions (`rotate_xy`, `rotate_xz`, `rotate_yz`).

* **Consequences:**
  - **Positive:** Reduces boilerplate and eliminates the runtime cost of dynamic dispatch. Concrete types improve compilation times and make reasoning about the code simpler and more straightforward.
  - **Negative:** If a future requirement genuinely demands polymorphism (e.g., loading arbitrary user-defined audio source logic at runtime), the interface will need to be re-abstracted or refactored to support it.
