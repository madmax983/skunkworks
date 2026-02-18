# 041. Extract Gray-Scott Simulation Logic

* **Status:** Accepted
* **Context:**
  Experiments like `code-phage` and `myco-diffusion` rely on Reaction-Diffusion simulations, specifically the Gray-Scott model. These simulations often involve grid updates that are computationally intensive. Implementations were scattered across projects, sometimes with subtle variations in laplacian convolution kernels or boundary handling. Additionally, there was a need for optional parallelism (using `rayon`) for larger grid sizes.
* **Decision:**
  We decided to create a dedicated crate, `crates/gray-scott`, to house the core simulation logic.
  This crate provides:
  - **`GrayScott`**: The main struct holding double-buffered grids (`u`, `v`, `next_u`, `next_v`) and managing the update loop.
  - **Laplacian Kernel**: A standardized 3x3 convolution kernel (weights: center=-1, orth=0.2, diag=0.05) ensuring isotropic diffusion.
  - **Parallelism**: An optional `feature = "parallel"` that uses `rayon` to parallelize the update loop across rows.
  - **Initialization**: Default parameters tuned for stable pattern formation ("Spots").

* **Consequences:**
  - **Positive:** Consistent simulation behavior across experiments. Easy to optimize the core update loop (e.g., using SIMD or compute shaders in the future) without modifying consumer code. Decouples physics from visualization, allowing for different rendering backends (TUI vs. WGPU vs. Macroquad).
  - **Negative:** Adds a dependency management overhead. Projects must explicitly opt-in to `parallel` via Cargo features.
