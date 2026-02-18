# 040. Extract Origami Logic

* **Status:** Accepted
* **Context:**
  Multiple experiments in the repository (`origami-constellation`, `rigid-origami`, `origami-spores`) involve folding geometries, specifically Miura-ori tessellations. The mathematical logic for calculating vertex positions based on folding angles and orientation (horizontal vs. vertical zig-zag) was being duplicated or re-implemented across these projects. This led to code redundancy and potential inconsistencies in how the folding physics were applied.
* **Decision:**
  We decided to extract the core geometric logic into a shared library crate, `crates/origami`.
  This crate encapsulates:
  - **`MiuraParams`**: Parameters defining unit cell dimensions (a, b) and the folding angle (gamma).
  - **`MiuraOri`**: Struct responsible for generating the grid of vertex positions.
  - **`Orientation`**: Enum to handle different folding directions (X-axis vs Y-axis shift).
  - **`OrigamiMesh`**: Helper for generating render-ready vertex/index buffers with UV mapping.

  The crate relies on `macroquad` for vector types (`Vec3`, `Vec2`) to ensure compatibility with the primary rendering engine used in these experiments.

* **Consequences:**
  - **Positive:** Centralized logic means bug fixes or improvements to the folding algorithm (e.g., adding new tessellation patterns) benefit all dependent experiments immediately. Testing geometric correctness is decoupled from the rendering loop.
  - **Negative:** Introduces a dependency on `macroquad` within a logic crate, which might limit its use in non-macroquad contexts (though `macroquad` is the standard for this repo).
